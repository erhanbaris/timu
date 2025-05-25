use std::borrow::Cow;

use crate::{
    ast::{FunctionArgumentAst, FunctionDefinitionAst, FunctionDefinitionLocationAst},
    nom_tools::{Span, ToRange},
    tir::{context::TirContext, module::ModuleRef, object_signature::TypeValue, resolver::get_object_location_or_resolve, signature::{SignatureInfo, SignaturePath}, TypeSignature, TirError},
};

use super::{build_type_name, try_resolve_signature, ResolveAst, TypeLocation};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct FunctionArgument<'base> {
    pub name: Span<'base>,
    pub field_type: TypeLocation,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct FunctionDefinition<'base> {
    pub is_public: bool,
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgument<'base>>,
    pub return_type: TypeLocation,
    pub body: Vec<TypeLocation>,
    pub signature_path: SignaturePath<'base>,
}

pub fn unwrap_for_this<'base>(parent: &Option<TypeLocation>, this: &Span<'base>) -> Result<TypeLocation, TirError<'base>> {
    match parent {
        Some(parent) => Ok(parent.clone()),
        None => Err(TirError::ThisNeedToDefineInClass { position: this.to_range(), source: this.extra.file.clone() }),
    }
}

impl<'base> ResolveAst<'base> for FunctionDefinitionAst<'base> {
    type Result = TypeLocation;
    
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, parent: Option<TypeLocation>) -> Result<TypeLocation, TirError<'base>> {
        simplelog::debug!("Resolving function: <u><b>{}</b></u>", self.name.fragment());
        let full_name = match &self.location {
            FunctionDefinitionLocationAst::Module => Cow::Borrowed(*self.name.fragment()),
            FunctionDefinitionLocationAst::Class(class) => Cow::Owned(format!("{}.{}", class.fragment(), self.name.fragment())),
        };
        
        let (signature_path, signature_location) = context.reserve_object_location(full_name.clone(), module, self.name.to_range(), self.name.extra.file.clone())?;

        let definition = self.build_definition(context, module, parent.clone(), signature_path.clone())?;

        let signature = TypeSignature::new(
            TypeValue::Function (definition),
            self.name.extra.file.clone(),
            self.name.to_range(),
            parent,
        );
        
        context.publish_object_location(signature_path, signature);
        Ok(signature_location)
    }
    
    fn finish(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, function_location: TypeLocation) -> Result<(), TirError<'base>> {

        /* Parse body */
        for statement in self.body.statements.iter() {
            let location = statement.resolve(context, module, Some(function_location.clone()))?;
            statement.finish(context, module, location)?;
        }

        Ok(())
     }
    
    fn name(&self) -> Cow<'base, str> {
        Cow::Borrowed(*self.name.fragment())
    }
}

impl<'base> FunctionDefinitionAst<'base> {
    fn build_definition(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, parent: Option<TypeLocation>, signature_path: SignaturePath<'base>) -> Result<FunctionDefinition<'base>, TirError<'base>> {
        let mut arguments = vec![];
        let return_type = get_object_location_or_resolve(context, &self.return_type, module)?;

        /* Parse arguments */
        for (index, argument) in self.arguments.iter().enumerate() {
            let (argument_name, range, file) = match argument {
                FunctionArgumentAst::This(this) => {

                    if index != 0 {
                        return Err(TirError::this_argument_must_be_first(this.to_range(), this.extra.file.clone()));
                    }
                    
                    match context.types.get_signature_from_location(unwrap_for_this(&parent, this)?).unwrap() {
                        SignatureInfo::Reserved(reservation) => {
                            let reservation = reservation.clone();
                            (reservation.name, reservation.position, reservation.file)
                        },
                        SignatureInfo::Value(value) => {
                            (Cow::Owned(value.value.get_name().to_string()), this.to_range(), this.extra.file.clone())
                        }
                    }
                },
                FunctionArgumentAst::Argument { name, .. } => (Cow::Borrowed(*name.fragment()), name.to_range(), name.extra.file.clone())
            };
            
            let type_name = match argument {
                FunctionArgumentAst::This(this) => {
                    match context.types.get_signature_from_location(unwrap_for_this(&parent, this)?).unwrap() {
                        SignatureInfo::Reserved(reservation) => reservation.name.clone(),
                        SignatureInfo::Value(value) => Cow::Owned(value.value.get_name().to_string())
                    }
                },
                FunctionArgumentAst::Argument { field_type, .. } => Cow::Owned(build_type_name(field_type)),
            };

            let field_type = match try_resolve_signature(context, module, type_name.as_ref())? {
                Some(field_type) => field_type,
                None => return Err(TirError::type_not_found(range, file))
            };

            if arguments.iter().any(|item: &FunctionArgument| *item.name.fragment() == argument_name) {
                return Err(TirError::already_defined(range, file));
            }

            arguments.push(FunctionArgument {
                name: match argument {
                    FunctionArgumentAst::This(this) => this.clone(),
                    FunctionArgumentAst::Argument { name, .. } => name.clone()
                },
                field_type,
            });
        }

        Ok(FunctionDefinition {
            is_public: self.is_public.is_some(),
            name: self.name.clone(),
            arguments,
            return_type,
            signature_path,
            body: Vec::new(),
        })
    }

}

#[cfg(test)]
mod tests {
    use crate::{process_ast, process_code, tir::TirError};

    #[test]
    fn missing_type_1() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "func test(): a {} ")?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn dublicated_function_argument() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "class a {} func test(a: a, a: a): a {} ")?;
        let error = crate::tir::build(vec![ast.into()]).unwrap_err();

        if let TirError::AlreadyDefined {
            position,
            source,
        } = error
        {
            assert_eq!(position, 27..28);
            assert_eq!(source.path().join("/"), "source");
        } else {
            panic!("Expected TirError::AlreadyDefined but got {:?}", error);
        }
        Ok(())
    }

    #[test]
    fn valid_types() -> Result<(), ()> {
        
        let source_1 = process_code(vec!["lib".into()], " class testclass1 {} ")?;
        let source_2 = process_code(vec!["main".into()],
            r#"use lib.testclass1 as test;
    func main(a: test): test {}"#,
        )?;

        let context = process_ast(vec![source_2.into(), source_1.into()])?;
        assert_eq!(context.modules.len(), 2);

        let main_module = context.modules.iter().find(|(name, _)| *name == "main").unwrap();
        let lib_module = context.modules.iter().find(|(name, _)| *name == "lib").unwrap();

        main_module.1.object_signatures.get("main").unwrap();

        assert!(main_module.1.ast_imported_modules.get("testclass1").is_none());
        assert!(main_module.1.ast_imported_modules.get("test").is_some());
        assert!(main_module.1.object_signatures.get("testclass1").is_none());

        lib_module.1.object_signatures.get("testclass1").unwrap();

        Ok(())
    }

    #[test]
    fn missing_type_2() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "func test(a: a): test {}")?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn not_in_class() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "func test(this): test {}")?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }
}
