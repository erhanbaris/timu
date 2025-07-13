//! Reference expression parsing for the Timu language.
//!
//! This module handles parsing of reference expressions, which use the `ref` keyword
//! to create references to variables and objects. References in Timu allow accessing
//! values indirectly and are fundamental for memory management and parameter passing.
//!
//! # Reference Syntax
//!
//! References are declared using the `ref` keyword followed by a dot-separated path:
//!
//! ```timu
//! ref variableName        // Simple reference
//! ref object.field        // Field reference
//! ref module.object.field // Qualified reference
//! ```
//!
//! # Reference Types
//!
//! ## Simple References
//! - **Variable references**: `ref variable` - References to local variables
//! - **Parameter references**: `ref param` - References to function parameters
//!
//! ## Path References
//! - **Field references**: `ref object.field` - References to object fields
//! - **Qualified references**: `ref module.Type.field` - References with module qualification
//! - **Nested references**: `ref outer.inner.field` - References through multiple levels
//!
//! # Integration with Type System
//!
//! Reference expressions are used extensively in:
//! - **Parameter passing**: Passing references instead of copying values
//! - **Field access**: Accessing object fields by reference
//! - **Memory management**: Enabling efficient memory usage patterns
//! - **Expression evaluation**: Supporting reference-based computations

use std::fmt::{Display, Formatter};

use nom::{bytes::complete::tag, character::complete::char, combinator::cut, error::context, multi::separated_list1, IResult, Parser};

use crate::{ast::{ExpressionAst, RefAst}, nom_tools::{cleanup, NomSpan, Span}};

use super::{ident, TimuParserError};

impl RefAst<'_> {
    /// Parses a reference expression starting with the `ref` keyword
    /// 
    /// This function parses reference expressions that begin with `ref` followed by
    /// a dot-separated path of identifiers. The path can be a simple variable name
    /// or a complex qualified path accessing nested fields or module members.
    /// 
    /// # Syntax
    /// ```timu
    /// ref variableName
    /// ref object.field
    /// ref module.Type.field
    /// ref deeply.nested.path.field
    /// ```
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, ref_ast))` - Successfully parsed reference expression
    /// * `Err(error)` - Parse error with context information
    /// 
    /// # Errors
    /// Returns errors for:
    /// - Missing reference name after `ref` keyword
    /// - Invalid identifier syntax in the path
    /// - Missing path components (empty path after `ref`)
    /// 
    /// # Features
    /// - **Path parsing**: Supports dot-separated identifier paths
    /// - **Error recovery**: Provides meaningful error messages for missing names
    /// - **Whitespace handling**: Automatically handles whitespace around components
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, RefAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup(tag("ref")).parse(input)?;
        let (input, names) = context("Reference name missing", cut(separated_list1(cleanup(char('.')), ident()))).parse(input)?;
        Ok((
            input,
            RefAst {
                names: names.into_iter().map(Span::from).collect::<Vec<_>>(),
            },
        ))
    }

    /// Parses a reference expression and wraps it as an `ExpressionAst`
    /// 
    /// This convenience method parses a reference expression using the main `parse`
    /// method and then wraps the result in an `ExpressionAst::Ref` variant for use
    /// within the expression parsing system.
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, expression))` - Successfully parsed reference as expression
    /// * `Err(error)` - Parse error from the underlying reference parsing
    /// 
    /// # Usage
    /// This method is primarily used by the expression parser when it encounters
    /// a reference expression within a larger expression context, such as:
    /// - Function call arguments: `func(ref variable)`
    /// - Arithmetic expressions: `ref field + 10`
    /// - Assignment targets: `ref object.field = value`
    pub fn parse_for_expression(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst<'_>, TimuParserError<'_>> {
        let (input, path) = Self::parse(input)?;
        Ok((
            input,
            ExpressionAst::Ref(path),
        ))
    }
}

impl Display for RefAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ref ")?;

        for (i, name) in self.names.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", name.text)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{ast::RefAst, file::SourceFile, nom_tools::{NomSpan, State}};

    #[rstest]
    #[case("ref erha_1n", "ref erha_1n")]
    #[case("ref a.b", "ref a.b")]
    #[case(" ref  a ", "ref a")]
    #[case("ref a . b  ", "ref a.b")]
    fn reference_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(source_file.code().as_str(), state);
        let (_, response) = RefAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{code}");
    }
}