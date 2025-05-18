use std::{borrow::Cow, rc::Rc};

use crate::{
    ast::InterfaceDefinitionAst,
    nom_tools::ToRange,
    tir::{context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, ObjectSignature, TirError},
};

use super::{ResolveSignature, SignatureLocation};

impl<'base> ResolveSignature<'base> for InterfaceDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>) -> Result<SignatureLocation, TirError<'base>> {
        let signature = Rc::new(ObjectSignature::new(ObjectSignatureValue::Interface, self.name.extra.file.clone(), self.name.to_range()));
        let module = context.modules.get_mut(module.as_ref()).unwrap();
        module.object_signatures.add_signature(Cow::Borrowed(self.name.fragment()), signature.clone())
            .map_err(|_| TirError::already_defined(self.name.to_range(), signature.file.clone()))
    }
    
    fn name(&self) -> &str {
        self.name.fragment()
    }

    fn full_path(&self, module: &ModuleRef<'base>) -> String {
        format!("{}.{}", module.as_ref(), self.name.fragment())
    }
}
