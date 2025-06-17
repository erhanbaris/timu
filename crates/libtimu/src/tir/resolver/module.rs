use std::borrow::Cow;

use crate::tir::{context::TirContext, module::ModuleRef, object_signature::{TypeValue, TypeValueDiscriminants}, scope::ScopeLocation, signature::SignaturePath, TirError, TypeSignature};

use super::{BuildFullNameLocater, ResolveAst, TypeLocation};

impl<'base> ResolveAst<'base> for ModuleRef<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        let full_name = self.build_full_name(context, BuildFullNameLocater::Scope(scope_location), None);
        let module_ref = context.get_scope(scope_location).unwrap().module_ref.clone();
        let (signature_path, signature_location) = context.reserve_object_location(self.name(), TypeValueDiscriminants::Module, SignaturePath::owned(full_name), &module_ref, 0..0, self.file())?;
        let signature = TypeSignature::new(TypeValue::Module(module_ref), self.file(), 0..0, None);
        context.publish_object_location(signature_path.clone(), signature);
        Ok(signature_location)
    }
    
    fn finish(&self, _: &mut TirContext<'base>, _: ScopeLocation) -> Result<(), TirError> { Ok(()) }
    
    fn name(&self) -> Cow<'base, str> {
        self.0.clone()
    }
}
