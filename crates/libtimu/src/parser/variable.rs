//! Variable declaration and assignment parsing for the Timu language.
//!
//! This module handles parsing of variable declarations and assignments, which are
//! fundamental constructs for data management in Timu programs. The module supports
//! both mutable and immutable variable declarations with optional type annotations
//! and initialization expressions.
//!
//! # Variable Declaration Syntax
//!
//! ## Variable Declarations (`var`)
//! ```timu
//! var name = expression;           // Type inferred from expression
//! var name: Type = expression;     // Explicit type with initialization
//! var name: ?Type;                 // Nullable type without initialization
//! ```
//!
//! ## Constant Declarations (`const`)
//! ```timu
//! const name = expression;         // Type inferred, must be initialized
//! const name: Type = expression;   // Explicit type with initialization
//! ```
//!
//! ## Variable Assignments
//! ```timu
//! variableName = newValue;         // Assign new value to existing variable
//! ```
//!
//! # Type System Integration
//!
//! ## Type Inference
//! When no explicit type is provided, the compiler infers the type from the
//! initialization expression. This reduces verbosity while maintaining type safety.
//!
//! ## Nullable Types
//! Variables with nullable types (`?Type`) can be declared without initialization,
//! and they will have a default value of `null`.
//!
//! ## Const Variables
//! Constants must always be initialized and cannot be reassigned after declaration.
//! They provide compile-time guarantees about value immutability.
//!
//! # Error Handling
//!
//! The parser provides detailed error messages for common mistakes:
//! - Missing variable names
//! - Missing initialization for non-nullable variables
//! - Missing initialization for const variables
//! - Invalid type annotations
//! - Missing assignment operators

use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, map, opt};
use nom::error::context;
use nom::{IResult, Parser};
use nom_language::error::{VerboseError, VerboseErrorKind};

use crate::ast::{BodyStatementAst, ExpressionAst, TypeNameAst, VariableAssignAst, VariableDefinitionAst, VariableDefinitionType};
use crate::nom_tools::{NomSpan, cleanup};
use crate::parser::{expected_ident, ident};

use super::TimuParserError;

impl VariableDefinitionAst<'_> {
    /// Parses a variable definition for use as a statement
    /// 
    /// This parser variant wraps the main variable definition parser for use
    /// within statement contexts such as function bodies and code blocks.
    pub fn parse_body_statement(input: NomSpan<'_>) -> IResult<NomSpan<'_>, BodyStatementAst<'_>, TimuParserError<'_>> {
        let (input, variable) = Self::parse(input)?;
        Ok((input, BodyStatementAst::VariableDefinition(variable)))
    }

    /// Parses a complete variable definition
    /// 
    /// This is the main parser for variable declarations. It handles both `var` and
    /// `const` declarations with optional type annotations and initialization expressions.
    /// The parser enforces language rules such as requiring initialization for constants.
    /// 
    /// # Parsing Logic
    /// 1. Parse variable kind (`var` or `const`)
    /// 2. Parse variable name (identifier)
    /// 3. Optionally parse type annotation after `:`
    /// 4. Optionally parse initialization expression after `=`
    /// 5. Validate combination according to language rules
    /// 6. Require terminating semicolon
    /// 
    /// # Language Rules
    /// - Constants must have initialization expressions
    /// - Variables without type annotations must have initialization
    /// - Nullable types can be declared without initialization
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, ast))` - Successfully parsed variable definition
    /// * `Err(error)` - Parse error with detailed context
    /// 
    /// # Examples
    /// ```timu
    /// var counter = 0;                    // Inferred type
    /// var name: string = "hello";         // Explicit type
    /// var optional: ?i32;                 // Nullable without init
    /// const PI = 3.14159;                 // Constant with inference
    /// const MAX_SIZE: i32 = 1000;         // Constant with explicit type
    /// ```
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, VariableDefinitionAst<'_>, TimuParserError<'_>> {
        let (input, variable_definition_type) = cleanup(alt((
            map(tag("var"), |_| VariableDefinitionType::Var),
            map(tag("const"), |_| VariableDefinitionType::Const))
        )).parse(input)?;
        let (input, name) = expected_ident("Missing variable name", input)?;
        
        let (input, expected_type, expression) = match cleanup(opt(char(':'))).parse(input)? {
            (input, Some(_)) => {
                let (input, expected_type) = context("Missing variable type", cut(cleanup(TypeNameAst::parse))).parse(input)?;
                
                // If the type is nullable, no need for expression, but if there is a equal sign, expression is required
                let (input, expression) = match cleanup(opt(char('='))).parse(input)? {
                    (input, Some(_)) => {
                        let (input, expression) = context("Invalid expression", cut(ExpressionAst::parse)).parse(input)?;
                        (input, Some(expression))
                    },
                    (input, None) => (input, None),
                };

                if variable_definition_type == VariableDefinitionType::Const && expression.is_none() {
                    return Err(nom::Err::Failure(VerboseError {
                        errors: vec![(input, VerboseErrorKind::Context("Const variable must have an assignment"))],
                    }));
                }
                
                (input, Some(expected_type), expression)
            }

            // If there is no type, we must have an expression
            (input, None) => {
                let (input, _) = context("Missing '='", cleanup(cut(char('=')))).parse(input)?;
                let (input, expression) = context("Invalid expression", cut(ExpressionAst::parse)).parse(input)?;
                (input, None, Some(expression))
            }
        };

        let (input, _) = context("Missing ';'", cleanup(char(';'))).parse(input)?;

        Ok((
            input,
            VariableDefinitionAst {
                variable_definition_type,
                name: name.into(),
                expected_type,
                expression,
            },
        ))
    }
}

