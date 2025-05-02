use crate::nom_tools::Span;

use super::ProjectModule;

#[derive(Debug, Default)]
pub struct TirContext<'a> {
    pub modules: Vec<ProjectModule<'a>>,
}

impl<'a> TirContext<'a> {
    pub fn get_module(&self, path: &[Span<'a>]) -> Option<&ProjectModule<'a>> {
        for module in self.modules.iter() {
            if let Some(module) = module.get_module(path) {
                return Some(module);
            }
        }

        None
    }
}
