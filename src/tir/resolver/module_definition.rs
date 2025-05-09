use std::cell::RefMut;

use crate::tir::{ObjectSignature, TirError, context::TirContext, module::Module, object_signature::ObjectSignatureValue};

use super::ResolveSignature;

impl<'base> ResolveSignature<'base> for Module<'base> {
    fn resolve(&self, _: &'_ TirContext<'base>, module: &mut RefMut<'_, Module<'base>>) -> Result<(), TirError<'base>> {
        let signature = ObjectSignature::new(ObjectSignatureValue::Module, self.file.clone(), 0..0);
        module.object_signatures.add_signature(self.name.to_string(), signature.into());
        Ok(())
    }
}
