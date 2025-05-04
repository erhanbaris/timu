use std::fmt::{Display, Formatter};

use nom::{branch::alt, bytes::complete::tag, character::complete::char, combinator::{cut, not, value}, error::context, multi::many, sequence::{delimited, pair, preceded}, IResult, Parser};

use crate::{ast::{ExpressionAst, ExpressionOperatorType, FunctionCallAst, PrimitiveType, RefAst}, nom_tools::{cleanup, Span}};

use super::{ident, TimuParserError};

pub type ControlExpressionGeneratorFn<'a, T> = fn(ExpressionAst<'a>, T, ExpressionAst<'a>) -> ExpressionAst<'a>;

pub trait TimuExpressionParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>>;
}

struct OrParser;
struct AndParser;
struct BitwiseXorParser;
struct BitwiseOrParser;
struct BitwiseAndParser;
struct EqualParser;
struct LessEqualParser;
struct BitwiseShiftParser;
struct AddSubParser;
struct MulDivModParser;
struct InnerParser;

impl TimuExpressionParser for OrParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, AndParser, _, _>(input, ExpressionOperatorType::Or, tag("||"), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for AndParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, BitwiseXorParser, _, _>(input, ExpressionOperatorType::And, tag("&&"), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for BitwiseXorParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, BitwiseOrParser, _, _>(input, ExpressionOperatorType::Xor, char('^'), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for BitwiseOrParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, BitwiseAndParser, _, _>(input, ExpressionOperatorType::LogicalOr, (char('|'), not(char('|'))), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for BitwiseAndParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, EqualParser, _, _>(input, ExpressionOperatorType::LogicalAnd, (char('&'), not(char('&'))), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for EqualParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::value_parser::<'_, LessEqualParser, _, _>(input, alt((
            value(ExpressionOperatorType::Equal, tag("==")),
            value(ExpressionOperatorType::NotEqual, tag("!="))
        )), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for LessEqualParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::value_parser::<'_, BitwiseShiftParser, _, _>(input, alt((
            value(ExpressionOperatorType::LessEqualThan, tag("<=")),
            value(ExpressionOperatorType::GreaterEqualThan, tag(">=")),
            value(ExpressionOperatorType::GreaterThan, char('>')),
            value(ExpressionOperatorType::LessThan, char('<')),
        )), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for BitwiseShiftParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::value_parser::<'_, AddSubParser, _, _>(input, alt((
            value(ExpressionOperatorType::BitwiseShiftRight, tag(">>")),
            value(ExpressionOperatorType::BitwiseShiftLeft, tag("<<")),
        )), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for AddSubParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::value_parser::<'_, MulDivModParser, _, _>(input, alt((
            value(ExpressionOperatorType::Add, char('+')),
            value(ExpressionOperatorType::Sub, char('-'))
        )), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for MulDivModParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::value_parser::<'_, InnerParser, _, _>(input, alt((
            value(ExpressionOperatorType::Div, char('/')),
            value(ExpressionOperatorType::Mul, char('*')),
            value(ExpressionOperatorType::Mod, char('%')),
        )), ExpressionAst::expr_builder)
    }
}

impl TimuExpressionParser for InnerParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::inner(input)
    }
}

