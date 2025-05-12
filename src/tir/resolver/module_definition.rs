use std::rc::Rc;

use crate::tir::{context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, ObjectSignature, TirError};

use super::ResolveSignature;

impl<'base> ResolveSignature<'base> for ModuleRef<'base> {
    type Item = Rc<ObjectSignature<'base>>;

    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>) -> Result<Self::Item, TirError<'base>> {
        let signature = Rc::new(ObjectSignature::new(ObjectSignatureValue::Module, self.file(), 0..0));
        let module = context.modules.get_mut(module.as_ref()).unwrap();
        module.object_signatures.add_signature(self.as_cow(), signature.clone());
        Ok(signature)
    }
    
    fn name(&self) -> &str {
        self.0.as_ref()
    }
}
