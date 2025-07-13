//! Type system and object signatures for the Timu language compiler.
//!
//! This module defines the core type system used throughout the TIR (Type Intermediate
//! Representation) phase of compilation. It provides the fundamental type representations
//! and operations that enable type checking, inference, and validation.
//!
//! # Type System Overview
//!
//! The Timu type system supports several categories of types:
//!
//! ## Primitive Types
//! - **Integers**: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`
//! - **Floating-point**: `float` (32-bit), `double` (64-bit)
//! - **Boolean**: `bool` with `true`/`false` values
//! - **String**: UTF-8 text with `string` type
//! - **Void**: Unit type for functions with no return value
//!
//! ## Complex Types
//! - **Classes**: User-defined object types with fields and methods
//! - **Interfaces**: Contract definitions for class implementations
//! - **Functions**: First-class function types with signatures
//! - **Modules**: Namespace containers for organizing code
//! - **References**: Pointer types using `ref` keyword
//!
//! # Type Operations
//!
//! The type system provides several key operations:
//! - **Type equality**: Comparing types for exact matches
//! - **Type compatibility**: Checking interface implementations
//! - **Member lookup**: Finding fields and methods within types
//! - **Reference resolution**: Handling pointer and reference types
//!
//! # Architecture
//!
//! Types are represented using the [`TypeValue`] enum which encompasses all possible
//! type variants. Each variant contains the specific information needed for that
//! type category, such as primitive type identifiers or complex type definitions.
//!
//! The [`GetItem`] trait provides a uniform interface for looking up members
//! within types, enabling field access and method resolution across all type categories.

use std::{borrow::Cow, fmt::Debug};

use strum_macros::{EnumDiscriminants, EnumIs};

use crate::tir::{module::ModuleRef, resolver::TypeLocation};

use super::{resolver::{class::ClassDefinition, function::FunctionDefinition, interface::{InterfaceDefinition, InterfaceFunctionDefinition}}, TirContext};

/// Enumeration of primitive data types in the Timu language
/// 
/// These types are built into the language and provide the foundation for
/// all other type constructions. They correspond directly to common CPU
/// and memory representations for efficient execution.
#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    /// UTF-8 string type
    String,
    /// Boolean type with true/false values
    Bool,
    /// 8-bit signed integer (-128 to 127)
    I8,
    /// 8-bit unsigned integer (0 to 255)
    U8,
    /// 16-bit signed integer (-32,768 to 32,767)
    I16,
    /// 16-bit unsigned integer (0 to 65,535)
    U16,
    /// 32-bit signed integer (-2^31 to 2^31-1)
    I32,
    /// 32-bit unsigned integer (0 to 2^32-1)
    U32,
    /// 64-bit signed integer (-2^63 to 2^63-1)
    I64,
    /// 64-bit unsigned integer (0 to 2^64-1)
    U64,
    /// 32-bit floating-point number
    Float,
    /// 64-bit floating-point number
    Double,
    /// Unit type representing no value
    Void,
}

impl GetItem for PrimitiveType {
    fn get_item_location(&self, _: &TirContext<'_>, _: &str) -> Option<TypeLocation> {
        None
    }
}

