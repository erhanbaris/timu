use std::{cell::RefCell, rc::Rc};

use super::{Module, ModuleSignature, signature::SignatureHolder};

#[derive(Debug, Default)]
pub struct TirContext<'base> {
    pub modules: Vec<Rc<RefCell<Module<'base>>>>,
    pub signatures: SignatureHolder<'base>,
}

impl<'base> TirContext<'base> {
    pub fn get_signature<T: AsRef<str>>(&self, key: T) -> Option<Rc<ModuleSignature<'base>>> {
        self.signatures.get(key.as_ref())
    }
}
