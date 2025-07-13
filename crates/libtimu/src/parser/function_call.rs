//! Function call parsing for the Timu language.
//!
//! This module handles parsing of function call expressions and statements in Timu.
//! Function calls can appear in multiple contexts: as expressions within other expressions,
//! as standalone statements, or as method calls on objects.
//!
//! # Function Call Syntax
//!
//! Timu supports several types of function calls:
//!
//! ## Direct Function Calls
//! ```timu
//! functionName()
//! functionName(arg1, arg2, arg3)
//! module.function(args)
//! nested.module.function(args)
//! ```
//!
//! ## Method Calls (This Context)
//! ```timu
//! this.method()
//! this.method(args)
//! this.field.method(args)
//! ```
//!
//! ## Nested Function Calls
//! ```timu
//! outer(inner(arg))
//! complex(func1(a), func2(b, c), func3())
//! ```
//!
//! # Parsing Architecture
//!
//! The parser handles function calls in different contexts:
//! - **Expression context**: Function calls that return values used in expressions
//! - **Statement context**: Function calls that stand alone as statements (with semicolon)
//! - **Path resolution**: Support for qualified names and method call syntax
//!
//! # Error Handling
//!
//! The parser provides detailed error messages for common issues:
//! - Missing closing parentheses
//! - Invalid argument syntax
//! - Malformed qualified paths

use std::fmt::{Display, Formatter};

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{consumed, cut, map, opt, peek};
use nom::error::context;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::terminated;
use nom::{IResult, Parser, sequence::delimited};

use crate::ast::{
    BodyStatementAst, ExpressionAst, FunctionCallAst, FunctionCallPathAst, FunctionCallType
};
use crate::nom_tools::{NomSpan, cleanup};
use crate::parser::ident;

use super::TimuParserError;

impl FunctionCallAst<'_> {
    /// Parses a function call expression with optional method call syntax
    /// 
    /// This is the main parsing function for function calls. It handles both direct
    /// function calls (`function()`) and method calls (`this.method()`). The parser
    /// uses lookahead to ensure it only consumes valid function call syntax.
    /// 
    /// # Syntax Patterns
    /// - `functionName(args)` - Direct function call
    /// - `module.function(args)` - Qualified function call
    /// - `this.method(args)` - Method call on current object
    /// - `this.field.method(args)` - Method call on field
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, ast))` - Successfully parsed function call
    /// * `Err(error)` - Parse error with context information
    /// 
    /// # Errors
    /// Returns errors for:
    /// - Missing opening parenthesis
    /// - Missing closing parenthesis  
    /// - Invalid argument expressions
    /// - Malformed path components
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, FunctionCallAst<'_>, TimuParserError<'_>> {
        let (input, this) = match opt(tag("this")).parse(input)? {
            (input, Some(this)) => {
                let (input, _) = opt(char('.')).parse(input)?;
                (input, Some(this))
            },
            (input, None) => (input, None),
        };
        
        let (input, (call_span, paths)) = consumed(terminated(
            separated_list1(char('.'), ident()),
            peek(cleanup(char('('))))).parse(input)?;
        
        let (input, (arguments_span, arguments)) =
            consumed(map(delimited(char('('), cleanup(separated_list0(char(','), ExpressionAst::parse)), context("Missing ')'", cut(char(')')))), |items| {
                items
            }))
            .parse(input)?;

        Ok((
            input,
            FunctionCallAst {
                call_span: call_span.into(),
                arguments_span: arguments_span.into(),
                path: match this {
                    Some(_) => FunctionCallType::This(paths.into_iter().map(|item| item.into()).collect::<Vec<_>>()),
                    None => FunctionCallType::Direct(paths.into_iter().map(|item| item.into()).collect::<Vec<_>>()),
                },
                arguments,
            },
        ))
    }

    /// Parses a function call as a statement (with required semicolon)
    /// 
    /// This parser variant handles function calls that appear as standalone statements
    /// in a code block. Unlike expression function calls, these must be terminated
    /// with a semicolon and don't return a value to be used elsewhere.
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, statement))` - Successfully parsed function call statement
    /// * `Err(error)` - Parse error, often due to missing semicolon
    /// 
    /// # Examples
    /// ```timu
    /// functionName();
    /// this.method(arg1, arg2);
    /// module.utility.function();
    /// ```
    pub fn parse_body_statement(input: NomSpan<'_>) -> IResult<NomSpan<'_>, BodyStatementAst<'_>, TimuParserError<'_>> {
        let (input, function_call) = Self::parse(input)?;
        let (input, _) = context("Missing ';'", cut(cleanup(char(';')))).parse(input)?;
        Ok((input, BodyStatementAst::FunctionCall(function_call)))
    }

    /// Parses a function call for use within expressions
    /// 
    /// This parser variant handles function calls that appear within larger expressions,
    /// such as as arguments to other functions, operands in arithmetic, or conditions
    /// in control flow. No semicolon is expected.
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, expression))` - Successfully parsed function call expression
    /// * `Err(error)` - Parse error from the underlying function call parser
    /// 
    /// # Examples
    /// ```timu
    /// result = calculate(getValue(), getMultiplier());
    /// if (isValid(input)) { ... }
    /// array[getIndex()]
    /// ```
    pub fn parse_for_expression(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst<'_>, TimuParserError<'_>> {
        let (input, function_call) = Self::parse(input)?;
        Ok((input, ExpressionAst::FunctionCall(function_call)))
    }

    /// Helper function to parse identifiers for function path components
    /// 
    /// This is an internal utility function that converts parsed identifiers
    /// into function call path AST nodes. Currently unused but maintained
    /// for potential future path resolution enhancements.
    #[allow(dead_code)]
    fn ident_for_function_path(input: NomSpan<'_>) -> IResult<NomSpan<'_>, FunctionCallPathAst<'_>, TimuParserError<'_>> {
        let (input, path) = ident().parse(input)?;
        Ok((
            input,
            FunctionCallPathAst::Ident(path.into()),
        ))
    }
}

impl Display for FunctionCallAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        /*for (i, path) in self.paths.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", path)?;
        }*/
        
        let call_path = match &self.path {
            FunctionCallType::This(paths) => {
                if paths.is_empty() {
                    "this".to_string()
                } else {
                    format!("this.{}", paths.iter().map(|p| p.text).collect::<Vec<_>>().join("."))
                }
            }
            FunctionCallType::Direct(paths) => paths.iter().map(|p| p.text).collect::<Vec<_>>().join("."),
        };
        write!(f, "{call_path}")?;
        write!(f, "(")?;
        for (i, arg) in self.arguments.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{arg}")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl Display for FunctionCallPathAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionCallPathAst::Ident(ident) => write!(f, "{}", ident.text),
            FunctionCallPathAst::TypeName(type_name) => write!(f, "{type_name}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{ast::FunctionCallAst, file::SourceFile, nom_tools::{NomSpan, State}};

    #[rstest]
    #[case("test()", "test()")]
    #[case("  test   (   )   ", "test()")]
    #[case(" test1. test2   (   )   ", "test1.test2()")]
    #[case(" this. test2   (  a,b,c )   ", "this.test2(a, b, c)")]
    #[case("a(b())", "a(b())")]
    #[case("  a ( b ( ) ) ", "a(b())")]
    #[case("a(b(1,2,3,4,true,false,1.2,2.2, c()))", "a(b(1, 2, 3, 4, true, false, 1.2, 2.2, c()))")]
    fn function_call_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(source_file.code().as_str(), state);
        let (_, response) = FunctionCallAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{code}");
    }
}