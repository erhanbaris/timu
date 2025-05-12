use std::{fmt::Debug, rc::Rc};

use super::resolver::{class_definition::ClassDefinition, function_definition::FunctionDefinition};

#[derive(Debug)]
pub enum ObjectSignatureValue<'base> {
    #[allow(dead_code)]
    Function(Rc<FunctionDefinition<'base>>),
    #[allow(dead_code)]
    Class(ClassDefinition<'base>),
    Module,
    Interface,
}
