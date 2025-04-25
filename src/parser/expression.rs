use std::fmt::{Display, Formatter};

use nom::{branch::alt, error::ParseError, IResult, Parser};

use crate::{ast::{ExpressionAst, PrimitiveType}, nom_tools::{cleanup, Span}};

impl ExpressionAst {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, ExpressionAst, E> {
        let (input, expression) = alt((cleanup(PrimitiveType::parse),)).parse(input)?;

        Ok((input, ExpressionAst::Primitive(expression)))
    }
}

impl Display for ExpressionAst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionAst::Primitive(primitive) => write!(f, "{}", primitive),
        }
    }
}
