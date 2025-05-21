use std::{borrow::Cow, rc::Rc};

use crate::tir::{context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, ObjectSignature, TirError};

use super::{ResolveSignature, SignatureLocation};

impl<'base> ResolveSignature<'base> for ModuleRef<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>) -> Result<SignatureLocation, TirError<'base>> {
        let (signature_path, signature_location) = context.reserve_object_location(self.as_cow(), module, 0..0, self.file())?;
        let signature = Rc::new(ObjectSignature::new(ObjectSignatureValue::Module, self.file(), 0..0));
        context.update_object_location(signature_path.clone(), signature.clone());
        Ok(signature_location)
    }
    
    fn name(&self) -> Cow<'base, str> {
        self.0.clone()
    }
}
