use std::borrow::Cow;

use crate::{
    ast::UseAst,
    tir::{context::TirContext, module::ModuleRef, TirError},
};

use super::{ResolveSignature, SignatureLocation};

impl<'base> ResolveSignature<'base> for UseAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>) -> Result<SignatureLocation, TirError<'base>> {
        if let Some(signature) = context.get_ast_signature(&self.import.text) {
            let name = match &self.alias {
                Some(alias) => std::borrow::Cow::Borrowed(*alias.fragment()),
                None => std::borrow::Cow::Borrowed(*self.name().fragment()),
            };

            let module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
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

        Ok(SignatureLocation(usize::MAX))
    }

    fn name(&self) -> Cow<'base, str> {
        if let Some(alias) = &self.alias {
            Cow::Borrowed(*alias.fragment())
        } else {
            Cow::Borrowed(*self.name().fragment())
        }
    }
}
