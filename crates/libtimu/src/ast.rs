//! Abstract Syntax Tree (AST) Definitions for the Timu Language
//!
//! This module defines the core AST data structures that represent the parsed
//! structure of Timu source code. The AST serves as the bridge between the
//! parser and the Type Intermediate Representation (TIR) system.
//!
//! # Overview
//!
//! The AST captures the hierarchical structure of Timu programs through
//! various node types that represent different language constructs:
//!
//! - **File Structure**: [`FileAst`] contains top-level statements
//! - **Declarations**: Classes, interfaces, functions, and extensions
//! - **Expressions**: Operations, function calls, primitives, and identifiers
//! - **Statements**: Variable definitions, assignments, and control flow
//! - **Types**: Type references with nullable and reference annotations
//!
//! # Key Design Principles
//!
//! 1. **Lifetime Management**: All AST nodes use lifetime parameter `'base`
//!    to reference source code strings without copying
//! 2. **Source Location Tracking**: Every node preserves span information
//!    for accurate error reporting and IDE integration
//! 3. **Reference Counting**: Complex nodes use `Rc<T>` for shared ownership
//!    between different parts of the compilation pipeline
//! 4. **Type Safety**: Strong typing prevents invalid AST construction
//!
//! # Module Organization
//!
//! The AST nodes are organized by their role in the language:
//!
//! ## Top-level Declarations
//! - [`ClassDefinitionAst`]: Class definitions with fields and methods
//! - [`InterfaceDefinitionAst`]: Interface contracts with method signatures
//! - [`FunctionDefinitionAst`]: Standalone function definitions
//! - [`ExtendDefinitionAst`]: Extensions that add functionality to existing classes
//! - [`UseAst`]: Import statements for cross-module dependencies
//!
//! ## Expressions and Operations
//! - [`ExpressionAst`]: All expression types including operations and calls
//! - [`PrimitiveValue`]: Literal values (strings, numbers, booleans)
//! - [`FunctionCallAst`]: Function invocations with arguments
//! - [`RefAst`]: Reference expressions for accessing nested properties
//!
//! ## Type System
//! - [`TypeNameAst`]: Type references with modifiers (nullable, reference)
//! - [`FunctionArgumentAst`]: Function parameter definitions
//!
//! ## Control Flow and Statements
//! - [`BodyAst`]: Statement blocks
//! - [`IfConditionAst`]: Conditional statements with else-if chains
//! - [`VariableDefinitionAst`]: Variable declarations
//! - [`VariableAssignAst`]: Variable assignments
//!
//! # Usage Example
//!
//! The AST is typically created by the parser and consumed by the TIR builder:
//!
//! ```ignore
//! use libtimu::{process_code, process_ast};
//! 
//! // Parser creates AST from source code
//! let ast = process_code(&state)?;
//! 
//! // TIR builder consumes AST to create type-checked representation
//! let tir_context = process_ast(vec![ast.into()])?;
//! ```

use std::{borrow::Cow, rc::Rc};
use strum_macros::EnumIs;
use crate::{
    file::SourceFile,
    nom_tools::{Span, ToRange},
    parser::splited_path::SplitedPath, tir::PrimitiveType,
};

/// Unique identifier for AST nodes within a compilation unit.
///
/// Used to distinguish between different AST nodes during compilation,
/// particularly useful for debugging and error reporting.
#[derive(PartialEq, Debug, Copy, Clone, Eq, PartialOrd, Ord, Hash)]
pub struct AstIndex(pub usize);

