use std::fmt::{Display, Formatter};

use nom::{branch::alt, character::complete::char, combinator::cut, error::context, multi::many0, IResult, Parser};

use crate::{ast::{BodyAst, BodyStatementAst, FunctionCallAst, IfConditionAst, VariableAssignAst, VariableDefinitionAst}, nom_tools::{cleanup, Span}};

use super::TimuParserError;


impl BodyAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, BodyAst<'_>, TimuParserError<'_>> {
        let (input, _) = context("Body's opening '{' missing", cut(cleanup(char('{')))).parse(input)?;
        let (input, statements) = many0(alt((
            IfConditionAst::parse_body_statement,
            FunctionCallAst::parse_body_statement,
            VariableAssignAst::parse_body_statement,
            VariableDefinitionAst::parse_body_statement,
        ))).parse(input)?;
        let (input, _) = context("Body's closing '}' missing", cut(cleanup(char('}')))).parse(input)?;

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
                write!(f, " ")?;
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
            BodyStatementAst::FunctionCall(func) => write!(f, "{};", func),
            BodyStatementAst::IfCondition(if_condition) => write!(f, "{}", if_condition),
        }
    }
}
