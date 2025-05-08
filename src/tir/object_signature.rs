use std::{fmt::Debug, rc::Rc};

use crate::nom_tools::ToRange;

use super::{resolver::function_definition::FunctionDefinition, signature::Signature};

#[derive(Debug)]
pub enum ObjectSignatureValue<'base> {
    Function(#[allow(dead_code)]Rc<FunctionDefinition<'base>>),
    Class,
    Module
}

impl<'base> From<Rc<FunctionDefinition<'base>>> for Signature<'base, ObjectSignatureValue<'base>> {
    fn from(function: Rc<FunctionDefinition<'base>>) -> Self {
        let position = function.name.to_range();
        let file = function.name.extra.file.clone();

        Signature::new(
            ObjectSignatureValue::Function(function),
            file,
            position,
        )
    }
}
