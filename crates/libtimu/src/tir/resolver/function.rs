use std::{borrow::Cow, ops::Range};

use libtimu_macros::TimuError;
use libtimu_macros_core::SourceCode;
use strum_macros::{EnumDiscriminants, EnumProperty};

use crate::{
    ast::{FunctionArgumentAst, FunctionDefinitionAst, FunctionDefinitionLocationAst}, nom_tools::{Span, SpanInfo, ToRange}, tir::{context::TirContext, module::ModuleRef, object_signature::{GetItem, TypeValue}, resolver::get_object_location_or_resolve, scope::ScopeLocation, signature::{SignatureInfo, SignaturePath}, TirError, TypeSignature}
};

use super::{build_type_name, try_resolve_signature, BuildFullNameLocater, ResolveAst, ResolverError, TypeLocation};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct FunctionArgument<'base> {
    pub name: Span<'base>,
    pub field_type: TypeLocation,
    pub field_type_span: Span<'base>,
}

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
        None => Err(FunctionResolveError::this_need_to_define_in_class(this.into()).into()),
    }
}

// Search class type location
pub fn find_class_location<'base>(context: &TirContext<'base>, scope_location: ScopeLocation) -> Option<TypeLocation> {
    let mut scope_location = scope_location;

    loop {
        let scope =  context.get_scope(scope_location).unwrap();
        match context.types.get_from_location(scope.current_type).cloned().map(|signature| signature.value) {
            // This scope belong to class so return the type location
            Some(TypeValue::Class(_)) => return Some(scope.current_type),

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
        let (signature_path, signature_location) = context.reserve_object_location(self.name(), SignaturePath::owned(full_name), &module_ref, self.name.to_range(), self.name.state.file.clone())?;

        let definition = self.build_definition(context, scope_location, parent_scope, &module_ref, parent_type, signature_path.clone())?;

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
                    let parent_type = match parent_type {
                        Some(parent_type) => parent_type,
                        None => return Err(FunctionResolveError::this_need_to_define_in_class(this.into()).into())
                    };

                    let parent_scope = context.get_mut_scope(parent_scope.unwrap()).unwrap();
                    parent_scope.add_variable(this.clone(), parent_type)?;

                    if index != 0 {
                        return Err(FunctionResolveError::this_need_to_define_in_class(this.into()).into());
                    }
                    
                    match context.types.get_signature_from_location(unwrap_for_this(&Some(parent_type), this)?).unwrap() {
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

                    scope.add_variable(name.clone(), field_type)?;
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
                field_type_span
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

impl<'base> FunctionResolveError {
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
    fn dublicated_function_argument() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class a {} func test(a: a, a: a): a {} ".to_string()));
        let ast = process_code(&state)?;
        let _error = crate::tir::build(vec![ast.into()]).unwrap_err();

        /*
        todo: fix this test
        if let TirError::AlreadyDefined(inner_error) = error
        {
            assert_eq!(inner_error.new_position, (27..28).into());
        } else {
            panic!("Expected TirError::AlreadyDefined but got {:?}", error);
        }
        */
        Ok(())
    }

    #[test]
    fn valid_types() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["lib".into()], " class testclass1 {} ".to_string()));
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
