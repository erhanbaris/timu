//! Expression parsing with operator precedence for the Timu language.
//!
//! This module implements a recursive descent parser for Timu expressions using operator
//! precedence climbing. It handles all expression types including:
//!
//! - Arithmetic operations: `+`, `-`, `*`, `/`, `%`
//! - Logical operations: `&&`, `||`, `!`
//! - Bitwise operations: `&`, `|`, `^`, `<<`, `>>`
//! - Comparison operations: `==`, `!=`, `<`, `>`, `<=`, `>=`
//! - Function calls and method invocations
//! - Variable references and field access
//! - Parenthesized expressions
//!
//! # Operator Precedence
//!
//! The parser implements the following precedence levels (highest to lowest):
//! 1. **Primary expressions**: literals, identifiers, parentheses, function calls
//! 2. **Multiplicative**: `*`, `/`, `%`
//! 3. **Additive**: `+`, `-`
//! 4. **Bitwise shift**: `<<`, `>>`
//! 5. **Relational**: `<`, `>`, `<=`, `>=`
//! 6. **Equality**: `==`, `!=`
//! 7. **Bitwise AND**: `&`
//! 8. **Bitwise XOR**: `^`
//! 9. **Bitwise OR**: `|`
//! 10. **Logical AND**: `&&`
//! 11. **Logical OR**: `||`
//!
//! # Architecture
//!
//! The module uses a trait-based approach where each precedence level is implemented
//! as a separate parser struct implementing `TimuExpressionParser`. This provides
//! clean separation of concerns and makes the precedence hierarchy explicit.

use std::fmt::{Display, Formatter};

use nom::{branch::alt, bytes::complete::tag, character::complete::char, combinator::{cut, not, value}, error::context, multi::many, sequence::{delimited, pair, preceded}, IResult, Parser};

use crate::{ast::{ExpressionAst, ExpressionOperatorType, FunctionCallAst, PrimitiveValue, RefAst}, nom_tools::{cleanup, NomSpan}};

use super::{ident, TimuParserError};

/// Type alias for expression generator functions used in operator parsing
/// 
/// This function type is used to create binary expression AST nodes from
/// left operand, operator, and right operand.
pub type ControlExpressionGeneratorFn<'base, T> = fn(ExpressionAst<'base>, T, ExpressionAst<'base>) -> ExpressionAst<'base>;

/// Trait for expression parsers at different precedence levels
/// 
/// Each precedence level implements this trait to provide a uniform
/// interface for parsing expressions at that level.
pub trait TimuExpressionParser {
    /// Parse an expression at this precedence level
    fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>>;
}

// Precedence level parsers (listed from lowest to highest precedence)

/// Parser for logical OR expressions (`||`) - lowest precedence
struct OrParser;

/// Parser for logical AND expressions (`&&`)
struct AndParser;

/// Parser for bitwise XOR expressions (`^`)
struct BitwiseXorParser;

/// Parser for bitwise OR expressions (`|`)
struct BitwiseOrParser;

/// Parser for bitwise AND expressions (`&`)
struct BitwiseAndParser;

/// Parser for equality expressions (`==`, `!=`)
struct EqualParser;

/// Parser for relational expressions (`<`, `>`, `<=`, `>=`)
struct LessEqualParser;

/// Parser for bitwise shift expressions (`<<`, `>>`)
struct BitwiseShiftParser;

/// Parser for additive expressions (`+`, `-`)
struct AddSubParser;

/// Parser for multiplicative expressions (`*`, `/`, `%`) - highest binary precedence
struct MulDivModParser;

/// Parser for primary expressions (literals, identifiers, function calls, parentheses)
struct InnerParser;

