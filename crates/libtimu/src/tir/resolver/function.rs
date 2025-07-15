//! Function definition resolution for the TIR system.
//!
//! This module handles the resolution of function definitions, including parameter
//! type checking, return type validation, method resolution within classes, and
//! proper handling of the special `this` parameter for object-oriented features.
//! Function resolution is one of the most complex parts of the TIR system due to
//! the need to handle various contexts and language features.
//!
//! # Function Resolution Process
//!
//! Function resolution operates in the standard two-phase approach:
//!
//! ## Phase 1: Signature Resolution
//! 1. **Parameter processing**: Resolve all parameter types and validate syntax
//! 2. **Return type resolution**: Ensure return type exists and is valid
//! 3. **Scope creation**: Create function scope for local variables
//! 4. **Context validation**: Verify function is in correct context (class vs module)
//! 5. **Signature registration**: Register function signature in type system
//!
//! ## Phase 2: Body Resolution
//! 1. **Statement processing**: Resolve all statements in function body
//! 2. **Variable resolution**: Ensure all variable references are valid
//! 3. **Type checking**: Validate all expressions and assignments
//! 4. **Return validation**: Ensure function returns match declared type
//!
//! # Function Contexts
//!
//! Functions can be defined in different contexts with varying rules:
//!
//! ## Module-Level Functions
//! ```timu
//! func moduleFunction(param: Type): ReturnType {
//!     // Function body
//! }
//! 
//! pub func publicFunction(): void {
//!     // Public function accessible from other modules
//! }
//! ```
//!
//! ## Class Methods
//! ```timu
//! class MyClass {
//!     func method(this): void {
//!         // Method with access to class instance
//!     }
//!     
//!     func methodWithParams(this, param: Type): ReturnType {
//!         // Method with additional parameters
//!     }
//! }
//! ```
//!
//! ## Extension Methods
//! ```timu
//! extend ExistingType : Interface {
//!     func extensionMethod(this): void {
//!         // Method added to existing type
//!     }
//! }
//! ```
//!
//! # The `this` Parameter
//!
//! The `this` parameter is special and has specific rules:
//! - **Must be first parameter** if present
//! - **Only valid in class/extension methods** 
//! - **Automatically typed** to the containing class
//! - **Provides instance access** within method body
//!
//! # Type Resolution Features
//!
//! ## Parameter Type Resolution
//! - Supports qualified type names (`module.Type`)
//! - Handles nullable types (`?Type`)
//! - Processes reference types (`ref Type`)
//! - Validates parameter name uniqueness
//!
//! ## Return Type Processing
//! - Resolves return type before function body
//! - Supports all type modifiers (nullable, reference)
//! - Enables forward references to types defined later
//!
//! ## Scope Management
//! - Creates dedicated scope for each function
//! - Manages parameter visibility within function body
//! - Handles nested scope resolution for local variables
//!
//! # Error Handling
//!
//! Comprehensive error reporting for:
//! - Invalid `this` parameter usage
//! - Missing or invalid parameter types
//! - Duplicate parameter names
//! - Missing return types
//! - Context mismatches (e.g., `this` in module function)

use std::{borrow::Cow, ops::Range};

use libtimu_macros::TimuError;
use libtimu_macros_core::SourceCode;
use strum_macros::{EnumDiscriminants, EnumProperty};

use crate::{
    ast::{FunctionArgumentAst, FunctionDefinitionAst, FunctionDefinitionLocationAst}, nom_tools::{Span, SpanInfo, ToRange}, tir::{context::TirContext, module::ModuleRef, object_signature::{GetItem, TypeValue, TypeValueDiscriminants}, resolver::get_object_location_or_resolve, scope::{ScopeLocation, TypeVariableInformation, VariableInformation}, signature::{SignatureInfo, SignaturePath}, TirError, TypeSignature}
};

use super::{build_type_name, try_resolve_signature, BuildFullNameLocater, ResolveAst, ResolverError, TypeLocation};

/// Represents a resolved function parameter with complete type information
/// 
/// This structure contains all the information needed for a function parameter
/// after type resolution has been completed. It includes both the parameter's
/// metadata and its resolved type location in the type system.
/// 
/// # Fields
/// 
/// - `name`: The parameter name as it appears in the source code
/// - `field_type`: The resolved type location in the TIR type system
/// - `field_type_span`: Source location of the type annotation
/// - `is_reference`: Whether this parameter uses reference semantics
/// - `is_nullable`: Whether this parameter can accept null values
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct FunctionArgument<'base> {
    pub name: Span<'base>,
    pub field_type: TypeLocation,
    pub field_type_span: Span<'base>,
    pub is_reference: bool,
    pub is_nullable: bool,
}

