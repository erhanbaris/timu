use crate::{
    ast::UseAst,
    tir::{context::TirContext, module::ModuleRef, TirError},
};

use super::ResolveSignature;

impl<'base> ResolveSignature<'base> for UseAst<'base> {
    type Item = ();

    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>) -> Result<Self::Item, TirError<'base>> {
        if let Some(signature) = context.get_ast_signature(&self.import.text) {
            let name = match &self.alias {
                Some(alias) => std::borrow::Cow::Borrowed(*alias.fragment()),
                None => std::borrow::Cow::Borrowed(*self.name().fragment()),
            };

            let module = context.modules.get_mut(module.as_ref()).unwrap();
            if module.imported_modules.insert(name, signature.clone()).is_some() {
                return Err(TirError::AstModuleAlreadyDefined {
                    position: self.import.to_range(),
                    source: self.name().extra.file.clone(),
                });
            }
        } else {
            return Err(TirError::ImportNotFound {
                module: self.import.text.clone(),
                position: self.import.to_range(),
                source: self.name().extra.file.clone(),
            });
        }

        Ok(())
    }
}
