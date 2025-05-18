use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, peek};
use nom::error::context;
use nom::multi::many0;
use nom::{IResult, Parser, sequence::delimited};

use crate::ast::{ClassDefinitionFieldAst, FieldAst, FunctionDefinitionAst};
use crate::{ast::{ClassDefinitionAst, FileStatementAst}, nom_tools::{cleanup, Span}};

use super::{expected_ident, TimuParserError};

impl ClassDefinitionAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, FileStatementAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup(tag("class")).parse(input)?;
        let (input, name) = expected_ident("Missing class name", input)?;
        let (input, _) = context("Class's opening '{' missing", cut(peek(cleanup(char('{'))))).parse(input)?;
        let (input, fields) = delimited(
            char('{'),
            cleanup(many0(alt((
                |input| {
                    FunctionDefinitionAst::parse_class_function(input, name.clone())
                },
                FieldAst::parse_class_field,
            )))),
            context("Class's closing '}' missing", cut(char('}'))),
        )
        .parse(input)?;

        Ok((
            input,
            FileStatementAst::Class(ClassDefinitionAst {
                name,
                fields,
            }.into()),
        ))
    }
}

impl Display for ClassDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {} {{", self.name.fragment())?;
        for field in self.fields.iter() {
            write!(f, "{}", field)?;
        }
        write!(f, "}}")
    }
}

impl Display for ClassDefinitionFieldAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassDefinitionFieldAst::ClassField(field) => write!(f, "{}", field),
            ClassDefinitionFieldAst::ClassFunction(function) => write!(f, "{}", function),
        }
    }
}
