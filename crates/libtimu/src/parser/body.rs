//! Code block and statement parsing for the Timu language.
//!
//! This module handles parsing of code blocks (function bodies, conditional blocks,
//! loop bodies, etc.) and the statements they contain. Code blocks are fundamental
//! structural elements that group statements together and define scope boundaries
//! in the Timu language.
//!
//! # Code Block Syntax
//!
//! Code blocks in Timu are delimited by braces and contain zero or more statements:
//!
//! ```timu
//! {
//!     // Statements go here
//!     var x = 10;
//!     functionCall();
//!     if (condition) {
//!         // Nested block
//!     }
//! }
//! ```
//!
//! # Supported Statement Types
//!
//! The body parser currently supports these statement types:
//!
//! ## Variable Declarations
//! ```timu
//! var name: Type = value;
//! const constant = 42;
//! ```
//!
//! ## Variable Assignments
//! ```timu
//! existingVariable = newValue;
//! object.field = value;
//! ```
//!
//! ## Function Calls
//! ```timu
//! functionName();
//! object.method(args);
//! this.method();
//! ```
//!
//! ## Control Flow
//! ```timu
//! if (condition) { /* body */ }
//! if (condition) { /* if body */ } else { /* else body */ }
//! ```
//!
//! # Parsing Architecture
//!
//! The body parser uses a straightforward approach:
//! 1. **Opening brace**: Expects `{` to start the block
//! 2. **Statement sequence**: Parses zero or more statements using alternation
//! 3. **Closing brace**: Expects `}` to end the block
//!
//! # Error Handling
//!
//! The parser provides detailed error messages for common issues:
//! - Missing opening or closing braces
//! - Invalid statement syntax within blocks
//! - Malformed control flow constructs
//!
//! # Integration with Language Features
//!
//! Code blocks integrate with several language features:
//! - **Scope management**: Each block creates a new lexical scope
//! - **Variable visibility**: Variables declared in blocks have block scope
//! - **Control flow**: Blocks serve as bodies for conditional and loop constructs
//! - **Function definitions**: Function bodies are parsed as code blocks

use std::fmt::{Display, Formatter};

use nom::{branch::alt, character::complete::char, combinator::cut, error::context, multi::many0, IResult, Parser};

use crate::{ast::{BodyAst, BodyStatementAst, FunctionCallAst, IfConditionAst, VariableAssignAst, VariableDefinitionAst}, nom_tools::{cleanup, NomSpan}};

use super::TimuParserError;


impl BodyAst<'_> {
    /// Parses a complete code block with opening brace, statements, and closing brace
    /// 
    /// This is the main parser for code blocks in Timu. It handles the full syntax
    /// including brace delimiters and the sequence of statements within the block.
    /// The parser supports all statement types and provides meaningful error messages
    /// for common syntax errors.
    /// 
    /// # Parsing Logic
    /// 1. Parse opening brace `{`
    /// 2. Parse zero or more statements using alternation to try different statement types
    /// 3. Parse closing brace `}`
    /// 4. Build AST with collected statements
    /// 
    /// # Supported Statements
    /// - **If conditions**: Conditional execution with optional else clauses
    /// - **Function calls**: Method invocations and function calls (as statements)
    /// - **Variable assignments**: Assignment of new values to existing variables
    /// - **Variable definitions**: Declaration of new variables and constants
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, body_ast))` - Successfully parsed code block
    /// * `Err(error)` - Parse error with context information
    /// 
    /// # Syntax Examples
    /// ```timu
    /// {
    ///     var counter = 0;
    ///     functionCall();
    ///     counter = counter + 1;
    ///     if (counter > 10) {
    ///         break;
    ///     }
    /// }
    /// ```
    /// 
    /// # Errors
    /// Returns errors for:
    /// - Missing opening brace
    /// - Missing closing brace
    /// - Invalid statement syntax within the block
    /// - Malformed control flow constructs
    /// 
    /// # Usage Context
    /// This parser is used for:
    /// - Function bodies in function definitions
    /// - If/else clause bodies in conditional statements
    /// - Loop bodies in iteration constructs (planned)
    /// - Block scopes for variable isolation
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, BodyAst<'_>, TimuParserError<'_>> {
        let (input, _) = context("Body's opening '{' missing", cut(cleanup(char('{')))).parse(input)?;
        let (input, statements) = many0(alt((
            IfConditionAst::parse_body_statement,
            FunctionCallAst::parse_body_statement,
            VariableAssignAst::parse_body_statement,
            VariableDefinitionAst::parse_body_statement,
        ))).parse(input)?;
        let (input, _) = context("Body's closing '}' missing", cut(cleanup(char('}')))).parse(input)?;

        Ok((
            input,
            BodyAst {
                statements,
            },
        ))
    }
}

impl Display for BodyAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (index, statement) in self.statements.iter().enumerate() {
            write!(f, "{statement}")?;
            if index < self.statements.len() - 1 {
                write!(f, " ")?;
            }
        }
        write!(f, "}}")
    }
}

impl Display for BodyStatementAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyStatementAst::VariableDefinition(var) => write!(f, "{var}"),
            BodyStatementAst::VariableAssign(var) => write!(f, "{var}"),
            BodyStatementAst::FunctionCall(func) => write!(f, "{func};"),
            BodyStatementAst::IfCondition(if_condition) => write!(f, "{if_condition}"),
        }
    }
}