/// Represents literal values in Timu source code.
///
/// This enum captures all primitive types that can appear as literals
/// in Timu programs. The values are parsed from source code and stored
/// with their original representation to preserve precision and formatting.
///
/// # Numeric Types
///
/// Integer literals are automatically sized based on their value range:
/// - Values fitting in `i8` range become `I8`
/// - Values fitting in `u8` range become `U8` 
/// - And so on up to `I64`/`U64`
///
/// Floating-point literals are distinguished by precision:
/// - `Float`: Single precision (fits in f32 range)
/// - `Double`: Double precision (requires f64)
///
/// The `u8` parameter in `Float` and `Double` variants stores the number
/// of decimal places from the original source for accurate display.
///
/// # String Literals
///
/// String values use `Cow<'base, str>` to avoid unnecessary allocations
/// when the parsed string matches the source exactly.
///
/// # Examples
///
/// ```ignore
/// // These source literals become:
/// "hello"     -> PrimitiveValue::String("hello")
/// true        -> PrimitiveValue::Bool(true)
/// 42          -> PrimitiveValue::I8(42)  // or I16/I32/I64 based on size
/// 3.14        -> PrimitiveValue::Float(3.14, 2)  // 2 decimal places
/// 255         -> PrimitiveValue::U8(255)
/// ```
#[derive(PartialEq, Debug, Clone)]
pub enum PrimitiveValue<'base> {
    /// String literal value with lifetime tied to source code
    String(Cow<'base, str>),
    /// Boolean literal (true/false)
    Bool(bool),
    /// 8-bit signed integer literal
    I8(i8),
    /// 8-bit unsigned integer literal
    U8(u8),
    /// 16-bit signed integer literal
    I16(i16),
    /// 16-bit unsigned integer literal
    U16(u16),
    /// 32-bit signed integer literal
    I32(i32),
    /// 32-bit unsigned integer literal
    U32(u32),
    /// 64-bit signed integer literal
    I64(i64),
    /// 64-bit unsigned integer literal
    U64(u64),
    /// Single-precision floating-point literal (fits in f32 range) with decimal place count
    /// Note: Stored as f64 but represents values that fit within f32 precision
    Float(f64, u8),
    /// Double-precision floating-point literal (requires f64 precision) with decimal place count
    Double(f64, u8),
}

impl PrimitiveValue<'_> {
    /// Converts the primitive value to its corresponding type information.
    ///
    /// This method extracts the type from a literal value for use in the
    /// type checking system. The returned [`PrimitiveType`] is used by the
    /// TIR system to perform type analysis and validation.
    ///
    /// # Returns
    ///
    /// The [`PrimitiveType`] corresponding to this literal value.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let string_literal = PrimitiveValue::String("hello".into());
    /// assert_eq!(string_literal.to_type(), PrimitiveType::String);
    ///
    /// let int_literal = PrimitiveValue::I32(42);
    /// assert_eq!(int_literal.to_type(), PrimitiveType::I32);
    /// ```
    pub fn to_type(&self) -> PrimitiveType {
       match self {
            PrimitiveValue::String(_) => PrimitiveType::String,
            PrimitiveValue::Bool(_) => PrimitiveType::Bool,
            PrimitiveValue::I8(_) => PrimitiveType::I8,
            PrimitiveValue::U8(_) => PrimitiveType::U8,
            PrimitiveValue::I16(_) => PrimitiveType::I16,
            PrimitiveValue::U16(_) => PrimitiveType::U16,
            PrimitiveValue::I32(_) => PrimitiveType::I32,
            PrimitiveValue::U32(_) => PrimitiveType::U32,
            PrimitiveValue::I64(_) => PrimitiveType::I64,
            PrimitiveValue::U64(_) => PrimitiveType::U64,
            PrimitiveValue::Float(_, _) => PrimitiveType::Float,
            PrimitiveValue::Double(_, _) => PrimitiveType::Double,
       }
    }
}

impl<'base> AsRef<PrimitiveValue<'base>> for PrimitiveValue<'base> {
    fn as_ref(&self) -> &PrimitiveValue<'base> {
        self
    }
}

impl<'base> AsMut<PrimitiveValue<'base>> for PrimitiveValue<'base> {
    fn as_mut(&mut self) -> &mut PrimitiveValue<'base> {
        self
    }
}

/// Specifies the mutability of variable definitions.
///
/// In Timu, variables can be declared as either mutable or immutable,
/// affecting how they can be used throughout their lifetime.
#[derive(PartialEq, Debug)]
pub enum VariableDefinitionType {
    /// Immutable variable - value cannot be changed after initialization
    Const,
    /// Mutable variable - value can be reassigned
    Var,
}