impl Display for VariableDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.variable_definition_type, self.name.text)?;
        if let Some(expected_type) = &self.expected_type {
            write!(f, ": {expected_type}")?;
        }
        if let Some(expression) = &self.expression {
            write!(f, " = {expression}")?;
        }
        write!(f, ";")
    }
}

impl VariableAssignAst<'_> {
    /// Parses a variable assignment for use as a statement
    /// 
    /// This parser variant wraps the main variable assignment parser for use
    /// within statement contexts such as function bodies and code blocks.
    pub fn parse_body_statement(input: NomSpan<'_>) -> IResult<NomSpan<'_>, BodyStatementAst<'_>, TimuParserError<'_>> {
        let (input, variable) = Self::parse(input)?;
        Ok((input, BodyStatementAst::VariableAssign(variable)))
    }

    /// Parses a variable assignment statement
    /// 
    /// This parser handles assignment of new values to existing variables. The
    /// variable must have been previously declared, and the assigned expression
    /// must be compatible with the variable's type.
    /// 
    /// # Syntax
    /// ```timu
    /// variableName = expression;
    /// ```
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, ast))` - Successfully parsed variable assignment
    /// * `Err(error)` - Parse error with context information
    /// 
    /// # Errors
    /// Returns errors for:
    /// - Invalid variable name
    /// - Missing assignment operator (`=`)
    /// - Invalid expression syntax
    /// - Missing terminating semicolon
    /// 
    /// # Examples
    /// ```timu
    /// counter = counter + 1;
    /// name = "new name";
    /// result = calculateValue();
    /// flag = !flag;
    /// ```
    /// 
    /// # Type Checking
    /// While this parser handles syntax validation, type compatibility between
    /// the variable and the assigned expression is validated during semantic
    /// analysis in the TIR phase.
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, VariableAssignAst<'_>, TimuParserError<'_>> {
        let (input, name) = ident().parse(input)?;
        let (input, _) = context("Missing '='", cleanup(char('='))).parse(input)?;
        let (input, expression) = context("Invalid expression", cut(ExpressionAst::parse)).parse(input)?;
        let (input, _) = context("Missing ';'", cut(cleanup(char(';')))).parse(input)?;

        Ok((
            input,
            VariableAssignAst {
                name: name.into(),
                expression,
            },
        ))
    }
}

impl Display for VariableAssignAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {};", self.name.text, self.expression)
    }
}

impl Display for VariableDefinitionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableDefinitionType::Var => write!(f, "var"),
            VariableDefinitionType::Const => write!(f, "const"),
        }
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseErrorKind;
    use rstest::rstest;

    use crate::{ast::VariableDefinitionAst, file::SourceFile, nom_tools::{NomSpan, State}};
    
    #[rstest]
    #[case("var a = false;", "var a = false;")]
    #[case("var a = 100;", "var a = 100;")]
    #[case("var a: ?i32;", "var a: ?i32;")]
    #[case("var a = 100;", "var a = 100;")]
    #[case("var _a = 200;", "var _a = 200;")]
    #[case("const a = 100;", "const a = 100;")]
    #[case("const _123a = 100;", "const _123a = 100;")]
    #[case("const a_123 = 100;", "const a_123 = 100;")]
    #[case("const a_123_____ = 100;", "const a_123_____ = 100;")]
    #[case("const a = 1.0;", "const a = 1.0;")]
    #[case("const a = 1.2;", "const a = 1.2;")]
    #[case("const a = -1.2;", "const a = -1.2;")]
    #[case("const a: f64 = -1.2;", "const a: f64 = -1.2;")]
    #[case("const a: ?f64 = -1.2;", "const a: ?f64 = -1.2;")]
    fn custom_variable_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(code, state);
        let result = VariableDefinitionAst::parse(input);
        assert!(result.is_ok(), "Failed to parse type name: {:?}", result.err());
        let (_, parsed) = result.unwrap();

        assert_eq!(parsed.to_string(), expected);
    }

    #[rstest]
    #[case("var a;", "Missing '='")]
    #[case("var a: ;", "Missing variable type")]
    #[case("const a: ?i32;", "Const variable must have an assignment")]
    fn invalid_variable_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(code, state);
        let result = VariableDefinitionAst::parse(input);
        let error = result.unwrap_err();

        if let nom::Err::Failure(error) = error {
            if let VerboseErrorKind::Context(ctx) = error.errors[error.errors.len() - 1].1 {
                assert_eq!(ctx, expected, "{code}");
            } else {
                panic!("Expected an error, but got: {error:#?}");
            }
        } else {
            panic!("Expected an error, but got: {error:#?}");
        }
    }
}