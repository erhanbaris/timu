use nom::{branch::alt, character::complete::char, multi::many, sequence::preceded, IResult, Parser};

use crate::{ast::{BinaryOperatorType, ExpressionAst}, nom_tools::Span};

use super::TimuParserError;

impl ExpressionAst<'_> {
    pub fn binary(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        Self::add_sub(input)
    }
    
    pub fn add_sub(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, left) = Self::mul_div(input)?;

        let (input, remainder): (Span<'_>, Vec<(BinaryOperatorType, ExpressionAst<'_>)>) = many(
            0..,
            alt((
            |input| {
                let (input, expr) = preceded(char('+'), Self::inner).parse(input)?;
                Ok((input, (BinaryOperatorType::Add, expr)))
            },
            |input| {
                let (input, expr) = preceded(char('-'), Self::inner).parse(input)?;
                Ok((input, (BinaryOperatorType::Sub, expr)))
            },
            )),
        )
        .parse(input)?;

        Ok((input, Self::binary_fold_exprs(left, remainder)))
    }

    fn binary_fold_exprs<'a>(initial: ExpressionAst<'a>, remainder: Vec<(BinaryOperatorType, ExpressionAst<'a>)>) -> ExpressionAst<'a> {
        remainder.into_iter().fold(initial, |left, pair| {
          let (operator, right) = pair;
          ExpressionAst::BinaryOperation {
            left: Box::new(left),
            operator,
            right: Box::new(right)
        }
        })
    }

    fn mul_div(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, left) = Self::inner(input)?;  

        let (input, remainder): (Span<'_>, Vec<(BinaryOperatorType, ExpressionAst<'_>)>) = many(
            0..,
            alt((
            |input| {
                let (input, expr) = preceded(char('*'), Self::inner).parse(input)?;
                Ok((input, (BinaryOperatorType::Mul, expr)))
            },
            |input| {
                let (input, expr) = preceded(char('/'), Self::inner).parse(input)?;
                Ok((input, (BinaryOperatorType::Div, expr)))
            },
            )),
        )
        .parse(input)?;

        Ok((input, Self::binary_fold_exprs(left, remainder)))
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