/// Binary operators available in Timu expressions.
///
/// These operators define the operations that can be performed between
/// two expressions. They are ordered by precedence groups, with higher
/// precedence operators binding more tightly.
///
/// # Precedence Groups (highest to lowest)
///
/// 1. **Multiplicative**: `*`, `/`, `%`
/// 2. **Additive**: `+`, `-`
/// 3. **Bitwise Shift**: `<<`, `>>`
/// 4. **Relational**: `<`, `<=`, `>`, `>=`
/// 5. **Equality**: `==`, `!=`
/// 6. **Bitwise AND**: `&`
/// 7. **Bitwise XOR**: `^`
/// 8. **Bitwise OR**: `|`
/// 9. **Logical AND**: `&&`
/// 10. **Logical OR**: `||`
///
/// # Operator Categories
///
/// - **Arithmetic**: `Add`, `Sub`, `Mul`, `Div`, `Mod`
/// - **Comparison**: `Equal`, `NotEqual`, `LessThan`, `GreaterThan`, etc.
/// - **Logical**: `And` (`&&`), `Or` (`||`)
/// - **Bitwise**: `LogicalAnd` (`&`), `LogicalOr` (`|`), `Xor`, shifts
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ExpressionOperatorType {
    /// Addition operator (`+`)
    Add,
    /// Subtraction operator (`-`)
    Sub,
    /// Multiplication operator (`*`)
    Mul,
    /// Division operator (`/`)
    Div,
    /// Modulo operator (`%`)
    Mod,
    /// Logical AND operator (`&&`)
    And,
    /// Logical OR operator (`||`)
    Or,
    /// Equality operator (`==`)
    Equal,
    /// Inequality operator (`!=`)
    NotEqual,
    /// Greater than or equal operator (`>=`)
    GreaterEqualThan,
    /// Greater than operator (`>`)
    GreaterThan,
    /// Less than or equal operator (`<=`)
    LessEqualThan,
    /// Less than operator (`<`)
    LessThan,
    /// Bitwise XOR operator (`^`)
    Xor,
    /// Bitwise OR operator (`|`)
    LogicalOr,
    /// Bitwise AND operator (`&`)
    LogicalAnd,
    /// Left bit shift operator (`<<`)
    BitwiseShiftLeft,
    /// Right bit shift operator (`>>`)
    BitwiseShiftRight,
}

/// Root AST node representing a complete Timu source file.
///
/// This is the top-level container for all parsed content from a single
/// source file. It includes metadata about the source file and all
/// top-level statements parsed from that file.
///
/// # Structure
///
/// A file AST contains:
/// - Source file metadata (path, content)
/// - A sequence of top-level statements (classes, functions, interfaces, etc.)
///
/// # Usage
///
/// The `FileAst` is created by the parser from source code and later
/// consumed by the TIR builder to create the type-checked representation.
///
/// ```ignore
/// // Created by parser
/// let file_ast = process_code(&state)?;
/// 
/// // Used by TIR builder
/// let tir = process_ast(vec![file_ast.into()])?;
/// ```
#[derive(Debug)]
pub struct FileAst<'base> {
    /// Source file information including path and content
    pub file: SourceFile,
    /// All top-level statements in the file
    pub statements: Vec<FileStatementAst<'base>>,
}

