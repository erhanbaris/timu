use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, map, opt};
use nom::error::context;
use nom::{IResult, Parser, error::ParseError};

use crate::ast::{BodyStatementAst, ExpressionAst, TypeNameAst, VariableAssignAst, VariableDefinitionAst, VariableDefinitionType};
use crate::nom_tools::{Span, cleanup};
use crate::parser::{expected_ident, ident};

impl VariableDefinitionAst<'_> {
    pub fn parse_body_statement<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, BodyStatementAst<'a>, E> {
        let (input, variable) = Self::parse(input)?;
        Ok((input, BodyStatementAst::VariableDefinition(variable)))
    }

    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, VariableDefinitionAst<'a>, E> {
        let (input, variable_definition_type) =
            cleanup(alt((map(tag("var"), |_| VariableDefinitionType::Var), map(tag("const"), |_| VariableDefinitionType::Const)))).parse(input)?;
        let (input, name) = expected_ident("Missing variable name", input)?;

        let (input, expected_type) = match cleanup(opt(char(':'))).parse(input)? {
            (input, Some(_)) => {
                let (input, expected_type) = context("Missing variable type", cut(cleanup(TypeNameAst::parse))).parse(input)?;
                (input, Some(expected_type))
            }
            (input, None) => (input, None),
        };

        let (input, _) = context("Missing '='", cleanup(char('='))).parse(input)?;
        let (input, expression) = context("Invalid expression", cut(ExpressionAst::parse)).parse(input)?;
        let (input, _) = context("Missing ';'", cleanup(char(';'))).parse(input)?;

        Ok((
            input,
            VariableDefinitionAst {
                variable_definition_type,
                name,
                expected_type,
                expression,
            },
        ))
    }
}

impl Display for VariableDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}{} = {};",
            self.variable_definition_type,
            self.name.fragment(),
            match &self.expected_type {
                Some(expected_type) => format!(": {}", expected_type),
                None => "".to_string(),
            },
            self.expression
        )
    }
}

impl VariableAssignAst<'_> {
    pub fn parse_body_statement<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, BodyStatementAst<'a>, E> {
        let (input, variable) = Self::parse(input)?;
        Ok((input, BodyStatementAst::VariableAssign(variable)))
    }

    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, VariableAssignAst<'a>, E> {
        let (input, name) = ident().parse(input)?;
        let (input, _) = context("Missing '='", cleanup(char('='))).parse(input)?;
        let (input, expression) = context("Invalid expression", cut(ExpressionAst::parse)).parse(input)?;
        let (input, _) = context("Missing ';'", cleanup(char(';'))).parse(input)?;

        Ok((
            input,
            VariableAssignAst {
                name,
                expression,
            },
        ))
    }
}

impl Display for VariableAssignAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {};", self.name.fragment(), self.expression)
    }
}

impl Display for VariableDefinitionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableDefinitionType::Var => write!(f, "var"),
            VariableDefinitionType::Const => write!(f, "const"),
        }
    }
}
