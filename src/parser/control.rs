use nom::{branch::alt, bytes::complete::tag, character::complete::char, combinator::value, multi::many, sequence::{pair, preceded}, IResult, Parser};

use crate::{ast::{ControlOperatorType, ExpressionAst}, nom_tools::Span};

use super::TimuParserError;

struct OrParser;
struct AndParser;
struct BitwiseXorParser;
struct BitwiseOrParser;
struct BitwiseAndParser;
struct EqualParser;
struct LessEqualParser;
struct BitwiseShiftParser;
struct AddSubParser;

type ExprGeneratorFn<'a, T> = fn(ExpressionAst<'a>, T, ExpressionAst<'a>) -> ExpressionAst<'a>;

trait TimuExpressionParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>>;
}

fn control_expr_builder<'a>(left: ExpressionAst<'a>, operator: ControlOperatorType, right: ExpressionAst<'a>) -> ExpressionAst<'a> {
    ExpressionAst::ControlOperation {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }
}

impl TimuExpressionParser for OrParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, AndParser, _, _>(input, ControlOperatorType::Or, tag("||"), control_expr_builder)
    }
}

impl TimuExpressionParser for AndParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, BitwiseXorParser, _, _>(input, ControlOperatorType::And, tag("&&"), control_expr_builder)
    }
}

impl TimuExpressionParser for BitwiseXorParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, BitwiseOrParser, _, _>(input, ControlOperatorType::Xor, char('^'), control_expr_builder)
    }
}

impl TimuExpressionParser for BitwiseOrParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, BitwiseXorParser, _, _>(input, ControlOperatorType::LogicalOr, char('|'), control_expr_builder)
    }
}

impl TimuExpressionParser for BitwiseAndParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::single_parser::<'_, EqualParser, _, _>(input, ControlOperatorType::LogicalAnd, char('&'), control_expr_builder)
    }
}

impl TimuExpressionParser for EqualParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::value_parser::<'_, LessEqualParser, _, _>(input, alt((
            value(ControlOperatorType::Equal, tag("==")),
            value(ControlOperatorType::NotEqual, tag("!="))
        )), control_expr_builder)
    }
}

impl TimuExpressionParser for LessEqualParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::value_parser::<'_, BitwiseShiftParser, _, _>(input, alt((
            value(ControlOperatorType::GreaterThan, tag(">")),
            value(ControlOperatorType::GreaterEqualThan, tag(">=")),
            value(ControlOperatorType::LessThan, tag("<")),
            value(ControlOperatorType::LessEqualThan, tag("<=")),
        )), control_expr_builder)
    }
}

impl TimuExpressionParser for BitwiseShiftParser {
    fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        ExpressionAst::value_parser::<'_, BitwiseShiftParser, _, _>(input, alt((
            value(ControlOperatorType::BitwiseShiftLeft, tag(">>")),
            value(ControlOperatorType::BitwiseShiftRight, tag("<<")),
        )), control_expr_builder)
    }
}

