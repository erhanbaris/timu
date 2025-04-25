use std::fmt::{Display, Formatter};

use nom::{branch::alt, character::complete::char, combinator::cut, error::{context, ParseError}, multi::many0, IResult, Parser};

use crate::{ast::{BodyAst, BodyStatementAst, VariableAssignAst, VariableDefinitionAst}, nom_tools::{cleanup, Span}};


impl BodyAst<'_> {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, BodyAst<'a>, E> {
        let (input, _) = context("Missing '{'", cut(cleanup(char('{')))).parse(input)?;
        let (input, statements) = many0(alt((VariableAssignAst::parse_body_statement::<E>, VariableDefinitionAst::parse_body_statement::<E>))).parse(input)?;
        let (input, _) = context("Missing '}'", cut(cleanup(char('}')))).parse(input)?;

        Ok((
            input,
            BodyAst {
                statements,
            },
        ))
    }
}

impl Display for BodyAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (index, statement) in self.statements.iter().enumerate() {
            write!(f, "{}", statement)?;
            if index < self.statements.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

impl Display for BodyStatementAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyStatementAst::VariableDefinition(var) => write!(f, "{}", var),
            BodyStatementAst::VariableAssign(var) => write!(f, "{}", var),
        }
    }
}