impl TimuExpressionParser for OrParser {
    fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, AndParser, _, _>(input, ExpressionOperatorType::Or, tag("||"), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for AndParser {
    fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, BitwiseXorParser, _, _>(input, ExpressionOperatorType::And, tag("&&"), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for BitwiseXorParser {
    fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, BitwiseOrParser, _, _>(input, ExpressionOperatorType::Xor, char('^'), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for BitwiseOrParser {
    fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, BitwiseAndParser, _, _>(input, ExpressionOperatorType::LogicalOr, (char('|'), not(char('|'))), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for BitwiseAndParser {
    fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, EqualParser, _, _>(input, ExpressionOperatorType::LogicalAnd, (char('&'), not(char('&'))), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for EqualParser {
    fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::value_parser::<'_, LessEqualParser, _, _>(input, alt((
            value(ExpressionOperatorType::Equal, tag("==")),
            value(ExpressionOperatorType::NotEqual, tag("!="))
        )), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for LessEqualParser {
    fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::value_parser::<'_, BitwiseShiftParser, _, _>(input, alt((
            value(ExpressionOperatorType::LessEqualThan, tag("<=")),
            value(ExpressionOperatorType::GreaterEqualThan, tag(">=")),
            value(ExpressionOperatorType::GreaterThan, char('>')),
            value(ExpressionOperatorType::LessThan, char('<')),
        )), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for BitwiseShiftParser {
    fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::value_parser::<'_, AddSubParser, _, _>(input, alt((
            value(ExpressionOperatorType::BitwiseShiftRight, tag(">>")),
            value(ExpressionOperatorType::BitwiseShiftLeft, tag("<<")),
        )), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for AddSubParser {
    fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::value_parser::<'_, MulDivModParser, _, _>(input, alt((
            value(ExpressionOperatorType::Add, char('+')),
            value(ExpressionOperatorType::Sub, char('-'))
        )), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for MulDivModParser {
    fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::value_parser::<'_, InnerParser, _, _>(input, alt((
            value(ExpressionOperatorType::Div, char('/')),
            value(ExpressionOperatorType::Mul, char('*')),
            value(ExpressionOperatorType::Mod, char('%')),
        )), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for InnerParser {
    fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::inner(input)
    }
}

impl ExpressionAst<'_> {
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        OrParser::parse(input)
    }

    fn inner(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, expression) = cleanup(alt((
            RefAst::parse_for_expression,
            FunctionCallAst::parse_for_expression,
            PrimitiveValue::parse_for_expression,
            Self::not,
            Self::ident_for_expression,
            Self::parentheses,
        ))).parse(input)?;

        Ok((input, expression))
    }

    pub fn parentheses(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, expr) = delimited(char('('), cleanup(Self::parse), char(')')).parse(input)?;
        Ok((input, expr))
    }

    pub fn not(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, _) = cleanup(char('!')).parse(input)?;
        let (input, expression) = context("Expression missing", cut(Self::inner)).parse(input)?;
        Ok((
            input,
            ExpressionAst::Not(Box::new(expression)),
        ))
    }

    fn ident_for_expression(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst<'_>, TimuParserError<'_>> {
        let (input, ident) = ident().parse(input)?;
        Ok((
            input,
            ExpressionAst::Ident(ident.into()),
        ))
    }

    pub fn expr_builder<'base>(left: ExpressionAst<'base>, operator: ExpressionOperatorType, right: ExpressionAst<'base>) -> ExpressionAst<'base> {
        ExpressionAst::Operation {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    #[allow(private_bounds)]
    pub fn single_parser<'base, P: TimuExpressionParser, T: Copy, F: Parser<NomSpan<'base>, Error = TimuParserError<'base>>>(input: NomSpan<'base>, val: T, parser: F, expr_func: ControlExpressionGeneratorFn<'base, T>) -> IResult<NomSpan<'base>, ExpressionAst<'base>, TimuParserError<'base>> {
        let (input, initial) = P::parse(input)?;
        let (input, remainder): (NomSpan<'_>, Vec<ExpressionAst<'_>>) = many(0.., preceded(parser, P::parse)).parse(input)?;
        Ok((input, Self::single_fold_exprs::<T>(initial, val, remainder, expr_func)))
    }
    
    #[allow(private_bounds)]
    pub fn value_parser<'base, P: TimuExpressionParser, T: Copy, F: Parser<NomSpan<'base>, Error = TimuParserError<'base>>>(input: NomSpan<'base>, parser: F, expr_func: ControlExpressionGeneratorFn<'base, T>) -> IResult<NomSpan<'base>, ExpressionAst<'base>, TimuParserError<'base>> 
        where Vec<(T, ExpressionAst<'base>)>: Extend<(<F as Parser<NomSpan<'base>>>::Output, ExpressionAst<'base>)>
    {
        let (input, initial) = P::parse(input)?;
        let (input, remainder): (NomSpan<'_>, Vec<(T, ExpressionAst<'_>)>) = many(0.., pair(parser, P::parse)).parse(input)?;
        Ok((input, Self::value_fold_exprs::<T>(initial, remainder, expr_func)))
    }

    pub fn single_fold_exprs<'base, T: Copy>(initial: ExpressionAst<'base>, operator: T, remainder: Vec<ExpressionAst<'base>>, expr_func: ControlExpressionGeneratorFn<'base, T>) -> ExpressionAst<'base> {
        remainder.into_iter().fold(initial, |left, right| {
          expr_func(left, operator, right)
        })
    }

    pub fn value_fold_exprs<'base, T: Copy>(initial: ExpressionAst<'base>, remainder: Vec<(T, ExpressionAst<'base>)>, expr_func: ControlExpressionGeneratorFn<'base, T>) -> ExpressionAst<'base> {
        remainder.into_iter().fold(initial, |left, (operator, right)| {
          expr_func(left, operator, right)
        })
    }
}

