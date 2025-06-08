use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, peek};
use nom::error::context;
use nom::multi::many0;
use nom::{IResult, Parser, sequence::delimited};

use crate::ast::{AstIndex, ClassDefinitionFieldAst, FieldAst, FunctionDefinitionAst};
use crate::{ast::{ClassDefinitionAst, FileStatementAst}, nom_tools::{cleanup, NomSpan}};

use super::{expected_ident, TimuParserError};

impl ClassDefinitionAst<'_> {
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, FileStatementAst<'_>, TimuParserError<'_>> {
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
        let index = AstIndex(input.extra.indexer.fetch_add(1, std::sync::atomic::Ordering::Relaxed));

        Ok((
            input,
            FileStatementAst::Class(ClassDefinitionAst {
                name: name.into(),
                fields,
                index
            }.into()),
        ))
    }
}

impl Display for ClassDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {} {{", self.name.text)?;
        for field in self.fields.iter() {
            write!(f, "{}", field)?;
        }
        write!(f, "}}")
    }
}

impl Display for ClassDefinitionFieldAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassDefinitionFieldAst::Field(field) => write!(f, "{}", field),
            ClassDefinitionFieldAst::Function(function) => write!(f, "{}", function),
        }
    }
}
