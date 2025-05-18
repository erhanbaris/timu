use std::rc::Rc;

use crate::tir::{context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, ObjectSignature, TirError};

use super::{ResolveSignature, SignatureLocation};

impl<'base> ResolveSignature<'base> for ModuleRef<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>) -> Result<SignatureLocation, TirError<'base>> {
        let signature = Rc::new(ObjectSignature::new(ObjectSignatureValue::Module, self.file(), 0..0));
        let module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
        module.object_signatures.add_signature(self.as_cow(), signature.clone())
            .map_err(|_| TirError::already_defined(0..0, signature.file.clone()))
    }
    
    fn name(&self) -> &str {
        self.0.as_ref()
    }
}
