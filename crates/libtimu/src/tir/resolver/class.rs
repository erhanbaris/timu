//! Class definition resolution for the Timu TIR system.
//!
//! This module handles the semantic analysis and type registration of class definitions
//! within the Timu Type Intermediate Representation (TIR). It processes class declarations,
//! resolves field types, manages method signatures, and integrates classes into the
//! type system for use by other compilation phases.
//!
//! # Class Resolution Process
//!
//! Class resolution follows a comprehensive multi-phase approach:
//!
//! ## Phase 1: Class Registration
//! 1. **Name resolution**: Build fully qualified class name
//! 2. **Type reservation**: Reserve a location in the type system
//! 3. **Scope creation**: Establish class scope for member resolution
//! 4. **Signature preparation**: Prepare class signature structure
//!
//! ## Phase 2: Member Resolution
//! 1. **Field processing**: Resolve field types and validate uniqueness
//! 2. **Method processing**: Resolve method signatures and create function scopes
//! 3. **Visibility handling**: Process public/private modifiers
//! 4. **Type validation**: Ensure all referenced types exist
//!
//! ## Phase 3: Finalization
//! 1. **Method completion**: Complete method body analysis in finish phase
//! 2. **Cross-reference validation**: Validate method calls and field access
//! 3. **Inheritance preparation**: Prepare for extension and interface implementation
//!
//! # Class Components
//!
//! ## Fields
//! - **Instance variables**: Data members that belong to each class instance
//! - **Type annotations**: All fields must have explicit type declarations
//! - **Visibility**: Support for public and private field access
//! - **Initialization**: Fields are initialized during object construction
//!
//! ## Methods
//! - **Instance methods**: Functions that operate on class instances
//! - **Constructor patterns**: Special handling for initialization methods
//! - **Parameter validation**: Type checking for method parameters
//! - **Return types**: All methods must specify return types
//! - **This parameter**: Support for explicit `this` parameter in methods
//!
//! # Type System Integration
//!
//! ## Class Registration
//! Classes are registered in the global type system with:
//! - **Fully qualified names**: Module-prefixed names for uniqueness
//! - **Type locations**: Unique identifiers for efficient lookup
//! - **Signature paths**: Hierarchical path information for resolution
//! - **Module references**: Association with defining module
//!
//! ## Field and Method Storage
//! - **Field map**: Hash map of field names to type information
//! - **Method integration**: Methods stored as fields with function types
//! - **Scope variables**: Class members added to appropriate scopes
//! - **Extension support**: Foundation for interface implementation
//!
//! # Object-Oriented Features
//!
//! ## Encapsulation
//! - **Access control**: Public/private visibility for fields and methods
//! - **Data hiding**: Private members inaccessible outside class
//! - **Interface contracts**: Support for implementing interfaces
//!
//! ## Future Extensibility
//! The architecture supports future object-oriented features:
//! - **Inheritance**: Single inheritance from base classes
//! - **Polymorphism**: Method overriding and virtual dispatch
//! - **Abstract classes**: Classes that cannot be instantiated
//! - **Generic classes**: Parameterized types for code reuse
//!
//! # Integration Points
//!
//! Class resolution integrates with:
//! - **Module system**: For qualified name resolution and imports
//! - **Interface system**: For contract implementation via extend declarations
//! - **Function system**: For method signature validation and calls
//! - **Scope system**: For member visibility and access control
//! - **Error system**: For comprehensive diagnostic reporting

use std::{borrow::Cow, collections::HashSet, rc::Rc};

use crate::{
    ast::{ClassDefinitionAst, ClassDefinitionFieldAst}, map::TimuHashMap, nom_tools::{Span, ToRange}, tir::{context::TirContext, object_signature::{GetItem, TypeValue, TypeValueDiscriminants}, resolver::{get_object_location_or_resolve, BuildFullNameLocater}, scope::{ScopeLocation, TypeVariableInformation, VariableInformation}, signature::SignaturePath, TirError, TypeSignature}
};

use super::{TypeLocation, ResolveAst};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassArgument<'base> {
    pub name: Span<'base>,
    pub field_type: Rc<TypeSignature<'base>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ClassDefinition<'base> {
    pub name: Span<'base>,
    pub fields: TimuHashMap<'base, Cow<'base, str>, TypeVariableInformation<'base>>,
    pub extends: HashSet<TypeLocation>,
}