/// Top-level statements that can appear in a Timu source file.
///
/// These represent the major language constructs that can be declared
/// at the file level. Each variant wraps the actual AST node in an `Rc`
/// for shared ownership during compilation.
///
/// # Statement Types
///
/// - **Class**: Class definitions with fields and methods
/// - **Function**: Standalone function definitions  
/// - **Interface**: Interface contracts defining method signatures
/// - **Extend**: Extensions that add functionality to existing classes
/// - **Use**: Import statements for cross-module dependencies
///
/// # Design Notes
///
/// The `EnumIs` derive macro automatically generates convenience methods
/// like `is_class()`, `is_function()`, etc. for pattern matching.
#[derive(EnumIs, Debug)]
pub enum FileStatementAst<'base> {
    /// Class definition statement
    Class(Rc<ClassDefinitionAst<'base>>),
    /// Function definition statement
    Function(Rc<FunctionDefinitionAst<'base>>),
    /// Interface definition statement
    Interface(Rc<InterfaceDefinitionAst<'base>>),
    /// Extend definition statement
    Extend(Rc<ExtendDefinitionAst<'base>>),
    /// Use/import statement
    Use(Rc<UseAst<'base>>),
}

/// Import statement for bringing external modules into scope.
///
/// Use statements allow Timu code to reference types and functions from
/// other modules. They support both direct imports and aliased imports
/// for namespace management.
///
/// # Syntax Examples
///
/// ```timu
/// use module.SomeClass;           // Direct import
/// use module.SomeClass as Alias;  // Aliased import  
/// use module;                     // Module import
/// ```
///
/// # Fields
///
/// - `import`: The module path being imported
/// - `alias`: Optional alias name for the imported item
#[derive(Debug)]
pub struct UseAst<'base> {
    /// Optional alias for the imported item
    pub alias: Option<Span<'base>>,
    /// Path to the module or item being imported
    pub import: SplitedPath<'base>,
}

impl<'base> UseAst<'base> {
    /// Returns the name that will be used to reference the imported item.
    ///
    /// This is either the alias (if provided) or the last component of
    /// the import path. This name is what becomes available in the local scope.
    ///
    /// # Returns
    ///
    /// The [`Span`] containing the effective name for this import.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // For "use module.SomeClass as Alias"
    /// // Returns span containing "SomeClass"
    /// let name = use_ast.ast_name();
    /// ```
    pub fn ast_name(&self) -> Span<'base> {
        self.import.paths.last().unwrap().clone()
    }
}

/// Class definition AST node.
///
/// Represents a class declaration in Timu source code. Classes can contain
/// both fields (data members) and methods (functions). They serve as the
/// primary unit of object-oriented programming in Timu.
///
/// # Syntax Example
///
/// ```timu
/// class Person {
///     name: string;
///     age: i32;
///     
///     func getName(): string {
///         return this.name;
///     }
/// }
/// ```
///
/// # Fields
///
/// - `name`: The class name identifier
/// - `fields`: All members (fields and methods) of the class
/// - `index`: Unique identifier for this class within the compilation unit
#[derive(Debug, PartialEq)]
pub struct ClassDefinitionAst<'base> {
    /// Class name identifier
    pub name: Span<'base>,
    /// All class members (fields and methods)
    pub fields: Vec<ClassDefinitionFieldAst<'base>>,
    /// Unique index for this class
    pub index: AstIndex,
}

/// Interface definition AST node.
///
/// Represents an interface declaration that defines a contract for classes
/// to implement. Interfaces can contain method signatures and field
/// declarations, and support inheritance from other interfaces.
///
/// # Syntax Example
///
/// ```timu
/// interface Drawable {
///     func draw(): void;
///     visible: bool;
/// }
/// 
/// interface Shape: Drawable {
///     func area(): f64;
/// }
/// ```
///
/// # Fields
///
/// - `name`: The interface name identifier
/// - `fields`: Method signatures and field declarations
/// - `base_interfaces`: Parent interfaces this interface extends
/// - `index`: Unique identifier for this interface
#[derive(Debug, PartialEq)]
pub struct InterfaceDefinitionAst<'base> {
    /// Interface name identifier
    pub name: Span<'base>,
    /// Interface members (method signatures and fields)
    pub fields: Vec<InterfaceDefinitionFieldAst<'base>>,
    /// Parent interfaces this interface extends
    pub base_interfaces: Vec<TypeNameAst<'base>>,
    /// Unique index for this interface
    pub index: AstIndex,
}

/// Members that can appear within an interface definition.
///
/// Interfaces can contain method signatures (without implementations)
/// and field declarations that implementing classes must provide.
#[derive(Debug, PartialEq)]
pub enum InterfaceDefinitionFieldAst<'base> {
    /// Method signature without implementation
    Function(InterfaceFunctionDefinitionAst<'base>),
    /// Field declaration
    Field(FieldAst<'base>),
}

/// Extension definition AST node.
///
/// Represents an extension that adds functionality to an existing class.
/// Extensions allow implementing interfaces on existing classes and
/// adding new methods without modifying the original class definition.
///
/// # Syntax Example
///
/// ```timu
/// extend Person: Drawable {
///     func draw(): void {
///         // Implementation
///     }
/// }
/// ```
///
/// # Fields
///
/// - `name`: The class being extended
/// - `fields`: New methods and fields added by this extension
/// - `base_interfaces`: Interfaces implemented by this extension
#[derive(Debug, PartialEq)]
pub struct ExtendDefinitionAst<'base> {
    /// The class being extended
    pub name: TypeNameAst<'base>,
    /// New members added by this extension
    pub fields: Vec<ExtendDefinitionFieldAst<'base>>,
    /// Interfaces implemented by this extension
    pub base_interfaces: Vec<TypeNameAst<'base>>,
}

