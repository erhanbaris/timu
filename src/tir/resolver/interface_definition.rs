use std::cell::RefMut;

use crate::{
    ast::InterfaceDefinitionAst,
    tir::{TirError, context::TirContext, module::Module},
};

use super::ResolveSignature;

impl<'base> ResolveSignature<'base> for InterfaceDefinitionAst<'base> {
    fn resolve(&self, _: &'_ TirContext<'base>, _: &mut RefMut<'_, Module<'base>>) -> Result<(), TirError<'base>> {
        Ok(())
    }
}
