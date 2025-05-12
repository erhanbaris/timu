use std::{borrow::Cow, rc::Rc};

use indexmap::IndexMap;

use super::{AstSignature, AstSignatureHolder, Module, ObjectSignatureHolder};

#[derive(Debug, Default)]
pub struct TirContext<'base> {
    pub modules: IndexMap<Cow<'base, str>, Module<'base>>,
    ast_signatures: AstSignatureHolder<'base>,
    #[allow(dead_code)]
    pub object_signatures: ObjectSignatureHolder<'base>,
}

impl<'base> TirContext<'base> {
    pub fn get_ast_signature<T: AsRef<str>>(&self, key: T) -> Option<Rc<AstSignature<'base>>> {
        self.ast_signatures.get(key.as_ref())
    }

    pub fn add_ast_signature(&mut self, key: Cow<'base, str>, signature: Rc<AstSignature<'base>>) -> Option<Rc<AstSignature<'base>>> {
        self.ast_signatures.add_signature(key, signature)
    }
}
