use std::fmt::{Display, Formatter};

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, map, opt, peek};
use nom::error::context;
use nom::multi::separated_list0;
use nom::sequence::terminated;
use nom::{IResult, Parser, sequence::delimited};
use nom_language::error::{VerboseError, VerboseErrorKind};

use crate::ast::{
    BodyAst, ClassDefinitionFieldAst, ExtendDefinitionFieldAst, FileStatementAst, FunctionArgumentAst, FunctionDefinitionAst, TypeNameAst
};
use crate::nom_tools::{Span, cleanup};
use crate::parser::{expected_ident, ident, is_public};

use super::TimuParserError;

impl FunctionDefinitionAst<'_> {
    pub fn parse_file_function(input: Span<'_>) -> IResult<Span<'_>, FileStatementAst<'_>, TimuParserError<'_>> {
        let (input, function) = Self::parse(input)?;
        Ok((input, FileStatementAst::Function(function)))
    }

    pub fn parse_class_function(input: Span<'_>) -> IResult<Span<'_>, ClassDefinitionFieldAst<'_>, TimuParserError<'_>> {
        let (input, function) = Self::parse(input)?;
        Ok((input, ClassDefinitionFieldAst::ClassFunction(function)))
    }

    pub fn parse_extend_function(input: Span<'_>) -> IResult<Span<'_>, ExtendDefinitionFieldAst<'_>, TimuParserError<'_>> {
        let (input, function) = Self::parse(input)?;
        if let Some(is_public) = function.is_public {
            let error = VerboseError {
                errors: vec![(is_public, VerboseErrorKind::Context("All extended functions already public"))],
            };
            return Err(nom::Err::Failure(error));
        }
        Ok((input, ExtendDefinitionFieldAst::Function(function)))
    }

    pub fn parse(
        input: Span<'_>,
    ) -> IResult<Span<'_>, FunctionDefinitionAst<'_>, TimuParserError<'_>> {
        let (input, is_public) = is_public(input)?;
        let (input, _) = cleanup(tag("func")).parse(input)?;
        let (input, name) = expected_ident("Missing function name", input)?;
        let (input, _) = context("Missing '('", cut(peek(cleanup(char('('))))).parse(input)?;
        let (input, arguments) =
            map(delimited(char('('), cleanup(separated_list0(char(','), FunctionArgumentAst::parse)), context("Missing ')'", cut(char(')')))), |items| {
                items
            })
            .parse(input)?;

        let (input, _) = context("Missing ':'", cleanup(opt(char(':')))).parse(input)?;
        let (input, return_type) = context("Missing function return type", cut(cleanup(cleanup(TypeNameAst::parse)))).parse(input)?;

        let (input, body) = BodyAst::parse(input)?;

        Ok((
            input,
            FunctionDefinitionAst {
                is_public,
                name,
                arguments,
                body,
                return_type,
            },
        ))
    }
}

impl Display for FunctionDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}func {}(", if self.is_public.is_some() { "pub " } else { "" }, self.name.fragment())?;
        for (index, arg) in self.arguments.iter().enumerate() {
            write!(f, "{}", arg)?;
            if index < self.arguments.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "): {} {}", self.return_type, self.body)
    }
}

impl FunctionArgumentAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, FunctionArgumentAst<'_>, TimuParserError<'_>> {
        let (input, (name, field_type)) = (cleanup(terminated(ident(), cleanup(char(':')))), cleanup(TypeNameAst::parse)).parse(input)?;
        Ok((
            input,
            FunctionArgumentAst {
                name,
                field_type,
            },
        ))
    }
}

impl Display for FunctionArgumentAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name.fragment(), self.field_type)
    }
}