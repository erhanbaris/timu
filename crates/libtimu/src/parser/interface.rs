//! Interface definition parsing for the Timu language.
//!
//! This module handles parsing of interface definitions, which define abstract contracts
//! that types must fulfill. Interfaces are a key component of Timu's type system,
//! enabling polymorphism, abstraction, and design by contract programming patterns.
//!
//! # Interface Syntax
//!
//! Timu interfaces support several forms of definition:
//!
//! ## Basic Interface
//! ```timu
//! interface Drawable {
//!     func draw(this): void;
//!     func area(this): f64;
//! }
//! ```
//!
//! ## Interface with Fields
//! ```timu
//! interface Shape {
//!     width: f64;
//!     height: f64;
//!     func area(this): f64;
//! }
//! ```
//!
//! ## Interface Inheritance
//! ```timu
//! interface ColoredShape : Shape, Drawable {
//!     color: string;
//!     func setColor(this, color: string): void;
//! }
//! ```
//!
//! # Interface Components
//!
//! ## Method Signatures
//! - **Abstract methods** - Function signatures without implementations
//! - **Parameters** - Typed parameters including optional `this` parameter
//! - **Return types** - All methods must specify return types
//! - **Termination** - Method signatures end with semicolons
//!
//! ## Fields
//! - **Abstract fields** - Field declarations that implementing types must provide
//! - **Type annotations** - All fields must have explicit type declarations
//! - **Access patterns** - Fields define the data contract for implementations
//!
//! ## Inheritance
//! - **Base interfaces** - Interfaces can extend one or more parent interfaces
//! - **Method inheritance** - Child interfaces inherit all parent method signatures
//! - **Field inheritance** - Child interfaces inherit all parent field requirements
//!
//! # Type System Integration
//!
//! Interfaces integrate with Timu's type system to provide:
//! - **Contract enforcement** - Types implementing interfaces must satisfy all requirements
//! - **Polymorphic dispatch** - Interface types enable runtime method dispatch
//! - **Type safety** - Interface contracts are verified at compile time
//! - **Composition** - Interfaces support multiple inheritance for flexible design

use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, map, opt, peek};
use nom::error::context;
use nom::multi::{many0, separated_list0};
use nom::{IResult, Parser, sequence::delimited};

use crate::ast::{AstIndex, FieldAst, FunctionArgumentAst, InterfaceDefinitionAst, InterfaceDefinitionFieldAst, InterfaceFunctionDefinitionAst, TypeNameAst};
use crate::{ast::FileStatementAst, nom_tools::{cleanup, NomSpan}};

use super::{expected_ident, TimuParserError};

impl InterfaceDefinitionAst<'_> {
    /// Parses a complete interface definition with inheritance and body
    /// 
    /// This function parses the full interface syntax including the interface keyword,
    /// name, optional inheritance specification, and body containing method signatures
    /// and field declarations. The parser ensures proper syntax validation and builds
    /// a complete AST representation for semantic analysis.
    /// 
    /// # Parsing Logic
    /// 1. Parse the `interface` keyword
    /// 2. Parse the interface name (identifier)
    /// 3. Optionally parse inheritance with `:` followed by comma-separated base interfaces
    /// 4. Parse the interface body within braces
    /// 5. Process method signatures and field declarations within the body
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, statement))` - Successfully parsed interface definition
    /// * `Err(error)` - Parse error with context information
    /// 
    /// # Syntax Examples
    /// ```timu
    /// interface Shape {
    ///     area: f64;
    ///     func draw(this): void;
    /// }
    /// 
    /// interface ColoredShape : Shape, Drawable {
    ///     color: string;
    ///     func setColor(this, color: string): void;
    /// }
    /// ```
    /// 
    /// # Errors
    /// Returns errors for:
    /// - Missing interface name
    /// - Missing opening brace
    /// - Missing closing brace
    /// - Invalid method signature or field syntax within interface body
    /// - Invalid inheritance specification
    /// 
    /// # Features
    /// - **Interface inheritance** - Supports extending multiple base interfaces
    /// - **Mixed content** - Handles both method signatures and field declarations
    /// - **Error recovery** - Provides meaningful error messages for common mistakes
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, FileStatementAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup(tag("interface")).parse(input)?;
        let (input, name) = expected_ident("Missing interface name", input)?;

        let (input, base_interfaces) = match cleanup(opt(char(':'))).parse(input)? {
            (input, Some(_)) => {
                let (input, base_interfaces) = context("Missing variable type", cut(separated_list0(tag(","), TypeNameAst::parse))).parse(input)?;
                (input, base_interfaces)
            }
            (input, None) => (input, vec![]),
        };

        let (input, _) = context("Interface's opening '{' missing", cut(peek(cleanup(char('{'))))).parse(input)?;
        let (input, fields) = delimited(
            char('{'),
            cleanup(many0(alt((
                InterfaceFunctionDefinitionAst::parse,
                FieldAst::parse_interface_field
            )))),
            context("Interface's closing '}' missing", cut(char('}'))),
        )
        .parse(input)?;
        let index = AstIndex(input.extra.indexer.fetch_add(1, std::sync::atomic::Ordering::Relaxed));

        Ok((
            input,
            FileStatementAst::Interface(InterfaceDefinitionAst {
                name: name.into(),
                fields,
                base_interfaces,
                index,
            }.into()),
        ))
    }
}