/// Complete function definition with resolved types and metadata
/// 
/// This structure represents a fully resolved function definition including
/// its signature, parameters, return type, and context information. It serves
/// as the canonical representation of a function in the TIR system.
/// 
/// # Fields
/// 
/// - `is_public`: Whether the function is publicly accessible from other modules
/// - `name`: The function name as it appears in source code  
/// - `arguments`: All function parameters with resolved types
/// - `return_type`: The resolved return type location
/// - `signature_path`: Qualified path for the function in the type system
/// - `ast`: Original AST node for accessing source information
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct FunctionDefinition<'base> {
    pub is_public: bool,
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgument<'base>>,
    pub return_type: TypeLocation,
    pub signature_path: SignaturePath<'base>,
    pub ast: FunctionDefinitionAst<'base>
}

impl GetItem for FunctionDefinition<'_> {
    fn get_item_location(&self, _: &TirContext<'_>, path: &str) -> Option<TypeLocation> {
        self
            .arguments
            .iter()
            .find(|argument| argument.name.text == path)
            .map(|argument| argument.field_type)
    }
}

pub fn unwrap_for_this<'base>(parent: &Option<TypeLocation>, this: &Span<'base>) -> Result<TypeLocation, TirError> {
    match parent {
        Some(parent) => Ok(*parent),
        None => Err(FunctionResolveError::this_need_to_define_in_class(this.into())),
    }
}

// Search class type location
/// Finds the containing class type location by traversing the scope hierarchy
/// 
/// This function searches up the scope hierarchy to find the nearest enclosing
/// class scope. It's used primarily for resolving the `this` parameter type
/// in class methods and ensuring that `this` is only used in appropriate contexts.
/// 
/// # Scope Traversal Strategy
/// 
/// The function traverses parent scopes looking for:
/// 1. **Resolved class types**: Classes that have completed type resolution
/// 2. **Reserved class types**: Classes that are still being resolved
/// 3. **Scope boundaries**: Points where no further traversal is possible
/// 
/// # Arguments
/// * `context` - The TIR context containing scope and type information
/// * `scope_location` - The starting scope for the search
/// 
/// # Returns
/// * `Some(TypeLocation)` - The type location of the containing class
/// * `None` - No containing class found (function is at module level)
/// 
/// # Usage
/// 
/// This function is called when:
/// - Processing `this` parameters in method definitions
/// - Validating that `this` is only used in class/extension contexts
/// - Determining the correct type for the `this` parameter
/// 
/// # Scope Resolution
/// 
/// The function handles both completed and in-progress type resolution:
/// - **Completed types**: Uses the final type information from `context.types`
/// - **Reserved types**: Uses type shadows from `context.types.get_reserve_from_location()`
pub fn find_class_location<'base>(context: &TirContext<'base>, scope_location: ScopeLocation) -> Option<TypeLocation> {
    let mut scope_location = scope_location;

    loop {
        let scope =  context.get_scope(scope_location).unwrap();
        if scope.current_type == TypeLocation::UNDEFINED && scope.parent_scope.is_some() {
            scope_location = scope.parent_scope.unwrap();
            continue;
        }

        match context.types.get_from_location(scope.current_type).cloned().map(|signature| signature.value) {
            // This scope belong to class so return the type location
            Some(TypeValue::Class(_)) => return Some(scope.current_type),

            // We are still in some of the child scope, continue to search
            Some(_) => if let Some(parent_scope) = scope.parent_scope {
                scope_location = parent_scope;
                continue;
            },

            // There is no more scope to search, so, we can stop
            None => ()
        };

        // The type not saved yet, lets check the reservations
        match context.types.get_reserve_from_location(scope.current_type).map(|reservation| reservation.type_shadow) {
            // This scope belong to class so return the type location
            Some(TypeValueDiscriminants::Class) => return Some(scope.current_type),

            // We are still in some of the child scope, continue to search
            Some(_) => match scope.parent_scope {

                // There is parent scope, lets continue to search
                Some(parent_scope) => scope_location = parent_scope,

                // End of the search, not found, return None
                None => return None
            },

            // There is no more scope to search, so, we can stop and return the error
            None => return None
        };
    }
}

