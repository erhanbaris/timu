//! Type annotation parsing for the Timu language.
//!
//! This module handles parsing of type annotations and type names throughout the Timu
//! language. Type annotations are used in variable declarations, function parameters,
//! return types, and anywhere else type information is needed.
//!
//! # Type Syntax
//!
//! Timu supports a rich type system with several type modifiers and qualifiers:
//!
//! ## Basic Types
//! ```timu
//! string          // Simple type name
//! MyClass         // User-defined type
//! module.Type     // Qualified type name
//! ```
//!
//! ## Type Modifiers
//! ```timu
//! ?string         // Nullable type (optional)
//! ref string      // Reference type
//! ref ?string     // Reference to nullable type
//! ```
//!
//! ## Qualified Names
//! ```timu
//! std.collections.HashMap     // Fully qualified type
//! io.File                     // Module-qualified type
//! this.NestedType            // Type within current scope
//! ```
//!
//! # Parsing Features
//!
//! - **Nullable types**: Optional types marked with `?` prefix
//! - **Reference types**: Reference semantics marked with `ref` keyword
//! - **Qualified names**: Dot-separated module and type hierarchies
//! - **Function path conversion**: Types used in function call contexts
//!
//! # Type System Integration
//!
//! Parsed type annotations are used throughout the compiler for:
//! - Variable type checking
//! - Function signature validation
//! - Return type verification
//! - Generic type resolution (planned)

use std::fmt::{Display, Formatter};

use nom::{character::complete::char, combinator::{consumed, map}, multi::separated_list1, IResult, Parser};

use crate::{ast::{FunctionCallPathAst, TypeNameAst}, nom_tools::NomSpan, parser::is_reference};

use super::{ident, is_nullable, TimuParserError};


impl TypeNameAst<'_> {
    /// Parses a complete type annotation with modifiers and qualified names
    /// 
    /// This is the main parsing function for type annotations. It handles all
    /// type modifiers (`ref`, `?`) and qualified type names in the correct order.
    /// The parser processes modifiers left-to-right and builds the complete
    /// type representation.
    /// 
    /// # Parsing Order
    /// 1. Reference modifier (`ref`)
    /// 2. Nullable modifier (`?`)
    /// 3. Qualified type name (dot-separated identifiers)
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, type_ast))` - Successfully parsed type annotation
    /// * `Err(error)` - Parse error with context information
    /// 
    /// # Examples
    /// ```timu
    /// string                    // Basic type
    /// ?string                   // Nullable string
    /// ref string                // String reference
    /// ref ?string               // Reference to nullable string
    /// module.CustomType         // Qualified type name
    /// ref ?module.CustomType    // Complex qualified nullable reference
    /// ```
    /// 
    /// # Errors
    /// Returns errors for:
    /// - Invalid identifier syntax in type names
    /// - Malformed qualified names
    /// - Missing type name after modifiers
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, TypeNameAst<'_>, TimuParserError<'_>> {
        let (input, reference) = is_reference(input)?;
        let (input, nullable) = is_nullable(input)?;
        let (input, (names_span, names)) = consumed(map(separated_list1(char('.'), ident()), |items| items)).parse(input)?;
        Ok((
            input,
            TypeNameAst {
                reference,
                nullable,
                names: names.into_iter().map(|item| item.into()).collect::<Vec<_>>(),
                names_span: names_span.into(),
            },
        ))
    }

    /// Parses a type name for use in function call path resolution
    /// 
    /// This parser variant converts a parsed type name into a function call path
    /// component. This is used when types appear in qualified function calls,
    /// such as static method invocations or type-qualified function calls.
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, path))` - Successfully parsed function call path
    /// * `Err(error)` - Parse error from underlying type parsing
    /// 
    /// # Examples
    /// ```timu
    /// String.fromInt(42)        // String type used as function path
    /// utils.Math.sqrt(16)       // Qualified type path
    /// ```
    /// 
    /// # Usage Context
    /// This is typically used in expression parsing where types can appear
    /// as part of function call syntax, enabling static method calls and
    /// type-qualified function resolution.
    pub fn parse_for_function_path(input: NomSpan<'_>) -> IResult<NomSpan<'_>, FunctionCallPathAst<'_>, TimuParserError<'_>> {
        let (input, path) = Self::parse(input)?;
        Ok((
            input,
            FunctionCallPathAst::TypeName(path),
        ))
    }
}

impl Display for TypeNameAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.reference {
            write!(f, "ref ")?;
        }

        if self.nullable {
            write!(f, "?")?;
        }

        for (i, name) in self.names.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", name.text)?;
        }
        Ok(())
    }
}