/// Members that can be added by an extension definition.
///
/// Extensions can add both new methods (with implementations) and
/// new fields to existing classes.
#[derive(Debug, PartialEq)]
pub enum ExtendDefinitionFieldAst<'base> {
    /// New method implementation
    Function(FunctionDefinitionAst<'base>),
    /// New field declaration
    Field(FieldAst<'base>),
}

/// Type reference AST node with modifiers.
///
/// Represents a type reference in Timu source code, including support
/// for type modifiers (nullable, reference) and qualified type names
/// with module paths.
///
/// # Type Modifiers
///
/// - **Nullable**: Types can be marked as nullable with `?` suffix
/// - **Reference**: Types can be references with `&` prefix
///
/// # Syntax Examples
///
/// ```timu
/// string          // Simple type
/// string?         // Nullable type
/// &string         // Reference type
/// &string?        // Nullable reference type
/// module.Class    // Qualified type name
/// &module.Class?  // Complex qualified nullable reference
/// ```
///
/// # Fields
///
/// - `reference`: Whether this is a reference type (`&`)
/// - `nullable`: Whether this type can be null (`?`)
/// - `names`: Path components (e.g., ["module", "Class"])
/// - `names_span`: Source span covering the entire type reference
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeNameAst<'base> {
    /// Whether this is a reference type (prefixed with &)
    pub reference: bool,
    /// Whether this type is nullable (suffixed with ?)
    pub nullable: bool,
    /// Type name components (for qualified names like module.Class)
    pub names: Vec<Span<'base>>,
    /// Source span covering the entire type reference
    pub names_span: Span<'base>
}

impl ToRange for TypeNameAst<'_> {
    fn to_range(&self) -> std::ops::Range<usize> {
        self.names_span.position.clone()
    }
}

/// Reference expression for accessing nested properties.
///
/// Represents property access chains like `obj.field.subfield` or
/// `module.function`. Used in expressions to reference variables,
/// fields, or other identifiers through a path.
///
/// # Syntax Examples
///
/// ```timu
/// this.name           // Access field on current object
/// obj.field.value     // Nested property access
/// module.function     // Module-qualified access
/// ```
#[derive(Debug, PartialEq)]
pub struct RefAst<'base> {
    /// Path components for the reference (e.g., ["obj", "field", "value"])
    pub names: Vec<Span<'base>>,
}

/// Function parameter definition.
///
/// Represents parameters in function signatures. Timu supports both
/// regular typed parameters and the special `this` parameter for
/// method definitions.
///
/// # Parameter Types
///
/// - **This**: The special `this` parameter in method definitions
/// - **Argument**: Regular typed parameters with name and type
///
/// # Syntax Examples
///
/// ```timu
/// func method(this, param: string): void { }    // Method with this
/// func function(param: string): void { }        // Regular function
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionArgumentAst<'base> {
    /// The special 'this' parameter for methods
    This(Span<'base>),
    /// Regular parameter with name and type
    Argument {
        /// Parameter name
        name: Span<'base>,
        /// Parameter type
        field_type: TypeNameAst<'base>
    }
}

