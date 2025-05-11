use std::{borrow::Cow, rc::Rc};

use crate::{
    ast::InterfaceDefinitionAst,
    nom_tools::ToRange,
    tir::{context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, ObjectSignature, TirError},
};

use super::ResolveSignature;

impl<'base> ResolveSignature<'base> for InterfaceDefinitionAst<'base> {
    type Item = Rc<ObjectSignature<'base>>;

    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>) -> Result<Self::Item, TirError<'base>> {
        let signature = Rc::new(ObjectSignature::new(ObjectSignatureValue::Interface, self.name.extra.file.clone(), self.name.to_range()));
        let module = context.modules.get_mut(module.as_ref()).unwrap();
        module.object_signatures.add_signature(Cow::Borrowed(self.name.fragment()), signature.clone());
        Ok(signature)
    }
}
