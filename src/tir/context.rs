use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use super::{AstSignature, AstSignatureHolder, Module, ObjectSignature, ObjectSignatureHolder};

#[derive(Debug, Default)]
pub struct TirContext<'base> {
    pub modules: HashMap<Cow<'base, str>, Rc<RefCell<Module<'base>>>>,
    ast_signatures: AstSignatureHolder<'base>,
    #[allow(dead_code)]
    pub object_signatures: ObjectSignatureHolder<'base>,
}

impl<'base> TirContext<'base> {
    #[allow(dead_code)]
    pub fn get_object_signature<T: AsRef<str>>(&self, key: T) -> Option<Rc<ObjectSignature<'base>>> {
        self.object_signatures.get(key.as_ref())
    }

    pub fn get_ast_signature<T: AsRef<str>>(&self, key: T) -> Option<Rc<AstSignature<'base>>> {
        self.ast_signatures.get(key.as_ref())
    }

    pub fn add_ast_signature(&mut self, key: String, signature: Rc<AstSignature<'base>>) -> Option<Rc<AstSignature<'base>>> {
        self.ast_signatures.add_signature(key, signature)
    }
}