/// Statements that can appear within function bodies.
///
/// Represents the various statement types that make up the body
/// of functions and control flow blocks. Each statement performs
/// an action or controls program flow.
///
/// # Statement Types
///
/// - **Variable Definition**: `var x: type = value;`
/// - **Variable Assignment**: `x = value;`
/// - **Function Call**: `function(args);`
/// - **If Condition**: `if (condition) { ... }`
#[derive(Debug, PartialEq)]
pub enum BodyStatementAst<'base> {
    /// Variable declaration with optional initialization
    VariableDefinition(VariableDefinitionAst<'base>),
    /// Assignment to existing variable
    VariableAssign(VariableAssignAst<'base>),
    /// Function call statement
    FunctionCall(FunctionCallAst<'base>),
    /// Conditional statement (if/else)
    IfCondition(IfConditionAst<'base>),
}

/// Statement block containing a sequence of statements.
///
/// Represents a block of statements that execute sequentially.
/// Used for function bodies, conditional blocks, and other
/// grouped statement sequences.
///
/// # Usage
///
/// Bodies appear in:
/// - Function implementations
/// - If/else blocks
/// - Loop bodies (when added to the language)
#[derive(Debug, PartialEq)]
pub struct BodyAst<'base> {
    /// Sequence of statements in this block
    pub statements: Vec<BodyStatementAst<'base>>,
}

/// Specifies where a function is defined.
///
/// Functions in Timu can be defined in different contexts, which affects
/// their scope and access patterns. This enum tracks the location context
/// for proper resolution and type checking.
#[derive(Debug, PartialEq)]
pub enum FunctionDefinitionLocationAst<'base> {
    /// Function defined within a class (method)
    Class(Span<'base>),
    /// Function defined at module level (standalone function)
    #[allow(dead_code)]
    Module,
}

/// Function definition AST node.
///
/// Represents a complete function definition including signature and
/// implementation. Functions can be standalone (module-level) or
/// methods (class-level) with different visibility modifiers.
///
/// # Syntax Example
///
/// ```timu
/// public func calculate(this, x: i32, y: i32): i32 {
///     return x + y;
/// }
/// ```
///
/// # Fields
///
/// - `is_public`: Optional public visibility modifier
/// - `name`: Function name identifier
/// - `arguments`: Parameter list including optional `this`
/// - `arguments_span`: Source span covering the parameter list
/// - `return_type`: Function return type
/// - `body`: Function implementation statements
/// - `location`: Context where function is defined (class/module)
/// - `index`: Unique identifier for this function
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinitionAst<'base> {
    /// Optional public visibility modifier
    pub is_public: Option<Span<'base>>,
    /// Function name identifier
    pub name: Span<'base>,
    /// Function parameters
    pub arguments: Vec<FunctionArgumentAst<'base>>,
    /// Source span covering the entire parameter list
    pub arguments_span: Span<'base>,
    /// Function return type
    pub return_type: TypeNameAst<'base>,
    /// Function body implementation
    pub body: Rc<BodyAst<'base>>,
    /// Definition context (class or module)
    pub location: Rc<FunctionDefinitionLocationAst<'base>>,
    /// Unique index for this function
    pub index: AstIndex,
}

/// Type of function call path.
///
/// Distinguishes between calls on the current object (`this`) and
/// direct calls to functions or methods on other objects/modules.
///
/// # Call Types
///
/// - **This**: Calls on current object (e.g., `this.method()`)
/// - **Direct**: Direct calls (e.g., `obj.method()`, `module.function()`)
#[derive(Debug, PartialEq)]
pub enum FunctionCallType<'base> {
    /// Call on current object (this.method)
    This(Vec<Span<'base>>),
    /// Direct call (obj.method, module.function)
    Direct(Vec<Span<'base>>),
}

/// Function call expression AST node.
///
/// Represents a function or method invocation with arguments.
/// Includes both the call path (how the function is referenced)
/// and the arguments passed to it.
///
/// # Syntax Examples
///
/// ```timu
/// function(arg1, arg2)        // Direct function call
/// obj.method(arg)             // Method call on object
/// this.method()               // Method call on current object
/// module.function(arg)        // Module-qualified function call
/// ```
///
/// # Fields
///
/// - `call_span`: Source span of the function name
/// - `arguments_span`: Source span covering the argument list
/// - `path`: How the function is referenced (this/direct)
/// - `arguments`: Expression arguments passed to the function
#[derive(Debug, PartialEq)]
pub struct FunctionCallAst<'base> {
    /// Source span of the function name being called
    pub call_span: Span<'base>,
    /// Source span covering the argument list
    pub arguments_span: Span<'base>,
    /// Function call path (this or direct)
    pub path: FunctionCallType<'base>,
    /// Arguments passed to the function
    pub arguments: Vec<ExpressionAst<'base>>,
}

impl FunctionCallType<'_> {
    /// Returns true if this is a call on the current object (`this`).
    pub fn is_this(&self) -> bool {
        matches!(self, FunctionCallType::This(_))
    }

    /// Returns true if this is a direct call (not on `this`).
    pub fn is_direct(&self) -> bool {
        matches!(self, FunctionCallType::Direct(_))
    }

    /// Converts the call path to a string representation.
    ///
    /// For `This` calls, prefixes with "this.". For `Direct` calls,
    /// joins the path components with dots.
    ///
    /// # Returns
    ///
    /// String representation of the call path.
    ///
    /// # Examples
    ///
    /// ```text
    /// This(["method"]) -> "this.method"
    /// Direct(["obj", "method"]) -> "obj.method"
    /// ```
    pub fn call(&self) -> String {
        match self {
            FunctionCallType::This(path) => format!("this.{}", path.iter().map(|p| p.text).collect::<Vec<_>>().join(".")),
            FunctionCallType::Direct(path) => path.iter().map(|p| p.text).collect::<Vec<_>>().join("."),
        }
    }

    /// Returns the path components for this function call.
    ///
    /// # Returns
    ///
    /// Reference to the vector of path components (spans).
    pub fn get_path(&self) -> &Vec<Span<'_>> {
        match self {
            FunctionCallType::This(path) => path,
            FunctionCallType::Direct(path) => path,
        }
    }
}