impl<'base> ResolveAst<'base> for FunctionDefinitionAst<'base> {    
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        let full_name = self.build_full_name(context, BuildFullNameLocater::Scope(scope_location), None);
        simplelog::debug!("Resolving function: <u><b>{}</b></u>", full_name.as_str());

        let (module_ref, parent_type, parent_scope,) = {
            let scope = context.get_scope(scope_location).expect("Scope not found, it is a bug");
            (scope.module_ref.clone(), scope.parent_type, scope.parent_scope)
        };
        let (signature_path, signature_location) = context.reserve_object_location(self.name(), TypeValueDiscriminants::Function, SignaturePath::owned(full_name), &module_ref, self.name.to_range(), self.name.state.file.clone())?;
        
        let definition = self.build_definition(context, scope_location, parent_scope, &module_ref, parent_type, signature_path.clone())?;
                
        /* Add function information as a variable */
        let parent_scope = context.get_mut_scope(parent_scope.expect("Parent scope not found, it is a bug")).expect("Scope not found, it is a bug");
        
        /*
        It parent has not a parent, it means function is not in the Class and class resolve operation will add function to variable list.
        It could be better to have function type like, ClassFunction and PureFunction etc. etc.
        */
        if parent_scope.parent_scope.is_none() {
            parent_scope.add_variable(TypeVariableInformation::new(self.name.clone(), signature_location, false, true, true))?;
        }
        
        let signature = TypeSignature::new(
            TypeValue::Function (definition.into()),
            self.name.state.file.clone(),
            self.name.to_range(),
            parent_type,
        );
        
        context.publish_object_location(signature_path, signature);
        Ok(signature_location)
    }
    
    fn finish(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<(), TirError> {        
        /* Parse body */
        for statement in self.body.statements.iter() {
            statement.resolve(context, scope_location)?;
            statement.finish(context, scope_location)?;
        }

        Ok(())
     }
    
    fn name(&self) -> Cow<'base, str> {
        match self.location.as_ref() {
            FunctionDefinitionLocationAst::Module => (*self.name.text).into(),
            FunctionDefinitionLocationAst::Class(class) => format!("{}::{}", class.text, self.name.text).into(),
        }
    }

    fn build_full_name<'a>(&self, context: &TirContext<'_>, locater: BuildFullNameLocater<'a, 'base>, _: Option<TypeLocation>) -> String {
        let module = match locater {
            BuildFullNameLocater::Scope(scope_location) => {
                let module_ref = context.get_scope(scope_location).expect("Scope not found").module_ref.clone();
                module_ref.upgrade(context).unwrap()
            },
            BuildFullNameLocater::Module(module_ref) => module_ref.upgrade(context).unwrap(),
        };
        
        match self.location.as_ref() {
            FunctionDefinitionLocationAst::Module => format!("{}.{}", module.path, self.name.text),
            FunctionDefinitionLocationAst::Class(class) => format!("{}.{}::{}", module.path, class.text, self.name.text),
        }
    }
}

impl<'base> FunctionDefinitionAst<'base> {
    fn build_definition(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation, parent_scope: Option<ScopeLocation>, module: &ModuleRef<'base>, parent_type: Option<TypeLocation>, signature_path: SignaturePath<'base>) -> Result<FunctionDefinition<'base>, TirError> {
        let mut arguments = vec![];
        let return_type = get_object_location_or_resolve(context, &self.return_type, module, scope_location)?;

        /* Parse arguments */
        for (index, argument) in self.arguments.iter().enumerate() {
            let (argument_name, range, file) = match argument {
                FunctionArgumentAst::This(this) => {
                    let class_type_location = match find_class_location(context, scope_location) {
                        Some(location) => location,
                        None => return Err(FunctionResolveError::this_need_to_define_in_class(this.into()))
                    };

                    let parent_scope = context.get_mut_scope(parent_scope.unwrap()).unwrap();
                    parent_scope.add_variable(VariableInformation::basic(this.clone(), class_type_location))?;

                    if index != 0 {
                        return Err(FunctionResolveError::this_need_to_define_in_class(this.into()));
                    }
                    
                    match context.types.get_signature_from_location(unwrap_for_this(&Some(class_type_location), this)?).unwrap() {
                        SignatureInfo::Reserved(reservation) => {
                            let reservation = reservation.clone();
                            (reservation.name, reservation.position, reservation.file)
                        },
                        SignatureInfo::Value(value) => {
                            (Cow::Owned(value.value.get_name().to_string()), this.to_range(), this.state.file.clone())
                        }
                    }
                },
                FunctionArgumentAst::Argument { name, field_type } => {
                    let field_type = get_object_location_or_resolve(context, field_type, module, scope_location)?;
                    let scope = context.get_mut_scope(scope_location).unwrap();

                    scope.add_variable(VariableInformation::basic(name.clone(), field_type))?;
                    (Cow::Borrowed(name.text), name.to_range(), name.state.file.clone())
                }
            };
            
            let (field_type_span, type_name) = match argument {
                FunctionArgumentAst::This(this) => {
                    (this.clone(), match context.types.get_signature_from_location(unwrap_for_this(&parent_type, this)?).unwrap() {
                        SignatureInfo::Reserved(reservation) => reservation.name.clone(),
                        SignatureInfo::Value(value) => Cow::Owned(value.value.get_name().to_string())
                    })
                },
                FunctionArgumentAst::Argument { field_type, .. } => (field_type.names_span.clone(), Cow::Owned(build_type_name(field_type))),
            };

            let field_type = match try_resolve_signature(context, module, scope_location, type_name.as_ref())? {
                Some(field_type) => field_type,
                None => return Err(TirError::type_not_found(context, type_name.to_string(), range, file))
            };

            if let Some(old) = arguments.iter().find(|item: &&FunctionArgument| *item.name.text == argument_name) {
                return Err(TirError::already_defined(old.name.to_range(), range.clone(), file));
            }

            arguments.push(FunctionArgument {
                name: match argument {
                    FunctionArgumentAst::This(this) => this.clone(),
                    FunctionArgumentAst::Argument { name, .. } => name.clone()
                },
                field_type,
                field_type_span,
                is_nullable: match argument {
                    FunctionArgumentAst::This(_) => false,
                    FunctionArgumentAst::Argument { field_type, .. } => field_type.nullable,
                },
                is_reference: match argument {
                    FunctionArgumentAst::This(_) => false,
                    FunctionArgumentAst::Argument { field_type, .. } => field_type.reference,
                }
            });
        }