impl Display for ExpressionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionAst::Primitive { value, .. } => write!(f, "{value}"),
            ExpressionAst::Ident(ident) => write!(f, "{ident}"),
            ExpressionAst::FunctionCall(function_call) => write!(f, "{function_call}"),
            ExpressionAst::Operation { left, operator, right } => {
                write!(f, "({left} {operator} {right})")
            },
            ExpressionAst::Ref(ref_expr) => {
                write!(f, "{ref_expr}")
            },
            ExpressionAst::Not(expression) => {
                write!(f, "!{expression}")
            },
        }
    }
}

impl Display for ExpressionOperatorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionOperatorType::Add => write!(f, "+"),
            ExpressionOperatorType::Sub => write!(f, "-"),
            ExpressionOperatorType::Mul => write!(f, "*"),
            ExpressionOperatorType::Div => write!(f, "/"),
            ExpressionOperatorType::Mod => write!(f, "%"),
            ExpressionOperatorType::And => write!(f, "&&"),
            ExpressionOperatorType::Or => write!(f, "||"),
            ExpressionOperatorType::Equal => write!(f, "=="),
            ExpressionOperatorType::NotEqual => write!(f, "!="),
            ExpressionOperatorType::LessThan => write!(f, "<"),
            ExpressionOperatorType::GreaterThan => write!(f, ">"),
            ExpressionOperatorType::GreaterEqualThan => write!(f, ">="),
            ExpressionOperatorType::LessEqualThan => write!(f, "<="),
            ExpressionOperatorType::Xor => write!(f, "^"),
            ExpressionOperatorType::LogicalOr => write!(f, "|"),
            ExpressionOperatorType::LogicalAnd => write!(f, "&"),
            ExpressionOperatorType::BitwiseShiftLeft => write!(f, "<<"),
            ExpressionOperatorType::BitwiseShiftRight => write!(f, ">>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{ast::ExpressionAst, file::SourceFile, nom_tools::{NomSpan, State}};

    #[rstest]
    #[case("1", "1")]
    #[case("1-2", "(1 - 2)")]
    #[case("1+2", "(1 + 2)")]
    #[case("(1+2)", "(1 + 2)")]
    #[case("    1 / 2  ", "(1 / 2)")]
    #[case("    1 / 2  ", "(1 / 2)")]
    #[case("    call(1,2,3) / 2  ", "(call(1, 2, 3) / 2)")]
    #[case("  \r\n\t  1 \r\n\t/\r\n\t 2  \r\n\t", "(1 / 2)")]
    #[case("2*2/ 2 * 22 - 2 - ( 5 - 1) + 3", "((((((2 * 2) / 2) * 22) - 2) - (5 - 1)) + 3)")]
    fn binary_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
        indexer: Default::default(),
    };

        let input = NomSpan::new_extra(source_file.code().as_str(), state);
        let (_, response) = ExpressionAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{code}");
    }

    #[rstest]
    #[case("!1", "!1")]
    #[case("!1 + 10", "(!1 + 10)")]
    #[case("!1", "!1")]
    #[case("!!1", "!!1")]
    #[case("!call(10)", "!call(10)")]
    #[case("!call(10) - 20", "(!call(10) - 20)")]
    fn not_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(source_file.code().as_str(), state);
        let (_, response) = ExpressionAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{code}");
    }

    #[rstest]
    #[case("1 - 10 == 20", "((1 - 10) == 20)")]
    #[case("1 - 10 == 20 * 4", "((1 - 10) == (20 * 4))")]
    #[case("1 - 10 == 20 * 4", "((1 - 10) == (20 * 4))")]
    #[case("1 - 10 == 20 * 4 >> 2", "((1 - 10) == ((20 * 4) >> 2))")]
    #[case("1 - 10 == 20 * 4 << 2", "((1 - 10) == ((20 * 4) << 2))")]
    #[case("20 && 10 | 30", "(20 && (10 | 30))")]
    #[case("20 || 10 & 30", "(20 || (10 & 30))")]
    #[case("20 % 10 == 10 || 30 > 20", "(((20 % 10) == 10) || (30 > 20))")]
    #[case("20 % 10 != 10 || 30 >= 20", "(((20 % 10) != 10) || (30 >= 20))")]
    #[case("20 % 10 != 10 || 30 < 20", "(((20 % 10) != 10) || (30 < 20))")]
    #[case("20 % 10 != 10 || 30 <= 20", "(((20 % 10) != 10) || (30 <= 20))")]
    #[case("20 ^ 10 | 30", "(20 ^ (10 | 30))")]
    fn general_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(source_file.code().as_str(), state);
        let (_, response) = ExpressionAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{code}");
    }
}