/// Function call path component.
///
/// Represents different ways to reference a function in a call path.
/// Currently supports simple identifiers and qualified type names.
///
/// # Usage
///
/// This enum is used internally by the parser to handle different
/// forms of function references during parsing.
#[derive(Debug)]
pub enum FunctionCallPathAst<'base> {
    /// Simple identifier reference
    Ident(Span<'base>),
    /// Qualified type name reference
    TypeName(TypeNameAst<'base>),
}

/// Interface method signature without implementation.
///
/// Represents a method signature declared in an interface.
/// Unlike regular function definitions, these only specify
/// the signature without providing an implementation.
///
/// # Syntax Example
///
/// ```timu
/// interface Drawable {
///     func draw(this, canvas: Canvas): void;  // Method signature only
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct InterfaceFunctionDefinitionAst<'base> {
    /// Method name identifier
    pub name: Span<'base>,
    /// Method parameters
    pub arguments: Vec<FunctionArgumentAst<'base>>,
    /// Method return type
    pub return_type: TypeNameAst<'base>,
}

/// Members that can appear within a class definition.
///
/// Classes can contain both data fields and method implementations.
/// This enum distinguishes between the two types of class members.
#[derive(Debug, PartialEq)]
pub enum ClassDefinitionFieldAst<'base> {
    /// Data field declaration
    Field(FieldAst<'base>),
    /// Method implementation
    Function(FunctionDefinitionAst<'base>),
}

/// Field declaration AST node.
///
/// Represents a data field in a class or interface. Fields have
/// a name, type, and optional visibility modifier.
///
/// # Syntax Examples
///
/// ```timu
/// name: string;           // Private field
/// public age: i32;        // Public field
/// data: &SomeType?;       // Complex type with modifiers
/// ```
///
/// # Fields
///
/// - `is_public`: Optional public visibility modifier
/// - `name`: Field name identifier
/// - `field_type`: Field type with optional modifiers
#[derive(Debug, PartialEq)]
pub struct FieldAst<'base> {
    /// Optional public visibility modifier
    pub is_public: Option<Span<'base>>,
    /// Field name identifier
    pub name: Span<'base>,
    /// Field type
    pub field_type: TypeNameAst<'base>,
}

