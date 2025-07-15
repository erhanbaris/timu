//! Class definition parsing for the Timu language.
//!
//! This module handles parsing of class definitions, which are fundamental object-oriented
//! constructs in Timu. Classes can contain fields (data members) and methods (functions)
//! and support object instantiation and method invocation.
//!
//! # Class Syntax
//!
//! ```timu
//! class ClassName {
//!     // Fields
//!     field1: Type1;
//!     field2: Type2;
//!     
//!     // Methods
//!     func method1(this): ReturnType {
//!         // method body
//!     }
//!     
//!     func method2(this, param: ParamType): ReturnType {
//!         // method body
//!     }
//! }
//! ```
//!
//! # Class Components
//!
//! ## Fields
//! - **Instance variables**: Data members that belong to each class instance
//! - **Type annotations**: All fields must have explicit type declarations
//! - **Initialization**: Fields can be initialized during object construction
//!
//! ## Methods
//! - **Instance methods**: Functions that operate on class instances
//! - **This parameter**: Methods can access the current instance via `this`
//! - **Parameters**: Methods can accept additional typed parameters
//! - **Return types**: All methods must specify return types
//!
//! # Object-Oriented Features
//!
//! Classes in Timu support:
//! - **Encapsulation**: Data and methods bundled together
//! - **Instantiation**: Creating objects from class definitions
//! - **Method calls**: Invoking methods on object instances
//! - **Field access**: Reading and writing object field values

use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, peek};
use nom::error::context;
use nom::multi::many0;
use nom::{IResult, Parser, sequence::delimited};

use crate::ast::{AstIndex, ClassDefinitionFieldAst, FieldAst, FunctionDefinitionAst};
use crate::{ast::{ClassDefinitionAst, FileStatementAst}, nom_tools::{cleanup, NomSpan}};

use super::{expected_ident, is_public, TimuParserError};

impl ClassDefinitionAst<'_> {
    /// Parses a complete class definition
    /// 
    /// This function parses a class definition including the class keyword, name,
    /// and body containing fields and methods. The parser ensures proper syntax
    /// and builds a complete AST representation for semantic analysis.
    /// 
    /// # Syntax
    /// ```timu
    /// class ClassName {
    ///     field1: Type1;
    ///     field2: Type2;
    ///     func method(this, param: Type): ReturnType { }
    /// }
    /// ```
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, statement))` - Successfully parsed class definition
    /// * `Err(error)` - Parse error with context information
    /// 
    /// # Errors
    /// Returns errors for:
    /// - Missing class name
    /// - Missing opening brace
    /// - Missing closing brace
    /// - Invalid field or method syntax within class body
    /// 
    /// # Features
    /// - **Field parsing**: Handles typed field declarations
    /// - **Method parsing**: Handles function definitions with `this` parameter
    /// - **Mixed content**: Supports any combination of fields and methods
    /// - **Error recovery**: Provides meaningful error messages for common mistakes
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, FileStatementAst<'_>, TimuParserError<'_>> {
        let (input, is_public) = is_public(input)?;
        let (input, _) = cleanup(tag("class")).parse(input)?;
        let (input, name) = expected_ident("Missing class name", input)?;
        let (input, _) = context("Class's opening '{' missing", cut(peek(cleanup(char('{'))))).parse(input)?;
        let (input, fields) = delimited(
            char('{'),
            cleanup(many0(alt((
                |input| {
                    FunctionDefinitionAst::parse_class_function(input, name.clone())
                },
                FieldAst::parse_class_field,
            )))),
            context("Class's closing '}' missing", cut(char('}'))),
        )
        .parse(input)?;
        let index = AstIndex(input.extra.indexer.fetch_add(1, std::sync::atomic::Ordering::Relaxed));

        Ok((
            input,
            FileStatementAst::Class(ClassDefinitionAst {
                is_public: is_public.map(|item| item.into()),
                name: name.into(),
                fields,
                index
            }.into()),
        ))
    }
}

impl Display for ClassDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}class {} {{", 
               if self.is_public.is_some() { "pub " } else { "" }, 
               self.name.text)?;
        for field in self.fields.iter() {
            write!(f, "{field}")?;
        }
        write!(f, "}}")
    }
}

impl Display for ClassDefinitionFieldAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassDefinitionFieldAst::Field(field) => write!(f, "{field}"),
            ClassDefinitionFieldAst::Function(function) => write!(f, "{function}"),
        }
    }
}
