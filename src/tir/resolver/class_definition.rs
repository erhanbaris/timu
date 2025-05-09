use std::cell::RefMut;

use crate::{
    ast::ClassDefinitionAst,
    nom_tools::ToRange,
    tir::{ObjectSignature, TirError, context::TirContext, module::Module, object_signature::ObjectSignatureValue},
};

use super::ResolveSignature;

impl<'base> ResolveSignature<'base> for ClassDefinitionAst<'base> {
    fn resolve(&self, _: &'_ TirContext<'base>, module: &mut RefMut<'_, Module<'base>>) -> Result<(), TirError<'base>> {
        let signature = ObjectSignature::new(ObjectSignatureValue::Class, self.name.extra.file.clone(), self.name.to_range());
        module.object_signatures.add_signature(self.name.fragment().to_string(), signature.into());
        Ok(())
    }
}
