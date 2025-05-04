use std::{collections::HashMap, rc::Rc};

use super::{ModuleSignature, ProjectModule};

#[derive(Debug, Default)]
pub struct TirContext<'base> {
    pub modules: Vec<Rc<ProjectModule<'base>>>,
    pub signatures: HashMap<String, ModuleSignature<'base>>,
}

impl<'base> TirContext<'base> {
    pub fn get_signature(&self, key: &'base str) -> Option<&ModuleSignature<'base>> {
        self.signatures.get(key)
        /*for code in self.modules.iter() {
            if let Some(module) = code.get_signature(path) {
                return Some(module);
            }
        }
        
        None
        */
    }
}
