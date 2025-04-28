use std::fmt::{Display, Formatter};

use nom::{branch::alt, character::complete::char, sequence::delimited, IResult, Parser};

use crate::{ast::{BinaryOperatorType, ControlOperatorType, ExpressionAst, FunctionCallAst, PrimitiveType}, nom_tools::{cleanup, Span}};

use super::{ident, TimuParserError};

impl ExpressionAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        Self::control(input)
    }

    pub fn inner(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, expression) = cleanup(alt((
            FunctionCallAst::parse_for_expression,
            PrimitiveType::parse_for_expression,
            Self::ident_for_expression,
            Self::parentheses,
        ))).parse(input)?;

        Ok((input, expression))
    }

    pub fn parentheses(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst, TimuParserError<'_>> {
        let (input, expr) = delimited(char('('), cleanup(Self::parse), char(')')).parse(input)?;
        Ok((input, expr))
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
            ExpressionAst::ControlOperation { left, operator, right } => {
                write!(f, "({} {} {})", left, operator, right)
            },
        }
    }
}

impl Display for BinaryOperatorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperatorType::Add => write!(f, "+"),
            BinaryOperatorType::Sub => write!(f, "-"),
            BinaryOperatorType::Mul => write!(f, "*"),
            BinaryOperatorType::Div => write!(f, "/"),
        }
    }
}

impl Display for ControlOperatorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlOperatorType::And => write!(f, "&&"),
            ControlOperatorType::Or => write!(f, "||"),
            ControlOperatorType::Not => write!(f, "!"),
            ControlOperatorType::Equal => write!(f, "=="),
            ControlOperatorType::NotEqual => write!(f, "!="),
            ControlOperatorType::LessThan => write!(f, "<"),
            ControlOperatorType::GreaterThan => write!(f, ">"),
            ControlOperatorType::GreaterEqualThan => write!(f, ">="),
            ControlOperatorType::LessEqualThan => write!(f, "<="),
            ControlOperatorType::Xor => write!(f, "^"),
            ControlOperatorType::LogicalOr => write!(f, "|"),
            ControlOperatorType::LogicalAnd => write!(f, "&"),
            ControlOperatorType::BitwiseShiftLeft => write!(f, "<<"),
            ControlOperatorType::BitwiseShiftRight => write!(f, ">>"),
        }
    }
}
