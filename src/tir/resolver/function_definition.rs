use std::{borrow::Cow, rc::Rc};

use crate::{
    ast::{FunctionDefinitionAst, FunctionDefinitionLocationAst},
    nom_tools::{Span, ToRange},
    tir::{context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, resolver::build_object_type, ObjectSignature, TirError},
};

use super::{build_type_name, try_resolve_signature, ResolveSignature, SignatureLocation};

#[derive(Debug)]
#[allow(dead_code)]
pub struct FunctionArgument<'base> {
    pub name: Span<'base>,
    pub field_type: SignatureLocation,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct FunctionDefinition<'base> {
    pub is_public: bool,
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgument<'base>>,
    pub return_type: SignatureLocation,
    // pub body: BodyAst<'base>,
}

impl<'base> ResolveSignature<'base> for FunctionDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>) -> Result<SignatureLocation, TirError<'base>> {
        simplelog::debug!("Resolving function: <u><b>{}</b></u>", self.name.fragment());
        let full_name = match &self.location {
            FunctionDefinitionLocationAst::Module => Cow::Borrowed(*self.name.fragment()),
            FunctionDefinitionLocationAst::Class(class) => Cow::Owned(format!("{}.{}", class.fragment(), self.name.fragment())),
        };
        
        let tmp_module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
        tmp_module.object_signatures.reserve(full_name.clone())
            .map_err(|_| TirError::already_defined(self.name.to_range(), self.name.extra.file.clone()))?;

        let mut arguments = vec![];
        let return_type = build_object_type(context, &self.return_type, module)?;

        for argument in self.arguments.iter() {
            let type_name = build_type_name(&argument.field_type);
            let field_type = match try_resolve_signature(context, module, type_name.as_str())? {
                Some(field_type) => field_type,
                None => {
                    return Err(TirError::TypeNotFound {
                        source: argument.field_type.names.last().unwrap().extra.file.clone(),
                        position: argument.field_type.to_range(),
                    });
                }
            };

            if arguments.iter().any(|item: &FunctionArgument| item.name.fragment() == argument.name.fragment()) {
                return Err(TirError::already_defined(argument.name.to_range(), argument.name.extra.file.clone()));
            }

            arguments.push(FunctionArgument {
                name: argument.name.clone(),
                field_type,
            });
        }

        let signature = Rc::new(ObjectSignature::new(
            ObjectSignatureValue::Function(
                FunctionDefinition {
                    is_public: self.is_public.is_some(),
                    name: self.name.clone(),
                    arguments,
                    return_type,
                },
            ),
            self.name.extra.file.clone(),
            self.name.to_range(),
        ));
        
        let module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
        Ok(module.object_signatures.update(full_name, signature.clone()))
    }
    
    fn name(&self) -> Cow<'base, str> {
        Cow::Borrowed(*self.name.fragment())
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

        assert!(main_module.1.imported_modules.get("testclass1").is_none());
        assert!(main_module.1.imported_modules.get("test").is_some());
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
}
