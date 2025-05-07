use std::cell::RefMut;

use crate::{ast::UseAst, tir::{context::TirContext, module::Module, TirError}};

use super::ResolveSignature;

impl<'base> ResolveSignature<'base> for UseAst<'base> {
    fn resolve(&self, context: &'_ TirContext<'base>, module: &mut RefMut<'_, Module<'base>>) -> Result<(), TirError<'base>> {
        if let Some(signature) = context.get_ast_signature(&self.import.text) {
            
            let name = match &self.alias {
                Some(alias) => std::borrow::Cow::Borrowed(*alias.fragment()),
                None => std::borrow::Cow::Borrowed(*self.name().fragment()),
            };
    
            if module.imported_modules.insert(name, signature.clone()).is_some() {                
                return Err(TirError::AstModuleAlreadyDefined {
                    position: self.import.to_range(),
                    source: self.name().extra.file.clone(),
                });
            }
        } else {
            return Err(TirError::ModuleNotFound {
                module: self.import.text.clone(),
                position: self.import.to_range(),
                source: self.name().extra.file.clone(),
            });
        }

        Ok(())
    }
}
