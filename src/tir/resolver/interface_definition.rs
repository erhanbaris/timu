use std::{borrow::Cow, cell::RefMut, rc::Rc};

use crate::{
    ast::InterfaceDefinitionAst,
    nom_tools::ToRange,
    tir::{ObjectSignature, TirError, context::TirContext, module::Module, object_signature::ObjectSignatureValue},
};

use super::ResolveSignature;

impl<'base> ResolveSignature<'base> for InterfaceDefinitionAst<'base> {
    type Item = Rc<ObjectSignature<'base>>;

    fn resolve(&self, _: &'_ TirContext<'base>, module: &mut RefMut<'_, Module<'base>>) -> Result<Self::Item, TirError<'base>> {
        let signature = Rc::new(ObjectSignature::new(ObjectSignatureValue::Interface, self.name.extra.file.clone(), self.name.to_range()));
        module.object_signatures.add_signature(Cow::Borrowed(self.name.fragment()), signature.clone());
        Ok(signature)
    }
}
