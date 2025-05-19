use std::{borrow::Cow, rc::Rc};

use indexmap::IndexMap;

use crate::{
    ast::{ExtendDefinitionAst, ExtendDefinitionFieldAst},
    nom_tools::{Span, ToRange},
    tir::{context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, resolver::build_object_type, ObjectSignature, TirError},
};

use super::{build_type_name, ResolveSignature, SignatureLocation};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ExtendArgument<'base> {
    pub name: Span<'base>,
    pub field_type: Rc<ObjectSignature<'base>>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ExtendDefinition<'base> {
    pub name: Span<'base>,
    pub fields: IndexMap<Cow<'base, str>, SignatureLocation>,
}

impl<'base> ResolveSignature<'base> for ExtendDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>) -> Result<SignatureLocation, TirError<'base>> {
        simplelog::debug!("Resolving extend: <u><b>{}</b></u>", self.name.names.first().unwrap().fragment());
        
        let class_signature = build_object_type(context, &self.name, module)?;
        let tmp_module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
        
        let class_binding = tmp_module.object_signatures.get_from_location(class_signature);
        let class = match &class_binding {
            Some(signature) => match signature.value.as_ref() {
                ObjectSignatureValue::Class(class) => class,
                _ => return Err(TirError::invalid_type(self.name.to_range(), self.name.names.first().unwrap().extra.file.clone())),
            },
            None => return Err(TirError::type_not_found(self.name.to_range(), self.name.names.first().unwrap().extra.file.clone())),
        };
        
        let mut fields = IndexMap::<Cow<'_, str>, SignatureLocation>::default();

        for field in self.fields.iter() {
            match field {
                ExtendDefinitionFieldAst::Function(function) => {
                    let signature = function.resolve(context, module)?;
                    fields.insert((*function.name.fragment()).into(), signature)
                        .map_or(Ok(()), |_| Err(TirError::already_defined(function.name.to_range(), function.name.extra.file.clone())))?;
                }
                ExtendDefinitionFieldAst::Field(field) => {
                    if field.is_public.is_some() {
                        return Err(TirError::extra_accessibility_identifier(field.is_public.as_ref().unwrap().to_range(), field.name.extra.file.clone()));
                    }

                    let field_type = build_object_type(context, &field.field_type, module)?;
                    fields.insert((*field.name.fragment()).into(), field_type)
                        .map_or(Ok(()), |_| Err(TirError::already_defined(field.name.to_range(), field.name.extra.file.clone())))?;
                }
            };
        }

        for (key, _value) in fields.into_iter() {
            if class.fields.contains_key(&key) {
                return Err(TirError::already_defined(self.name.to_range(), self.name.names.first().unwrap().extra.file.clone()));
            }

            // class.fields.insert(key, value);
        }
        
        Ok(SignatureLocation(usize::MAX))
    }
    
    fn name(&self) -> Cow<'base, str> {
        let name = self.name.names.first().unwrap().fragment();
        let interfaces = self.base_interfaces
            .iter()
            .map(|item| build_type_name(item))
            .collect::<Vec<_>>()
            .join("-");

        format!("{}-{}", name, interfaces).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::process_code;

    #[test]
    fn empty_interface() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface ITest {}
extend TestClass: ITest {}
class TestClass {}
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn dublicate_field_1() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface ITest { a: TestClass; }
extend TestClass: ITest { a: TestClass; }
class TestClass { a: TestClass; }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn dublicate_field_2() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface ITest { func test(): TestClass; }
extend TestClass: ITest { func test(): TestClass { } }
class TestClass { func test(): TestClass { } }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }
}

