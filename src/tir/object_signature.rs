use std::{fmt::Debug, rc::Rc};

use super::resolver::function_definition::FunctionDefinition;

#[derive(Debug)]
pub enum ObjectSignatureValue<'base> {
    #[allow(dead_code)]
    Function(Rc<FunctionDefinition<'base>>),
    Class,
    Module,
    Interface,
}
