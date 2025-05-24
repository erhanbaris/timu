use std::borrow::Cow;

use crate::{
    ast::UseAst,
    tir::{context::TirContext, module::ModuleRef, TirError},
};

use super::{ResolveSignature, ObjectLocation};

impl<'base> ResolveSignature<'base> for UseAst<'base> {
    fn definition(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, _: Option<ObjectLocation>) -> Result<ObjectLocation, TirError<'base>> {
        if let Some(signature_location) = context.get_ast_location(&self.import.text) {
            let name = match &self.alias {
                Some(alias) => std::borrow::Cow::Borrowed(*alias.fragment()),
                None => std::borrow::Cow::Borrowed(*self.name().fragment()),
            };

            let module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
            if module.ast_imported_modules.insert(name, signature_location).is_some() {
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

        Ok(ObjectLocation::UNDEFINED)
    }

    fn finish(&self, _: &mut TirContext<'base>, _: &ModuleRef<'base>, _: ObjectLocation) -> Result<(), TirError<'base>> { Ok(()) }
    
    fn name(&self) -> Cow<'base, str> {
        if let Some(alias) = &self.alias {
            Cow::Borrowed(*alias.fragment())
        } else {
            Cow::Borrowed(*self.name().fragment())
        }
    }
}