impl ExpressionAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        OrParser::parse(input)
    }

    fn inner(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, expression) = cleanup(alt((
            RefAst::parse_for_expression,
            FunctionCallAst::parse_for_expression,
            PrimitiveType::parse_for_expression,
            Self::not,
            Self::ident_for_expression,
            Self::parentheses,
        ))).parse(input)?;

        Ok((input, expression))
    }

    pub fn parentheses(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, expr) = delimited(char('('), cleanup(Self::parse), char(')')).parse(input)?;
        Ok((input, expr))
    }

    pub fn not(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, _) = cleanup(char('!')).parse(input)?;
        let (input, expression) = context("Expression missinh", cut(Self::inner)).parse(input)?;
        Ok((
            input,
            ExpressionAst::Not(Box::new(expression)),
        ))
    }

    fn ident_for_expression(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst<'_>, TimuParserError<'_>> {
        let (input, ident) = ident().parse(input)?;
        Ok((
            input,
            ExpressionAst::Ident(ident),
        ))
    }

    pub fn expr_builder<'a>(left: ExpressionAst<'a>, operator: ExpressionOperatorType, right: ExpressionAst<'a>) -> ExpressionAst<'a> {
        ExpressionAst::Operation {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    #[allow(private_bounds)]
    pub fn single_parser<'a, P: TimuExpressionParser, T: Copy, F: Parser<Span<'a>, Error = TimuParserError<'a>>>(input: Span<'a>, val: T, parser: F, expr_func: ControlExpressionGeneratorFn<'a, T>) -> IResult<Span<'a>, ExpressionAst<'a>, TimuParserError<'a>> {
        let (input, initial) = P::parse(input)?;
        let (input, remainder): (Span<'_>, Vec<ExpressionAst<'_>>) = many(0.., preceded(parser, P::parse)).parse(input)?;
        Ok((input, Self::single_fold_exprs::<T>(initial, val, remainder, expr_func)))
    }
    
    #[allow(private_bounds)]
    pub fn value_parser<'a, P: TimuExpressionParser, T: Copy, F: Parser<Span<'a>, Error = TimuParserError<'a>>>(input: Span<'a>, parser: F, expr_func: ControlExpressionGeneratorFn<'a, T>) -> IResult<Span<'a>, ExpressionAst<'a>, TimuParserError<'a>> 
        where Vec<(T, ExpressionAst<'a>)>: Extend<(<F as Parser<Span<'a>>>::Output, ExpressionAst<'a>)>
    {
        let (input, initial) = P::parse(input)?;
        let (input, remainder): (Span<'_>, Vec<(T, ExpressionAst<'_>)>) = many(0.., pair(parser, P::parse)).parse(input)?;
        Ok((input, Self::value_fold_exprs::<T>(initial, remainder, expr_func)))
    }

    pub fn single_fold_exprs<'a, T: Copy>(initial: ExpressionAst<'a>, operator: T, remainder: Vec<ExpressionAst<'a>>, expr_func: ControlExpressionGeneratorFn<'a, T>) -> ExpressionAst<'a> {
        remainder.into_iter().fold(initial, |left, right| {
          expr_func(left, operator, right)
        })
    }

    pub fn value_fold_exprs<'a, T: Copy>(initial: ExpressionAst<'a>, remainder: Vec<(T, ExpressionAst<'a>)>, expr_func: ControlExpressionGeneratorFn<'a, T>) -> ExpressionAst<'a> {
        remainder.into_iter().fold(initial, |left, (operator, right)| {
          expr_func(left, operator, right)
        })
    }
}

impl Display for ExpressionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionAst::Primitive(primitive) => write!(f, "{}", primitive),
            ExpressionAst::Ident(ident) => write!(f, "{}", ident),
            ExpressionAst::FunctionCall(function_call) => write!(f, "{}", function_call),
            ExpressionAst::Operation { left, operator, right } => {
                write!(f, "({} {} {})", left, operator, right)
            },
            ExpressionAst::Ref(ref_expr) => {
                write!(f, "{}", ref_expr)
            },
            ExpressionAst::Not(expression) => {
                write!(f, "!{}", expression)
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
    use std::rc::Rc;

    use rstest::rstest;

    use crate::{ast::ExpressionAst, file::SourceFile, nom_tools::{Span, State}};

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
    fn binary_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), "<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(state.file.code(), state);
        let (_, response) = ExpressionAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{}", code);
    }

    #[rstest]
    #[case("!1", "!1")]
    #[case("!1 + 10", "(!1 + 10)")]
    #[case("!1", "!1")]
    #[case("!!1", "!!1")]
    #[case("!call(10)", "!call(10)")]
    #[case("!call(10) - 20", "(!call(10) - 20)")]
    fn not_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), "<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(state.file.code(), state);
        let (_, response) = ExpressionAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{}", code);
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
    fn general_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), "<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(state.file.code(), state);
        let (_, response) = ExpressionAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{}", code);
    }
}
