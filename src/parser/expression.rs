use std::fmt::{Display, Formatter};

use nom::{branch::alt, IResult, Parser};

use crate::{ast::{ExpressionAst, FunctionCallAst, PrimitiveType}, nom_tools::{cleanup, Span}};

use super::{ident, TimuParserError};

impl ExpressionAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, expression) = cleanup(alt((
            FunctionCallAst::parse_for_expression,
            PrimitiveType::parse_for_expression,
            Self::ident_for_expression,
        ))).parse(input)?;

        Ok((input, expression))
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
        }
    }
}