impl PartialEq for ClassDefinition<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl GetItem for ClassDefinition<'_> {
    fn get_item_location(&self, _: &TirContext<'_>, path: &str) -> Option<TypeLocation> {
        self
            .fields
            .get(path)
            .map(|item| item.location)
    }
}

impl<'base> ResolveAst<'base> for ClassDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        simplelog::debug!("Resolving class: <u><b>{}</b></u>", self.name.text);

        let full_name = self.build_full_name(context, BuildFullNameLocater::Scope(scope_location), None);
        let module_ref = context.get_scope(scope_location).expect("Scope not found").module_ref.clone();

        let (signature_path, class_location) = context.reserve_object_location(self.name(), TypeValueDiscriminants::Class, SignaturePath::owned(full_name), &module_ref, self.name.to_range(), self.name.state.file.clone())?;
        let mut fields = TimuHashMap::<'base, Cow<'_, str>, TypeVariableInformation<'base>>::default();

        context.get_mut_scope(scope_location).expect("Scope not found, it is a bug").set_current_type(class_location);

        let mut function_signatures = Vec::new();

        for field in self.fields.iter() {
            match field {
                ClassDefinitionFieldAst::Field(field) => {
                    let field_type = get_object_location_or_resolve(context, &field.field_type, &module_ref, scope_location)?;

                    let variable = TypeVariableInformation::basic(field.name.clone(), field_type);
                    fields.validate_insert(Cow::Borrowed(field.name.text), variable)?;
                    context.get_mut_scope(scope_location).expect("Scope not found, it is a bug").add_variable(VariableInformation::basic(field.name.clone(), field_type))?;
                }
                ClassDefinitionFieldAst::Function(function) => {
                    let type_name = function.build_full_name(context, BuildFullNameLocater::Module(&module_ref), None);

                    let function_scope_location = context.create_child_scope(type_name.into(), scope_location, None);
                    let function_type_location = function.resolve(context, function_scope_location)?;
                    
                    // Set scope type information
                    context.get_mut_scope(function_scope_location).expect("Scope not found, it is a bug").set_current_type(function_type_location);

                    let variable = TypeVariableInformation::basic(function.name.clone(), function_type_location);
                    fields.validate_insert((*function.name.text).into(), variable)?;
                    context.get_mut_scope(scope_location).expect("Scope not found, it is a bug").add_variable(VariableInformation::basic(function.name.clone(), function_type_location))?;
                    function_signatures.push((function_type_location, function));
                }
            };
        }
        
        let class_signature = TypeSignature::new(TypeValue::Class(ClassDefinition {
            name: self.name.clone(),
            fields,
            extends: Default::default(),
        }), self.name.state.file.clone(), self.name.to_range(), None);

        context.publish_object_location(signature_path.clone(), class_signature);

        Ok(class_location)
    }
    
    fn finish(&self, context: &mut TirContext<'base>, scope: ScopeLocation) -> Result<(), TirError> {

        for field in self.fields.iter() {
            if let ClassDefinitionFieldAst::Function(function) = field {
                let module_ref = context.get_scope(scope).unwrap().module_ref.clone();
                let full_name = format!("{}.{}", module_ref.as_cow(), function.name());
                let search_scope = context.types_scope.get(full_name.as_str()).unwrap();               
                function.finish(context, *search_scope)?;
            }
        }
        
        Ok(())
    }
    
    fn name(&self) -> Cow<'base, str> {
        Cow::Borrowed(self.name.text)
    }
}

#[cfg(test)]
mod tests {
    use crate::{file::SourceFile, nom_tools::State, process_code, tir::TirError};

    #[test]
    fn missing_type() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class test { func test(a: a): a {} }".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn recursive_type() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class test { a: test; func test(a: test): test {} }".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn this_location_1() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class test { func test(this): test {} }".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn this_location_2() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class test { func test(this, a: test): test {} }".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn this_location_3() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class test { func test(a: test, this): test {} }".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn call_interface_function() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface ITest {
    func test(): string;
    a: TestClass;
}

extend TestClass: ITest {
    func test(): string {
        
    }
    a: TestClass;
}

class TestClass {
    func init(this): string {
        this.test();
        this.a.test();
    }
}
    "#.to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }
}
