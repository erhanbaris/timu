use std::fmt::{Display, Formatter};

use nom::{branch::alt, IResult, Parser};

use crate::{ast::{ExpressionAst, PrimitiveType}, nom_tools::{cleanup, Span}};

use super::TimuParserError;

impl ExpressionAst {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
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
