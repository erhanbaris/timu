use std::fmt::Debug;

use strum_macros::EnumIs;

use super::resolver::{class::ClassDefinition, function::FunctionDefinition, interface::{InterfaceDefinition, InterfaceFunctionDefinition}, ObjectLocation, TypeLocation};

#[derive(Debug, PartialEq)]
pub enum PrimitiveType {
    Int,
    Float,
    String,
    Bool,
    Void,
}

#[derive(Debug, EnumIs, PartialEq)]
pub enum TypeValue<'base> {
    #[allow(dead_code)]
    Object(ObjectLocation),

    #[allow(dead_code)]
    PrimitiveType(PrimitiveType),
    
    #[allow(dead_code)]
    Function(FunctionDefinition<'base>),
    
    #[allow(dead_code)]
    Class(ClassDefinition<'base>),
    
    #[allow(dead_code)]
    Module,
    
    #[allow(dead_code)]
    Interface(InterfaceDefinition<'base>),
    
    #[allow(dead_code)]
    InterfaceFunction(InterfaceFunctionDefinition<'base>),
    
    #[allow(dead_code)]
    FunctionCall { callee: TypeLocation, arguments: Vec<TypeLocation> },
}

impl<'base> AsRef<TypeValue<'base>> for TypeValue<'base> {
    fn as_ref(&self) -> &TypeValue<'base> {
        self
    }
}

impl<'base> AsMut<TypeValue<'base>> for TypeValue<'base> {
    fn as_mut(&mut self) -> &mut TypeValue<'base> {
        self
    }
}

impl TypeValue<'_> {
    pub fn compare_skeleton(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeValue::PrimitiveType(left), TypeValue::PrimitiveType(right)) => Self::compare_primitive_types(left, right),
            (TypeValue::Object(left), TypeValue::Object(right)) => Self::compare_primitive_value(left, right),
            (TypeValue::Function(left_function), TypeValue::Function(right_function)) => Self::compare_functions(left_function, right_function),
            (TypeValue::Class(left_class), TypeValue::Class(right_class)) => Self::compare_classes(left_class, right_class),
            (TypeValue::Module, TypeValue::Module) => false,
            (TypeValue::InterfaceFunction(interface_function), TypeValue::Function(function)) => Self::compare_interface_function_and_function(interface_function, function),
            (TypeValue::Function(function), TypeValue::InterfaceFunction(interface_function)) => Self::compare_interface_function_and_function(interface_function, function),
            (TypeValue::InterfaceFunction(left_function), TypeValue::InterfaceFunction(right_function)) => Self::compare_interface_functions(left_function, right_function),
            _ => false,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            TypeValue::PrimitiveType(primitive) => match primitive {
                PrimitiveType::Int => "int",
                PrimitiveType::Float => "float",
                PrimitiveType::String => "string",
                PrimitiveType::Bool => "bool",
                PrimitiveType::Void => "void",
                
            },
            TypeValue::Object(_) => "PrimitiveValue",
            TypeValue::Function(function) => function.name.fragment(),
            TypeValue::Class(class) => class.name.fragment(),
            TypeValue::Module => "Module",
            TypeValue::Interface(interface) => interface.name.fragment(),
            TypeValue::InterfaceFunction(interface_function) => interface_function.name.fragment(),
            TypeValue::FunctionCall { .. } => "FunctionCall",
        }
    }

    fn compare_primitive_types(left: &PrimitiveType, right: &PrimitiveType) -> bool {
        std::ptr::eq(left, right)
    }

    fn compare_primitive_value(left: &ObjectLocation, right: &ObjectLocation) -> bool {
        left == right
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
