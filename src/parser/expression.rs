use std::fmt::{Display, Formatter};

use nom::{branch::alt, character::complete::char, multi::many, sequence::{delimited, preceded}, IResult, Parser};

use crate::{ast::{ExpressionAst, FunctionCallAst, OperatorType, PrimitiveType}, nom_tools::{cleanup, Span}};

use super::{ident, TimuParserError};

impl ExpressionAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, left) = Self::term(input)?;

        let (input, remainder): (Span<'_>, Vec<(OperatorType, ExpressionAst<'_>)>) = many(
            0..,
            alt((
            |input| {
                let (input, expr) = preceded(char('+'), Self::factor).parse(input)?;
                Ok((input, (OperatorType::Add, expr)))
            },
            |input| {
                let (input, expr) = preceded(char('-'), Self::factor).parse(input)?;
                Ok((input, (OperatorType::Sub, expr)))
            },
            )),
        )
        .parse(input)?;

        Ok((input, Self::fold_exprs(left, remainder)))
    }

    fn fold_exprs<'a>(initial: ExpressionAst<'a>, remainder: Vec<(OperatorType, ExpressionAst<'a>)>) -> ExpressionAst<'a> {
        remainder.into_iter().fold(initial, |left, pair| {
          let (operator, right) = pair;
          ExpressionAst::BinaryOperation {
            left: Box::new(left),
            operator,
            right: Box::new(right)
        }
        })
    }

    pub fn parentheses(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, expr) = delimited(char('('), cleanup(Self::parse), char(')')).parse(input)?;
        Ok((input, expr))
    }
    
    pub fn factor(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, expression) = cleanup(alt((
            FunctionCallAst::parse_for_expression,
            PrimitiveType::parse_for_expression,
            Self::ident_for_expression,
            Self::parentheses,
        ))).parse(input)?;

        Ok((input, expression))
    }

    pub fn term(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, left) = Self::factor(input)?;  

        let (input, remainder): (Span<'_>, Vec<(OperatorType, ExpressionAst<'_>)>) = many(
            0..,
            alt((
            |input| {
                let (input, expr) = preceded(char('*'), Self::factor).parse(input)?;
                Ok((input, (OperatorType::Mul, expr)))
            },
            |input| {
                let (input, expr) = preceded(char('/'), Self::factor).parse(input)?;
                Ok((input, (OperatorType::Div, expr)))
            },
            )),
        )
        .parse(input)?;

        Ok((input, Self::fold_exprs(left, remainder)))
    }

    fn ident_for_expression(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst<'_>, TimuParserError<'_>> {
        let (input, ident) = ident().parse(input)?;
        Ok((
            input,
            ExpressionAst::Ident(ident),
        ))
    }
}

impl Display for ExpressionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionAst::Primitive(primitive) => write!(f, "{}", primitive),
            ExpressionAst::Ident(ident) => write!(f, "{}", ident),
            ExpressionAst::FunctionCall(function_call) => write!(f, "{}", function_call),
            ExpressionAst::BinaryOperation { left, operator, right } => {
                write!(f, "({} {} {})", left, operator, right)
            },
        }
    }
}

impl Display for OperatorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorType::Add => write!(f, "+"),
            OperatorType::Sub => write!(f, "-"),
            OperatorType::Mul => write!(f, "*"),
            OperatorType::Div => write!(f, "/"),
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
        let source_file = Rc::new(SourceFile::new("<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(state.file.code(), state);
        let (_, response) = ExpressionAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{}", code);
    }
}
