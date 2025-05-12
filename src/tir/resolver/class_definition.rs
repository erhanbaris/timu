use std::{borrow::Cow, rc::Rc};

use crate::{
    ast::{ClassDefinitionAst, ClassDefinitionFieldAst},
    nom_tools::{Span, ToRange},
    tir::{context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, resolver::build_object_type, signature::SignatureHolder, ObjectSignature, TirError},
};

use super::ResolveSignature;

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
    pub fields: SignatureHolder<'base, ObjectSignatureValue<'base>>,
}

impl<'base> ResolveSignature<'base> for ClassDefinitionAst<'base> {
    type Item = Rc<ObjectSignature<'base>>;

    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>) -> Result<Self::Item, TirError<'base>> {
        simplelog::info!("Resolving class: <u><b>{}</b></u>", self.name.fragment());
        let mut fields = SignatureHolder::new();

        for field in self.fields.iter() {
            match field {
                ClassDefinitionFieldAst::ClassField(field) => {
                    let field_type = build_object_type(context, &field.field_type, module)?;
                    fields.add_signature((*field.name.fragment()).into(), field_type.clone())
                    .map_or(Ok(()), |_| Err(TirError::already_defined(field.name.to_range(), field_type.file.clone())))?;
                }
                ClassDefinitionFieldAst::ClassFunction(function) => {
                    let signature = function.resolve(context, module)?;
                    fields.add_signature((*function.name.fragment()).into(), signature.clone())
                    .map_or(Ok(()), |_| Err(TirError::already_defined(function.name.to_range(), signature.file.clone())))?;
                }
            };
        }
        
        let signature = Rc::new(ObjectSignature::new(ObjectSignatureValue::Class(ClassDefinition {
            name: self.name.clone(),
            fields,
        }), self.name.extra.file.clone(), self.name.to_range()));

        let module = context.modules.get_mut(module.as_ref()).unwrap();
        module.object_signatures.add_signature(Cow::Borrowed(self.name.fragment()), signature.clone())
            .map_or(Ok(()), |_| Err(TirError::already_defined(self.name.to_range(), signature.file.clone())))?;

        Ok(signature)
    }
    
    fn name(&self) -> &str {
        self.name.fragment()
    }
}