/// Expression AST node representing all expression types.
///
/// Expressions are the building blocks of computations in Timu.
/// They can represent literal values, variable references, function calls,
/// and complex operations with proper precedence handling.
///
/// # Expression Types
///
/// - **Primitive**: Literal values (strings, numbers, booleans)
/// - **Ref**: Property access chains (`obj.field.subfield`)
/// - **Not**: Logical negation (`!expression`)
/// - **Ident**: Simple identifiers (`variable`)
/// - **FunctionCall**: Function/method invocations
/// - **Operation**: Binary operations with left/right operands
///
/// # Syntax Examples
///
/// ```timu
/// 42                      // Primitive
/// "hello"                 // Primitive string
/// variable                // Ident
/// obj.field               // Ref
/// !condition              // Not
/// func(arg1, arg2)        // FunctionCall
/// a + b * c               // Operation (with precedence)
/// ```
#[derive(Debug, PartialEq)]
pub enum ExpressionAst<'base> {
    /// Literal primitive value
    Primitive { 
        /// Source span of the literal
        span: Span<'base>,
        /// The literal value
        value: PrimitiveValue<'base>
    },
    /// Property access chain
    Ref(RefAst<'base>),
    /// Logical negation expression
    Not(Box<ExpressionAst<'base>>),
    /// Simple identifier reference
    Ident(Span<'base>),
    /// Function or method call
    FunctionCall(FunctionCallAst<'base>),
    /// Binary operation
    Operation { 
        /// Left operand
        left: Box<ExpressionAst<'base>>, 
        /// Binary operator
        operator: ExpressionOperatorType, 
        /// Right operand
        right: Box<ExpressionAst<'base>> 
    },
}

/// Conditional statement AST node (if/else if/else).
///
/// Represents conditional control flow with support for multiple
/// else-if branches and an optional else clause.
///
/// # Syntax Example
///
/// ```timu
/// if (condition1) {
///     // true_body
/// } else if (condition2) {
///     // else_if body
/// } else {
///     // false_body
/// }
/// ```
///
/// # Fields
///
/// - `expression`: Main condition expression
/// - `true_body`: Statements executed when condition is true
/// - `else_ifs`: Additional condition/body pairs for else-if branches
/// - `false_body`: Optional else clause statements
#[derive(Debug, PartialEq)]
pub struct IfConditionAst<'base> {
    /// Main condition expression
    pub expression: ExpressionAst<'base>,
    /// Statements executed when condition is true
    pub true_body: BodyAst<'base>,
    /// Else-if branches with their conditions and bodies
    pub else_ifs: Vec<(ExpressionAst<'base>, BodyAst<'base>)>,
    /// Optional else clause statements
    pub false_body: Option<BodyAst<'base>>,
}

/// Variable definition statement AST node.
///
/// Represents variable declarations with optional type annotations
/// and initialization expressions. Supports both mutable (`var`)
/// and immutable (`const`) variable declarations.
///
/// # Syntax Examples
///
/// ```timu
/// var x: i32 = 42;        // Mutable with explicit type and initializer
/// const name = "hello";   // Immutable with inferred type
/// var count: i32;         // Mutable with explicit type, no initializer
/// ```
///
/// # Fields
///
/// - `variable_definition_type`: Mutability (var/const)
/// - `name`: Variable name identifier
/// - `expected_type`: Optional explicit type annotation
/// - `expression`: Optional initialization expression
#[derive(Debug, PartialEq)]
pub struct VariableDefinitionAst<'base> {
    /// Variable mutability (var or const)
    pub variable_definition_type: VariableDefinitionType,
    /// Variable name identifier
    pub name: Span<'base>,
    /// Optional explicit type annotation
    pub expected_type: Option<TypeNameAst<'base>>,
    /// Optional initialization expression
    pub expression: Option<ExpressionAst<'base>>,
}

/// Variable assignment statement AST node.
///
/// Represents assignment of a new value to an existing variable.
/// The variable must have been previously declared and must be
/// mutable (declared with `var`, not `const`).
///
/// # Syntax Example
///
/// ```timu
/// variable = newValue;
/// obj.field = expression;
/// ```
///
/// # Fields
///
/// - `name`: Variable name being assigned to
/// - `expression`: New value expression
#[derive(Debug, PartialEq)]
pub struct VariableAssignAst<'base> {
    /// Variable name being assigned to
    pub name: Span<'base>,
    /// New value expression
    pub expression: ExpressionAst<'base>,
}
