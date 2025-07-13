//! Extension definition parsing for the Timu language.
//!
//! This module handles parsing of `extend` statements, which are Timu's mechanism for
//! adding functionality to existing types without modifying their original definitions.
//! Extensions enable a form of composition-based programming that provides many of the
//! benefits of inheritance while maintaining type safety and modularity.
//!
//! # Extension Syntax
//!
//! Timu extensions follow a specific syntax pattern:
//!
//! ## Basic Extension
//! ```timu
//! extend String : Drawable {
//!     func draw(this): void {
//!         // implementation
//!     }
//! }
//! ```
//!
//! ## Extension with Multiple Interfaces
//! ```timu
//! extend MyType : Interface1, Interface2, Interface3 {
//!     field1: string;
//!     
//!     func method1(this): void { }
//!     func method2(this, param: i32): string { }
//! }
//! ```
//!
//! ## Extension with Fields and Methods
//! ```timu
//! extend Point : Drawable, Serializable {
//!     color: string;
//!     visible: bool;
//!     
//!     func setColor(this, color: string): void { }
//!     func serialize(this): string { }
//! }
//! ```
//!
//! # Extension Components
//!
//! ## Target Type
//! - **Type name** - The existing type being extended
//! - **Qualification** - Can extend built-in types or user-defined types
//! - **Scope** - Extensions affect the type within the declaring module's scope
//!
//! ## Interface Implementation
//! - **Required interfaces** - Extensions must implement one or more interfaces
//! - **Contract fulfillment** - All interface methods and fields must be provided
//! - **Multiple inheritance** - Extensions can implement multiple interfaces simultaneously
//!
//! ## Extension Body
//! - **Additional fields** - Extensions can add new fields to the extended type
//! - **Method implementations** - Extensions provide concrete implementations for interface methods
//! - **Public visibility** - All extension members are implicitly public
//!
//! # Type System Integration
//!
//! Extensions integrate with Timu's type system to provide:
//! - **Interface satisfaction** - Extended types can be used wherever their implemented interfaces are expected
//! - **Type safety** - Extensions are verified at compile time for correctness
//! - **Method resolution** - Extension methods are resolved during type checking
//! - **Scoped application** - Extensions apply only within the scope where they're defined

use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, peek};
use nom::error::context;
use nom::multi::{many0, separated_list1};
use nom::{IResult, Parser, sequence::delimited};

use crate::ast::{ExtendDefinitionAst, ExtendDefinitionFieldAst, FieldAst, FunctionDefinitionAst, TypeNameAst};
use crate::{ast::FileStatementAst, nom_tools::{cleanup, NomSpan}};

use super::{expected_ident, TimuParserError};

impl ExtendDefinitionAst<'_> {
    /// Parses a complete extension definition with interface implementations
    /// 
    /// This function parses the full extension syntax including the extend keyword,
    /// target type name, interface specifications, and body containing method
    /// implementations and field additions. Extensions must implement at least
    /// one interface and provide all required methods and fields.
    /// 
    /// # Parsing Logic
    /// 1. Parse the `extend` keyword
    /// 2. Parse the target type name (the type being extended)
    /// 3. Parse `:` followed by comma-separated list of interfaces to implement
    /// 4. Parse the extension body within braces
    /// 5. Process method implementations and field additions within the body
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, statement))` - Successfully parsed extension definition
    /// * `Err(error)` - Parse error with context information
    /// 
    /// # Syntax Examples
    /// ```timu
    /// extend String : Drawable {
    ///     func draw(this): void { }
    /// }
    /// 
    /// extend Point : Drawable, Serializable {
    ///     color: string;
    ///     func draw(this): void { }
    ///     func serialize(this): string { }
    /// }
    /// ```
    /// 
    /// # Errors
    /// Returns errors for:
    /// - Missing target type name
    /// - Missing colon before interface list
    /// - Missing interface specifications
    /// - Missing opening brace
    /// - Missing closing brace
    /// - Invalid method or field syntax within extension body
    /// - Use of `pub` keyword (extension members are implicitly public)
    /// 
    /// # Extension Rules
    /// - **Required interfaces** - Must specify at least one interface to implement
    /// - **Complete implementation** - Must provide all interface methods and fields
    /// - **Implicit visibility** - All extension members are automatically public
    /// - **Scoped application** - Extensions apply only within the declaring module
    /// - **Type compatibility** - Extended type gains the interface types for polymorphism
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, FileStatementAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup(tag("extend")).parse(input)?;
        let (input, name) = expected_ident("Missing class name", input)?;
        let (input, _) = context("Missing ':'", cut(cleanup(char(':')))).parse(input)?;
        let (input, base_interfaces) = context("Missing interface type(s)", cut(separated_list1(tag(","), TypeNameAst::parse))).parse(input)?;

        let (input, _) = context("Extend's opening '{' missing", cut(peek(cleanup(char('{'))))).parse(input)?;
        let (input, fields) = delimited(
            char('{'),
            cleanup(many0(alt((
                FunctionDefinitionAst::parse_extend_function,
                FieldAst::parse_extend_field
            )))),
            context("Extend's closing '}' missing", cut(char('}'))),
        )
        .parse(input)?;
    
        let name = TypeNameAst {
            reference: false,
            nullable: false,
            names: vec![name.clone().into()],
            names_span: name.into()
        };

        Ok((
            input,
            FileStatementAst::Extend(ExtendDefinitionAst {
                name,
                fields,
                base_interfaces,
            }.into()),
        ))
    }
}