        Ok(FunctionDefinition {
            is_public: self.is_public.is_some(),
            name: self.name.clone(),
            arguments,
            return_type,
            signature_path,
            ast: self.clone()
        })
    }

}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("This argument need to be defined in class function")]
pub struct ThisNeedToDefineInClass {
    #[label("`this` defined out of the class function")]
    pub position: Range<usize>,

    #[source_code]
    pub code: SourceCode,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Variable not found")]
pub struct VariableNotFound {
    #[label("Maybe not defined")]
    pub position: Range<usize>,

    #[source_code]
    pub code: SourceCode,
}

#[derive(Clone, Debug, TimuError, thiserror::Error, EnumDiscriminants, EnumProperty)]
pub enum FunctionResolveError {
    #[error("`this` needs to be first argument in function definition")]
    ThisArgumentMustBeFirst(SpanInfo),

    #[error(transparent)]
    #[diagnostic(transparent)]
    ThisNeedToDefineInClass(Box<ThisNeedToDefineInClass>),

    #[error(transparent)]
    #[diagnostic(transparent)]
    VariableNotFound(Box<VariableNotFound>),
}

impl From<FunctionResolveError> for TirError {
    fn from(value: FunctionResolveError) -> Self {
        ResolverError::FunctionResolve(Box::new(value)).into()
    }
}

impl FunctionResolveError {
    pub fn this_need_to_define_in_class(span: SpanInfo) -> TirError {
        FunctionResolveError::ThisNeedToDefineInClass(ThisNeedToDefineInClass {
            position: span.position.clone(),
            code: span.file.clone().into(),
        }.into()).into()
    }
    pub fn variable_not_found(span: SpanInfo) -> TirError {
        FunctionResolveError::VariableNotFound(VariableNotFound {
            position: span.position.clone(),
            code: span.file.clone().into(),
        }.into()).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{file::SourceFile, nom_tools::State, process_ast, process_code, tir::TirError};

    #[test]
    fn missing_type_1() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "func test(): a {} ".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn duplicated_function_argument() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class a {} func test(a: a, a: a): a {} ".to_string()));
        let ast: crate::ast::FileAst<'_> = process_code(&state)?;
        let error = crate::tir::build(vec![ast.into()]).unwrap_err();

        if let TirError::AlreadyDefined(inner_error) = error
        {
            assert_eq!(inner_error.new_position, (27..28));
        } else {
            panic!("Expected TirError::AlreadyDefined but got {error:?}");
        }
        
        Ok(())
    }

    #[test]
    fn valid_types() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["lib".into()], " pub class testclass1 {} ".to_string()));
        let state_2 = State::new(SourceFile::new(vec!["main".into()],
            r#"use lib.testclass1 as test;
    func main(a: test): test {}"#.to_string()));
        let source_1 = process_code(&state_1)?;
        let source_2 = process_code(&state_2)?;


        let context = process_ast(vec![source_2.into(), source_1.into()])?;
        assert_eq!(context.modules.len(), 2);

        let main_module = context.modules.iter().find(|(name, _)| *name == "main").unwrap();
        let lib_module = context.modules.iter().find(|(name, _)| *name == "lib").unwrap();

        main_module.1.types.get("main").unwrap();

        assert!(main_module.1.ast_imported_modules.get("testclass1").is_none());
        assert!(main_module.1.ast_imported_modules.get("test").is_some());
        assert!(main_module.1.types.get("testclass1").is_none());

        lib_module.1.types.get("testclass1").unwrap();

        Ok(())
    }

    #[test]
    fn missing_type_2() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "func test(a: a): test {}".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn not_in_class() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "func test(this): test {}".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }
}
