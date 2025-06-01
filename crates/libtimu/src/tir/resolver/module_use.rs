use std::borrow::Cow;

use miette::SourceSpan;

use crate::{
    ast::UseAst, tir::{context::TirContext, error::{ImportNotFound, ModuleAlreadyImported}, scope::ScopeLocation, TirError}
};

use super::{ResolveAst, TypeLocation};

impl<'base> ResolveAst<'base> for UseAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        if let Some(signature_location) = context.get_ast_location(&self.import.text) {
            let name = match &self.alias {
                Some(alias) => std::borrow::Cow::Borrowed(*alias.fragment()),
                None => std::borrow::Cow::Borrowed(*self.ast_name().fragment()),
            };

            let module_ref = context.get_scope(scope_location).unwrap().module_ref.clone();
            let module = context.modules.get_mut(module_ref.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module_ref.as_ref()));
            if let Some(old) = module.ast_imported_modules.insert(name, signature_location) {
                let old_signature  = context.ast_signatures.get_from_location(old).unwrap();
                return Err(TirError::ModuleAlreadyImported(ModuleAlreadyImported {
                    new_position: self.import.to_range().into(),
                    old_position: old_signature.position.clone().into(),
                    code: self.ast_name().extra.file.clone().into(),
                }.into()));
            }
        } else {
            return Err(TirError::ImportNotFound(ImportNotFound {
                module: self.import.text.to_string(),
                position: SourceSpan::from(self.import.to_range()),
                code: self.ast_name().extra.file.into(),
            }.into()));
        }

        Ok(TypeLocation::UNDEFINED)
    }

    fn finish(&self, _: &mut TirContext<'base>, _: ScopeLocation) -> Result<(), TirError> { Ok(()) }
    
    fn name(&self) -> Cow<'base, str> {
        if let Some(alias) = &self.alias {
            Cow::Borrowed(*alias.fragment())
        } else {
            Cow::Borrowed(*self.ast_name().fragment())
        }
    }
}