impl Display for ExtendDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "extend {}", self.name.names.first().unwrap().text)?;

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
                ExtendDefinitionFieldAst::Function(function) => {
                    write!(f, "{function}")?;
                }
                ExtendDefinitionFieldAst::Field(field) => {
                    write!(f, "{field}")?;
                }
            }
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use nom::Finish;
    use nom_language::error::VerboseErrorKind;
    use rstest::rstest;

    use crate::{file::SourceFile, nom_tools::State};


    #[rstest]
    #[case("extend Myclass: a {}", "extend Myclass: a {}")]
    #[case("    extend     Myclass : a,b   \r\n\t{} ", "extend Myclass: a, b {}")]
    #[case("    extend     Myclass: a    \r\n\t{\r\n\t} ", "extend Myclass: a {}")]
    #[case("extend ___MyType___: a {}", "extend ___MyType___: a {}")]
    #[case("extend Myclass: a { a: string; }", "extend Myclass: a {a: string;}")]
    #[case("extend Myclass: a { \r\n\ta\r\n\t: \r\n\tstring ;\r\n\t}", "extend Myclass: a {a: string;}")]
    #[case("extend Myclass: a { \r\n\t\r\n\t\r\n\t\r\n\t}", "extend Myclass: a {}")]
    #[case("extend Myclass: a { a: string; }", "extend Myclass: a {a: string;}")]
    #[case("extend Myclass: a { a: ?string; }", "extend Myclass: a {a: ?string;}")]
    #[case("extend Myclass: a { a: ?string; }", "extend Myclass: a {a: ?string;}")]
    #[case("extend Myclass: a { a: ?string.base; }", "extend Myclass: a {a: ?string.base;}")]
    #[case("extend Myclass: a { a: string; b: string; }", "extend Myclass: a {a: string;b: string;}")]
    #[case("extend Myclass: a { func init(): MyType {} }", "extend Myclass: a {func init(): MyType {}}")]
    #[case("extend Myclass: a { func init(): MyType {} func init(): MyType {} }", "extend Myclass: a {func init(): MyType {}func init(): MyType {}}")]
    #[case(
        "extend Myclass: a { a: ?string.base; func init(): MyType {} func init(): MyType {} }",
        "extend Myclass: a {a: ?string.base;func init(): MyType {}func init(): MyType {}}"
    )]
    fn extend_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let (_, response) = crate::parser::parse(&state).finish().unwrap();
        assert_eq!(response.statements[0].to_string(), expected, "{code}");
    }

    #[rstest]
    #[case("extend Myclass: a { pub a: string; }", "All extended fields already public")]
    #[case("extend Myclass: a { pub func init(): MyType {} }", "All extended functions already public")]
    fn alread_public<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let error = crate::parser::parse(&state).finish().unwrap_err();
        if let VerboseErrorKind::Context(ctx) = error.errors[0].1 {
            assert_eq!(ctx, expected, "{code}");
        } else {
            panic!("Expected an error, but got: {error:?}");
        }
    }
}