impl ExpressionAst<'_> {
    pub fn control(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        Self::equal_not_equal(input)
    }
    
    #[allow(private_bounds)]
    pub fn single_parser<'a, P: TimuExpressionParser, T: Copy, F: Parser<Span<'a>, Error = TimuParserError<'a>>>(input: Span<'a>, val: T, parser: F, expr_func: ExprGeneratorFn<'a, T>) -> IResult<Span<'a>, ExpressionAst<'a>, TimuParserError<'a>> {
        let (input, initial) = P::parse(input)?;
        let (input, remainder): (Span<'_>, Vec<ExpressionAst<'_>>) = many(0.., preceded(parser, Self::inner)).parse(input)?;
        Ok((input, Self::single_fold_exprs::<T>(initial, val, remainder, expr_func)))
    }
    
    #[allow(private_bounds)]
    pub fn value_parser<'a, P: TimuExpressionParser, T: Copy, F: Parser<Span<'a>, Error = TimuParserError<'a>>>(input: Span<'a>, parser: F, expr_func: ExprGeneratorFn<'a, T>) -> IResult<Span<'a>, ExpressionAst<'a>, TimuParserError<'a>> 
        where Vec<(T, ExpressionAst<'a>)>: Extend<(<F as Parser<Span<'a>>>::Output, ExpressionAst<'a>)>
    {
        let (input, initial) = P::parse(input)?;
        let (input, remainder): (Span<'_>, Vec<(T, ExpressionAst<'_>)>) = many(0.., pair(parser, Self::inner)).parse(input)?;
        Ok((input, Self::value_fold_exprs::<T>(initial, remainder, expr_func)))
    }
    
    pub fn and_or(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, left) = Self::less_greater(input)?;

        let (input, remainder): (Span<'_>, Vec<(ControlOperatorType, ExpressionAst<'_>)>) = many(
            0..,
            alt((
            |input| {
                let (input, expr) = preceded(tag("&&"), Self::inner).parse(input)?;
                Ok((input, (ControlOperatorType::And, expr)))
            },
            |input| {
                let (input, expr) = preceded(tag("||"), Self::inner).parse(input)?;
                Ok((input, (ControlOperatorType::Or, expr)))
            },
            )),
        )
        .parse(input)?;

        Ok((input, Self::control_fold_exprs(left, remainder)))
    }

    fn equal_not_equal(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, left) = Self::and_or(input)?;  

        let (input, remainder): (Span<'_>, Vec<(ControlOperatorType, ExpressionAst<'_>)>) = many(
            0..,
            alt((
            |input| {
                let (input, expr) = preceded(tag("=="), Self::inner).parse(input)?;
                Ok((input, (ControlOperatorType::Equal, expr)))
            },
            |input| {
                let (input, expr) = preceded(tag("!="), Self::inner).parse(input)?;
                Ok((input, (ControlOperatorType::NotEqual, expr)))
            },
            )),
        )
        .parse(input)?;

        Ok((input, Self::control_fold_exprs(left, remainder)))
    }

    fn less_greater(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, left) = Self::binary(input)?;  

        let (input, remainder): (Span<'_>, Vec<(ControlOperatorType, ExpressionAst<'_>)>) = many(
            0..,
            alt((
            |input| {
                let (input, expr) = preceded(tag(">"), Self::inner).parse(input)?;
                Ok((input, (ControlOperatorType::GreaterThan, expr)))
            },
            |input| {
                let (input, expr) = preceded(tag(">="), Self::inner).parse(input)?;
                Ok((input, (ControlOperatorType::GreaterEqualThan, expr)))
            },
            |input| {
                let (input, expr) = preceded(tag("<"), Self::inner).parse(input)?;
                Ok((input, (ControlOperatorType::LessThan, expr)))
            },
            |input| {
                let (input, expr) = preceded(tag("<="), Self::inner).parse(input)?;
                Ok((input, (ControlOperatorType::LessEqualThan, expr)))
            },
            )),
        )
        .parse(input)?;

        Ok((input, Self::control_fold_exprs(left, remainder)))
    }

    fn control_fold_exprs<'a>(initial: ExpressionAst<'a>, remainder: Vec<(ControlOperatorType, ExpressionAst<'a>)>) -> ExpressionAst<'a> {
        remainder.into_iter().fold(initial, |left, pair| {
          let (operator, right) = pair;
          ExpressionAst::ControlOperation {
            left: Box::new(left),
            operator,
            right: Box::new(right)
        }
        })
    }

    fn single_fold_exprs<'a, T: Copy>(initial: ExpressionAst<'a>, operator: T, remainder: Vec<ExpressionAst<'a>>, expr_func: ExprGeneratorFn<'a, T>) -> ExpressionAst<'a> {
        remainder.into_iter().fold(initial, |left, right| {
          expr_func(left, operator, right)
        })
    }

    fn value_fold_exprs<'a, T: Copy>(initial: ExpressionAst<'a>, remainder: Vec<(T, ExpressionAst<'a>)>, expr_func: ExprGeneratorFn<'a, T>) -> ExpressionAst<'a> {
        remainder.into_iter().fold(initial, |left, (operator, right)| {
          expr_func(left, operator, right)
        })
    }
}

