use std::{cell::RefMut, rc::Rc};

use crate::tir::{ObjectSignature, TirError, context::TirContext, module::Module, object_signature::ObjectSignatureValue};

use super::ResolveSignature;

impl<'base> ResolveSignature<'base> for Module<'base> {
    type Item = Rc<ObjectSignature<'base>>;

    fn resolve(&self, _: &'_ TirContext<'base>, module: &mut RefMut<'_, Module<'base>>) -> Result<Self::Item, TirError<'base>> {
        let signature = Rc::new(ObjectSignature::new(ObjectSignatureValue::Module, self.file.clone(), 0..0));
        module.object_signatures.add_signature(self.name.to_string(), signature.clone());
        Ok(signature)
    }
}
