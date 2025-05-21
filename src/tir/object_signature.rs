use std::fmt::Debug;

use super::resolver::{class_definition::ClassDefinition, function_definition::FunctionDefinition, interface_definition::{InterfaceDefinition, InterfaceFunctionDefinition}};

#[derive(Debug)]
pub enum ObjectSignatureValue<'base> {
    #[allow(dead_code)]
    Function(FunctionDefinition<'base>),
    #[allow(dead_code)]
    Class(ClassDefinition<'base>),
    Module,
    #[allow(dead_code)]
    Interface(InterfaceDefinition<'base>),
    #[allow(dead_code)]
    InterfaceFunction(InterfaceFunctionDefinition<'base>),
}

impl<'base> AsRef<ObjectSignatureValue<'base>> for ObjectSignatureValue<'base> {
    fn as_ref(&self) -> &ObjectSignatureValue<'base> {
        self
    }
}

impl<'base> AsMut<ObjectSignatureValue<'base>> for ObjectSignatureValue<'base> {
    fn as_mut(&mut self) -> &mut ObjectSignatureValue<'base> {
        self
    }
}
