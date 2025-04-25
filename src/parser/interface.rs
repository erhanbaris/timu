use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, map, opt, peek};
use nom::error::context;
use nom::multi::{many0, separated_list0};
use nom::{IResult, Parser, error::ParseError, sequence::delimited};

use crate::ast::{FieldAst, FunctionArgumentAst, InterfaceDefinitionAst, InterfaceDefinitionFieldAst, InterfaceFunctionDefinitionAst, TypeNameAst};
use crate::{ast::FileStatementAst, nom_tools::{cleanup, Span}};

use super::expected_ident;

impl InterfaceDefinitionAst<'_> {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, FileStatementAst<'a>, E> {
        let (input, _) = cleanup(tag("interface")).parse(input)?;
        let (input, name) = expected_ident("Missing interface name", input)?;
        let (input, _) = context("Missing '{'", cut(peek(cleanup(char('{'))))).parse(input)?;
        let (input, fields) = delimited(
            char('{'),
            cleanup(many0(alt((
                InterfaceFunctionDefinitionAst::parse,
                FieldAst::parse_interface_field
            )))),
            context("Missing '}'", cut(char('}'))),
        )
        .parse(input)?;

        Ok((
            input,
            FileStatementAst::Interface(InterfaceDefinitionAst {
                name,
                fields,
            }),
        ))
    }
}

impl Display for InterfaceDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "interface {} {{", self.name.fragment())?;
        for field in self.fields.iter() {
            match field {
                InterfaceDefinitionFieldAst::Function(function) => {
                    write!(f, "{}", function)?;
                }
                InterfaceDefinitionFieldAst::Field(field) => {
                    write!(f, "{}", field)?;
                }
            }
        }
        write!(f, "}}")
    }
}

impl InterfaceFunctionDefinitionAst<'_> {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, InterfaceDefinitionFieldAst<'a>, E> {
        let (input, _) = cleanup(tag("func")).parse(input)?;
        let (input, name) = expected_ident("Missing function name", input)?;
        let (input, _) = context("Missing '('", cut(peek(cleanup(char('('))))).parse(input)?;
        let (input, arguments) =
            map(delimited(char('('), cleanup(separated_list0(char(','), FunctionArgumentAst::parse)), context("Missing ')'", cut(char(')')))), |items| items)
                .parse(input)?;

        let (input, _) = context("Missing ':'", cleanup(opt(char(':')))).parse(input)?;
        let (input, return_type) = context("Missing function return type", cut(cleanup(cleanup(TypeNameAst::parse)))).parse(input)?;
        let (input, _) = cleanup(char(';')).parse(input)?;

        Ok((
            input,
            InterfaceDefinitionFieldAst::Function(InterfaceFunctionDefinitionAst {
                name,
                arguments,
                return_type,
            }),
        ))
    }
}

impl Display for InterfaceFunctionDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "func {}(", self.name.fragment())?;
        for (index, arg) in self.arguments.iter().enumerate() {
            write!(f, "{}", arg)?;
            if index < self.arguments.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "): {};", self.return_type)
    }
}