impl Display for InterfaceDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "interface {}", self.name.text)?;

        if !self.base_interfaces.is_empty() {
            write!(f, ": ")?;
            for (index, base_interface) in self.base_interfaces.iter().enumerate() {
                write!(f, "{base_interface}")?;
                if index < self.base_interfaces.len() - 1 {
                    write!(f, ", ")?;
                }
            }
        }

        write!(f, " {{")?;
        for field in self.fields.iter() {
            match field {
                InterfaceDefinitionFieldAst::Function(function) => {
                    write!(f, "{function}")?;
                }
                InterfaceDefinitionFieldAst::Field(field) => {
                    write!(f, "{field}")?;
                }
            }
        }
        write!(f, "}}")
    }
}

impl InterfaceFunctionDefinitionAst<'_> {
    /// Parses an interface method signature declaration
    /// 
    /// This function parses method signatures within interface definitions. Unlike
    /// regular function definitions, interface methods have no implementation body
    /// and end with a semicolon. These signatures define the contract that implementing
    /// types must fulfill.
    /// 
    /// # Parsing Logic
    /// 1. Parse the `func` keyword
    /// 2. Parse the method name (identifier)
    /// 3. Parse the parameter list within parentheses
    /// 4. Parse the return type after colon
    /// 5. Require terminating semicolon (no function body)
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, field))` - Successfully parsed interface method signature
    /// * `Err(error)` - Parse error with context information
    /// 
    /// # Syntax Examples
    /// ```timu
    /// func getName(this): string;
    /// func setName(this, name: string): void;
    /// func calculate(this, x: i32, y: i32): f64;
    /// ```
    /// 
    /// # Errors
    /// Returns errors for:
    /// - Missing method name
    /// - Missing opening parenthesis
    /// - Missing closing parenthesis
    /// - Missing colon before return type
    /// - Missing return type specification
    /// - Missing terminating semicolon
    /// - Invalid parameter syntax
    /// 
    /// # Interface Contracts
    /// - **Abstract signatures** - No implementation body, only type contracts
    /// - **Mandatory parameters** - All parameters must have explicit types
    /// - **Return type required** - All interface methods must specify return types
    /// - **Semicolon termination** - Interface methods end with `;` not `{}`
    pub fn parse(
        input: NomSpan<'_>,
    ) -> IResult<NomSpan<'_>, InterfaceDefinitionFieldAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup(tag("func")).parse(input)?;
        let (input, name) = expected_ident("Missing function name", input)?;
        let (input, _) = context("Missing '('", cut(peek(cleanup(char('('))))).parse(input)?;
        let (input, arguments) =
            map(delimited(char('('), cleanup(separated_list0(char(','), FunctionArgumentAst::parse)), context("Missing ')'", cut(char(')')))), |items| items)
                .parse(input)?;

        let (input, _) = context("Missing ':'", cleanup(opt(char(':')))).parse(input)?;
        let (input, return_type) = context("Missing function return type", cut(cleanup(cleanup(TypeNameAst::parse)))).parse(input)?;
        let (input, _) = cleanup(char(';')).parse(input)?;

        Ok((
            input,
            InterfaceDefinitionFieldAst::Function(InterfaceFunctionDefinitionAst {
                name: name.into(),
                arguments,
                return_type,
            }),
        ))
    }
}

impl Display for InterfaceFunctionDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "func {}(", self.name.text)?;
        for (index, arg) in self.arguments.iter().enumerate() {
            write!(f, "{arg}")?;
            if index < self.arguments.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "): {};", self.return_type)
    }
}
