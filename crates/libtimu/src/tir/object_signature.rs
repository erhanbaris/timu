use std::fmt::Debug;

use strum_macros::{EnumDiscriminants, EnumIs};

use crate::tir::{module::ModuleRef, resolver::TypeLocation};

use super::{resolver::{class::ClassDefinition, function::FunctionDefinition, interface::{InterfaceDefinition, InterfaceFunctionDefinition}}, TirContext};

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    String,
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    Float,
    Double,
    Void,
}

impl GetItem for PrimitiveType {
    fn get_item_location(&self, _: &TirContext<'_>, _: &str) -> Option<TypeLocation> {
        None
    }
}

#[derive(Debug, Clone, EnumIs, EnumDiscriminants, PartialEq)]
#[strum_discriminants(vis(pub))]
pub enum TypeValue<'base> {
    #[allow(dead_code)]
    PrimitiveType(PrimitiveType),
    
    #[allow(dead_code)]
    Function(Box<FunctionDefinition<'base>>),
    
    #[allow(dead_code)]
    Class(ClassDefinition<'base>),
    
    #[allow(dead_code)]
    Module(ModuleRef<'base>),
    
    #[allow(dead_code)]
    Interface(InterfaceDefinition<'base>),
    
    #[allow(dead_code)]
    InterfaceFunction(InterfaceFunctionDefinition<'base>),
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

pub trait GetItem {
    fn get_item_location(&self, context: &TirContext<'_>, path: &str) -> Option<TypeLocation>;
}

impl GetItem for TypeValue<'_> {
    fn get_item_location(&self, context: &TirContext<'_>, path: &str) -> Option<TypeLocation> {
        match self {
            TypeValue::PrimitiveType(primitive_type) => primitive_type.get_item_location(context, path),
            TypeValue::Function(function_definition) => function_definition.get_item_location(context, path),
            TypeValue::Class(class_definition) => class_definition.get_item_location(context, path),
            TypeValue::Module(module_ref) => module_ref.get_item_location(context, path),
            TypeValue::Interface(interface_definition) => interface_definition.get_item_location(context, path),
            TypeValue::InterfaceFunction(interface_function_definition) => interface_function_definition.get_item_location(context, path),
        }
    }
}
impl TypeValue<'_> {
    pub fn is_same_type(&self, context: &TirContext<'_>, other: &Self) -> bool {
        match (self, other) {
            (TypeValue::PrimitiveType(left), TypeValue::PrimitiveType(right)) => Self::compare_primitive_types(left, right),
            (TypeValue::Function(left_function), TypeValue::Function(right_function)) => Self::compare_functions(left_function, right_function),
            (TypeValue::Class(left_class), TypeValue::Class(right_class)) => Self::compare_classes(left_class, right_class),
            (TypeValue::Module(_), TypeValue::Module(_)) => false,
            (TypeValue::InterfaceFunction(interface_function), TypeValue::Function(function)) => Self::compare_interface_function_and_function(interface_function, function),
            (TypeValue::Function(function), TypeValue::InterfaceFunction(interface_function)) => Self::compare_interface_function_and_function(interface_function, function),
            (TypeValue::InterfaceFunction(left_function), TypeValue::InterfaceFunction(right_function)) => Self::compare_interface_functions(left_function, right_function),
            (TypeValue::Interface(interface), TypeValue::Class(class)) => Self::compare_interface_and_class(context, interface, class),
            _ => false,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            TypeValue::PrimitiveType(primitive) => match primitive {
                PrimitiveType::String => "String",
                PrimitiveType::Bool => "Bool",
                PrimitiveType::I8 => "I8",
                PrimitiveType::U8 => "U8",
                PrimitiveType::I16 => "I16",
                PrimitiveType::U16 => "U16",
                PrimitiveType::I32 => "I32",
                PrimitiveType::U32 => "U32",
                PrimitiveType::I64 => "I64",
                PrimitiveType::U64 => "U64",
                PrimitiveType::Float => "Float",
                PrimitiveType::Double => "Double",
                PrimitiveType::Void => "Void",
            },
            TypeValue::Function(function) => function.name.text,
            TypeValue::Class(class) => class.name.text,
            TypeValue::Module(_) => "Module",
            TypeValue::Interface(interface) => interface.name.text,
            TypeValue::InterfaceFunction(interface_function) => interface_function.name.text,
        }
    }

    fn compare_primitive_types(left: &PrimitiveType, right: &PrimitiveType) -> bool {
        std::ptr::eq(left, right)
    }

    fn compare_classes(left: &ClassDefinition, right: &ClassDefinition) -> bool {
        std::ptr::eq(left, right)
    }

    fn compare_interface_functions(left: &InterfaceFunctionDefinition, right: &InterfaceFunctionDefinition) -> bool {
        if left.name.text != right.name.text ||
            left.arguments.len() != right.arguments.len() ||
            left.return_type != right.return_type {
            return false;
        }

        for (left_arg, right_arg) in left.arguments.iter().zip(right.arguments.iter()) {
            if left_arg.name.text != right_arg.name.text || left_arg.field_type != right_arg.field_type {
                return false;
            }
        }

        true
    }

    fn compare_interface_and_class(context: &TirContext<'_>, interface: &InterfaceDefinition, class: &ClassDefinition) -> bool {
        for type_location in class.extends.iter() {
            if let Some(TypeValue::Interface(class_interface)) = context.types.get_from_location(*type_location).map(|signature| signature.value.as_ref()) {
                if class_interface.full_name == interface.full_name {
                    return true;
                }
            }
        }
        
        false
    }

    fn compare_interface_function_and_function(left: &InterfaceFunctionDefinition, right: &FunctionDefinition) -> bool {
        if left.name.text != right.name.text ||
            left.arguments.len() != right.arguments.len() ||
            left.return_type != right.return_type {
            return false;
        }

        for (left_arg, right_arg) in left.arguments.iter().zip(right.arguments.iter()) {
            if left_arg.name.text != right_arg.name.text || left_arg.field_type != right_arg.field_type {
                return false;
            }
        }

        true
    }

    fn compare_functions(left: &FunctionDefinition, right: &FunctionDefinition) -> bool {
        if left.name.text != right.name.text ||
            left.arguments.len() != right.arguments.len() ||
            left.return_type != right.return_type ||
            left.is_public != right.is_public {
            return false;
        }

        for (left_arg, right_arg) in left.arguments.iter().zip(right.arguments.iter()) {
            if left_arg.name.text != right_arg.name.text || left_arg.field_type != right_arg.field_type {
                return false;
            }
        }

        true
    }
} 
