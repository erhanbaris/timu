use std::borrow::Cow;

use crate::tir::{context::TirContext, module::ModuleRef, object_signature::TypeValue, signature::SignaturePath, TirError, TypeSignature};

use super::{ResolveAst, TypeLocation};

impl<'base> ResolveAst<'base> for ModuleRef<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module_ref: &ModuleRef<'base>, _: Option<TypeLocation>) -> Result<TypeLocation, TirError<'base>> {
        let full_name = self.build_full_name(context, module_ref, None);
        let (signature_path, signature_location) = context.reserve_object_location(self.name(), SignaturePath::owned(full_name), module_ref, 0..0, self.file())?;
        let signature = TypeSignature::new(TypeValue::Module, self.file(), 0..0, None);
        context.publish_object_location(signature_path.clone(), signature);
        Ok(signature_location)
    }
    
    fn finish(&self, _: &mut TirContext<'base>, _: &ModuleRef<'base>, _: TypeLocation) -> Result<(), TirError<'base>> { Ok(()) }
    
    fn name(&self) -> Cow<'base, str> {
        self.0.clone()
    }
}
