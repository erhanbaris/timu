use std::{borrow::Cow, rc::Rc};

use indexmap::IndexMap;

use crate::{
    ast::{ClassDefinitionAst, ClassDefinitionFieldAst, TypeNameAst}, nom_tools::{Span, ToRange}, tir::{context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, resolver::build_object_type, ObjectSignature, TirError}
};

use super::{ResolveSignature, SignatureLocation};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassArgument<'base> {
    pub name: Span<'base>,
    pub field_type: Rc<ObjectSignature<'base>>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassDefinition<'base> {
    pub name: Span<'base>,
    pub fields: IndexMap<Cow<'base, str>, SignatureLocation>,
    pub extends:Vec<TypeNameAst<'base>>,
}

impl<'base> ResolveSignature<'base> for ClassDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>) -> Result<SignatureLocation, TirError<'base>> {
        simplelog::debug!("Resolving class: <u><b>{}</b></u>", self.name.fragment());
        let tmp_module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
        tmp_module.object_signatures.reserve(Cow::Borrowed(self.name.fragment()))
            .map_err(|_| TirError::already_defined(self.name.to_range(), self.name.extra.file.clone()))?;

        let mut fields = IndexMap::<Cow<'_, str>, SignatureLocation>::default();

        for field in self.fields.iter() {
            match field {
                ClassDefinitionFieldAst::Field(field) => {
                    let field_type = build_object_type(context, &field.field_type, module)?;
                    fields.insert((*field.name.fragment()).into(), field_type)
                        .map_or(Ok(()), |_| Err(TirError::already_defined(field.name.to_range(), field.name.extra.file.clone())))?;
                }
                ClassDefinitionFieldAst::Function(function) => {
                    let signature = function.resolve(context, module)?;
                    fields.insert((*function.name.fragment()).into(), signature)
                        .map_or(Ok(()), |_| Err(TirError::already_defined(function.name.to_range(), function.name.extra.file.clone())))?;
                }
            };
        }
        
        let signature = Rc::new(ObjectSignature::new(ObjectSignatureValue::Class(ClassDefinition {
            name: self.name.clone(),
            fields,
            extends: Default::default()
        }), self.name.extra.file.clone(), self.name.to_range()));

        let module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
        Ok(module.object_signatures.update(Cow::Borrowed(self.name.fragment()), signature.clone()))
    }
    
    fn name(&self) -> Cow<'base, str> {
        Cow::Borrowed(*self.name.fragment())
    }
}

#[cfg(test)]
mod tests {
    use crate::process_code;

    #[test]
    fn missing_type() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "class test { func test(a: a): a {} }")?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn recursive_type() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "class test { a: test; func test(a: test): test {} }")?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }
}
