use std::fmt::Debug;

use strum_macros::EnumIs;

use super::{resolver::{class::ClassDefinition, function::FunctionDefinition, interface::{InterfaceDefinition, InterfaceFunctionDefinition}, ObjectLocation, TypeLocation}, TirContext};

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

#[derive(Debug, Clone, EnumIs, PartialEq)]
pub enum ExpressionValue {    
    #[allow(dead_code)]
    FunctionCall { callee: TypeLocation, arguments: Vec<TypeLocation> },
}

#[derive(Debug, Clone, EnumIs, PartialEq)]
pub enum StatementValue {    
    #[allow(dead_code)]
    VariableAssign(TypeLocation, ExpressionValue),
}

impl StatementValue {
    pub fn get_name(&self) -> &str {
        match self {
            StatementValue::VariableAssign(_, _) => "VariableAssign",
        }
    }
}

#[derive(Debug, Clone, EnumIs, PartialEq)]
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
    Statement(StatementValue),
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
    pub fn compare_skeleton(&self, context: &TirContext<'_>, other: &Self) -> bool {
        match (self, other) {
            (TypeValue::PrimitiveType(left), TypeValue::PrimitiveType(right)) => Self::compare_primitive_types(left, right),
            (TypeValue::Object(left), TypeValue::Object(right)) => Self::compare_primitive_value(left, right),
            (TypeValue::PrimitiveType(left), TypeValue::Object(right)) => Self::compare_primitive_object(context, left, right),
            (TypeValue::Function(left_function), TypeValue::Function(right_function)) => Self::compare_functions(left_function, right_function),
            (TypeValue::Class(left_class), TypeValue::Class(right_class)) => Self::compare_classes(left_class, right_class),
            (TypeValue::Module, TypeValue::Module) => false,
            (TypeValue::InterfaceFunction(interface_function), TypeValue::Function(function)) => Self::compare_interface_function_and_function(interface_function, function),
            (TypeValue::Function(function), TypeValue::InterfaceFunction(interface_function)) => Self::compare_interface_function_and_function(interface_function, function),
            (TypeValue::InterfaceFunction(left_function), TypeValue::InterfaceFunction(right_function)) => Self::compare_interface_functions(left_function, right_function),
            _ => false,
        }
    }

    fn compare_primitive_object(context: &TirContext<'_>, primitive: &PrimitiveType, object_location: &ObjectLocation) -> bool {
        if let Some(object) = context.objects.get_from_location(object_location.clone()) {
           return *primitive == object.to_type();
        }
        false
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
            TypeValue::Object(_) => "PrimitiveValue",
            TypeValue::Function(function) => function.name.fragment(),
            TypeValue::Class(class) => class.name.fragment(),
            TypeValue::Module => "Module",
            TypeValue::Interface(interface) => interface.name.fragment(),
            TypeValue::InterfaceFunction(interface_function) => interface_function.name.fragment(),
            TypeValue::Statement(statement) => statement.get_name(),
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