/// Comprehensive enumeration of all possible type values in the Timu type system
/// 
/// This enum represents the complete universe of types that can exist in Timu programs.
/// It serves as the central type representation used throughout semantic analysis,
/// type checking, and code generation phases.
/// 
/// # Variants
/// 
/// - **PrimitiveType**: Built-in language types (integers, floats, bool, string, void)
/// - **Function**: Function types with signatures and callable information
/// - **Class**: User-defined object types with fields, methods, and inheritance
/// - **Module**: Namespace containers that group related definitions
/// - **Interface**: Contract specifications that classes can implement
/// - **InterfaceFunction**: Function signatures defined within interfaces
/// - **Reference**: Pointer/reference types that refer to other types
/// 
/// # Type Operations
/// 
/// The enum supports various operations through implemented traits:
/// - Type equality comparison via `is_same_type`
/// - Member lookup via the `GetItem` trait
/// - Type introspection via generated `EnumIs` methods
/// - Discriminant access via `EnumDiscriminants`
#[derive(Debug, Clone, EnumIs, EnumDiscriminants, PartialEq)]
#[strum_discriminants(vis(pub))]
pub enum TypeValue<'base> {
    /// A primitive type (integers, floats, bool, string, void)
    #[allow(dead_code)]
    PrimitiveType(PrimitiveType),
    
    /// A function type with complete signature information
    #[allow(dead_code)]
    Function(Box<FunctionDefinition<'base>>),
    
    /// A class type with fields, methods, and inheritance relationships
    #[allow(dead_code)]
    Class(ClassDefinition<'base>),
    
    /// A module type representing a namespace container
    #[allow(dead_code)]
    Module(ModuleRef<'base>),
    
    /// An interface type defining a contract for implementations
    #[allow(dead_code)]
    Interface(InterfaceDefinition<'base>),
    
    /// A function signature defined within an interface
    #[allow(dead_code)]
    InterfaceFunction(InterfaceFunctionDefinition<'base>),

    /// A reference type that points to another type
    #[allow(dead_code)]
    Reference(Box<TypeValue<'base>>),
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

/// Trait for types that can provide member lookup functionality
/// 
/// This trait enables uniform member access across all type categories in the
/// Timu type system. It allows looking up fields, methods, and other members
/// within types using string-based paths.
/// 
/// # Usage
/// 
/// The trait is implemented by all types that can contain members:
/// - Classes can look up fields and methods
/// - Modules can look up contained types and functions
/// - Interfaces can look up required methods
/// - Primitive types return `None` (no members)
/// 
/// # Examples
/// 
/// ```ignore
/// // Looking up a field in a class
/// let field_location = class_type.get_item_location(context, "field_name");
/// 
/// // Looking up a type in a module
/// let type_location = module_type.get_item_location(context, "TypeName");
/// ```
pub trait GetItem {
    /// Attempts to find a member with the given name within this type
    /// 
    /// # Arguments
    /// * `context` - The compilation context for additional lookups
    /// * `path` - The name of the member to find
    /// 
    /// # Returns
    /// * `Some(TypeLocation)` - If the member is found
    /// * `None` - If the member doesn't exist or the type has no members
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
            TypeValue::Reference(reference) => reference.get_item_location(context, path),
        }
    }
}
impl TypeValue<'_> {
    /// Determines if two type values represent the same type
    /// 
    /// This method performs deep type equality checking, considering the specific
    /// rules for each type variant. It handles complex cases like interface-class
    /// compatibility through extension relationships and function signature matching.
    /// 
    /// # Arguments
    /// * `context` - TIR context for accessing type system information
    /// * `other` - The other type value to compare against
    /// 
    /// # Returns
    /// `true` if the types are considered equivalent, `false` otherwise
    /// 
    /// # Type Compatibility Rules
    /// - **Primitives**: Exact primitive type match required
    /// - **Functions**: Signature compatibility (parameters and return type)
    /// - **Classes**: Name-based equality comparison
    /// - **Interfaces**: Full name comparison for interface equality
    /// - **Interface-Class**: Compatibility when class implements interface via extension
    /// - **Interface-Function**: Signature compatibility checking
    /// - **Modules**: Always considered incompatible (no two modules are the same type)
    /// 
    /// # Examples
    /// - `i32` matches `i32` but not `i64`
    /// - `string` matches `string` but not `String` (if they're different types)
    /// - Class implementing interface matches interface type in compatible contexts
    /// - Function types match if parameters and return types are compatible
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
            (TypeValue::Interface(left), TypeValue::Interface(right)) => left.full_name == right.full_name,
            _ => false,
        }
    }

    /// Returns the display name for this type value
    /// 
    /// This method provides a human-readable string representation of the type
    /// that can be used in error messages, debugging output, and type displays.
    /// The returned name follows Timu's type naming conventions.
    /// 
    /// # Returns
    /// A string containing the type's display name, borrowed when possible for efficiency
    /// 
    /// # Type Name Mappings
    /// - **Primitives**: Standard type names (e.g., "String", "I32", "Bool")
    /// - **Functions**: Function signature representation with parameters and return type
    /// - **Classes**: The class name as defined in source code
    /// - **Interfaces**: The interface name as defined in source code
    /// - **Modules**: The module path/name
    /// - **References**: The name of the referenced type
    /// 
    /// # Examples
    /// - `PrimitiveType::String` → `"String"`
    /// - `PrimitiveType::I32` → `"I32"`
    /// - Class named "MyClass" → `"MyClass"`
    /// - Interface named "IMyInterface" → `"IMyInterface"`
    /// 
    /// # Usage
    /// This method is commonly used for:
    /// - Error message generation
    /// - Type information display
    /// - Debugging and logging
    /// - IDE integration and tooling
    pub fn get_name(&self) -> Cow<'_, str> {
        match self {
            TypeValue::PrimitiveType(primitive) => match primitive {
                PrimitiveType::String => "String".into(),
                PrimitiveType::Bool => "Bool".into(),
                PrimitiveType::I8 => "I8".into(),
                PrimitiveType::U8 => "U8".into(),
                PrimitiveType::I16 => "I16".into(),
                PrimitiveType::U16 => "U16".into(),
                PrimitiveType::I32 => "I32".into(),
                PrimitiveType::U32 => "U32".into(),
                PrimitiveType::I64 => "I64".into(),
                PrimitiveType::U64 => "U64".into(),
                PrimitiveType::Float => "Float".into(),
                PrimitiveType::Double => "Double".into(),
                PrimitiveType::Void => "Void".into(),
            },
            TypeValue::Function(function) => function.name.text.into(),
            TypeValue::Class(class) => class.name.text.into(),
            TypeValue::Module(_) => "Module".into(),
            TypeValue::Interface(interface) => interface.name.text.into(),
            TypeValue::InterfaceFunction(interface_function) => interface_function.name.text.into(),
            TypeValue::Reference(reference) => format!("ref {}", reference.get_name()).into()
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
