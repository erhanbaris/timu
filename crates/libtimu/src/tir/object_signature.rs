use std::fmt::Debug;

use super::resolver::{class::ClassDefinition, function::{ClassFunctionSignature, FunctionDefinition}, interface::{InterfaceDefinition, InterfaceFunctionDefinition}};

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
    #[allow(dead_code)]
    ClassFunctionSignature(ClassFunctionSignature<'base>),
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

impl ObjectSignatureValue<'_> {
    pub fn compare_skeleton(&self, other: &Self) -> bool {
        match (self, other) {
            (ObjectSignatureValue::Function(left_function), ObjectSignatureValue::Function(right_function)) => Self::compare_functions(left_function, right_function),
            (ObjectSignatureValue::Class(left_class), ObjectSignatureValue::Class(right_class)) => Self::compare_classes(left_class, right_class),
            (ObjectSignatureValue::Module, ObjectSignatureValue::Module) => false,
            (ObjectSignatureValue::InterfaceFunction(interface_function), ObjectSignatureValue::Function(function)) => Self::compare_interface_function_and_function(interface_function, function),
            (ObjectSignatureValue::Function(function), ObjectSignatureValue::InterfaceFunction(interface_function)) => Self::compare_interface_function_and_function(interface_function, function),
            (ObjectSignatureValue::InterfaceFunction(left_function), ObjectSignatureValue::InterfaceFunction(right_function)) => Self::compare_interface_functions(left_function, right_function),
            _ => false,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            ObjectSignatureValue::Function(function) => function.name.fragment(),
            ObjectSignatureValue::Class(class) => class.name.fragment(),
            ObjectSignatureValue::Module => "Module",
            ObjectSignatureValue::Interface(interface) => interface.name.fragment(),
            ObjectSignatureValue::InterfaceFunction(interface_function) => interface_function.name.fragment(),
            ObjectSignatureValue::ClassFunctionSignature(function) => function.name.fragment(),
        }
    }

    fn compare_classes(left: &ClassDefinition, right: &ClassDefinition) -> bool {
        std::ptr::eq(left, right)
    }

    fn compare_interface_functions(left: &InterfaceFunctionDefinition, right: &InterfaceFunctionDefinition) -> bool {
        if left.name.fragment() != right.name.fragment() ||
            left.arguments.len() != right.arguments.len() ||
            left.return_type != right.return_type {
            return false;
        }

        for (left_arg, right_arg) in left.arguments.iter().zip(right.arguments.iter()) {
            if left_arg.name.fragment() != right_arg.name.fragment() || left_arg.field_type != right_arg.field_type {
                return false;
            }
        }

        true
    }

    fn compare_interface_function_and_function(left: &InterfaceFunctionDefinition, right: &FunctionDefinition) -> bool {
        if left.name.fragment() != right.name.fragment() ||
            left.arguments.len() != right.arguments.len() ||
            left.return_type != right.return_type {
            return false;
        }

        for (left_arg, right_arg) in left.arguments.iter().zip(right.arguments.iter()) {
            if left_arg.name.fragment() != right_arg.name.fragment() || left_arg.field_type != right_arg.field_type {
                return false;
            }
        }

        true
    }

    fn compare_functions(left: &FunctionDefinition, right: &FunctionDefinition) -> bool {
        if left.name.fragment() != right.name.fragment() ||
            left.arguments.len() != right.arguments.len() ||
            left.return_type != right.return_type ||
            left.is_public != right.is_public {
            return false;
        }

        for (left_arg, right_arg) in left.arguments.iter().zip(right.arguments.iter()) {
            if left_arg.name.fragment() != right_arg.name.fragment() || left_arg.field_type != right_arg.field_type {
                return false;
            }
        }

        true
    }
} 
