use std::cell::RefMut;

use crate::{ast::InterfaceDefinitionAst, tir::{context::TirContext, module::Module, TirError}};

use super::ResolveSignature;

impl<'base> ResolveSignature<'base> for InterfaceDefinitionAst<'base> {
    fn resolve(&self, _: &'_ TirContext<'base>, _: &mut RefMut<'_, Module<'base>>) -> Result<(), TirError<'base>> {
        Ok(())
    }
}
