use std::borrow::Cow;

use crate::tir::{context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, ObjectSignature, TirError};

use super::{ResolveSignature, ObjectLocation};

impl<'base> ResolveSignature<'base> for ModuleRef<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, _: Option<ObjectLocation>) -> Result<ObjectLocation, TirError<'base>> {
        let (signature_path, signature_location) = context.reserve_object_location(self.as_cow(), module, 0..0, self.file())?;
        let signature = ObjectSignature::new(ObjectSignatureValue::Module, self.file(), 0..0);
        context.publish_object_location(signature_path.clone(), signature);
        Ok(signature_location)
    }
    
    fn name(&self) -> Cow<'base, str> {
        self.0.clone()
    }
}
