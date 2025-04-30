use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, map, opt, peek};
use nom::error::context;
use nom::multi::{many0, separated_list0};
use nom::{IResult, Parser, sequence::delimited};

use crate::ast::{FieldAst, FunctionArgumentAst, InterfaceDefinitionAst, InterfaceDefinitionFieldAst, InterfaceFunctionDefinitionAst, TypeNameAst};
use crate::{ast::FileStatementAst, nom_tools::{cleanup, Span}};

use super::{expected_ident, TimuParserError};

impl InterfaceDefinitionAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, FileStatementAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup(tag("interface")).parse(input)?;
        let (input, name) = expected_ident("Missing interface name", input)?;

        let (input, base_interfaces) = match cleanup(opt(char(':'))).parse(input)? {
            (input, Some(_)) => {
                let (input, base_interfaces) = context("Missing variable type", cut(separated_list0(tag(","), TypeNameAst::parse))).parse(input)?;
                (input, base_interfaces)
            }
            (input, None) => (input, vec![]),
        };

        let (input, _) = context("Interface's opening '{' missing", cut(peek(cleanup(char('{'))))).parse(input)?;
        let (input, fields) = delimited(
            char('{'),
            cleanup(many0(alt((
                InterfaceFunctionDefinitionAst::parse,
                FieldAst::parse_interface_field
            )))),
            context("Interface's closing '}' missing", cut(char('}'))),
        )
        .parse(input)?;

        Ok((
            input,
            FileStatementAst::Interface(InterfaceDefinitionAst {
                name,
                fields,
                base_interfaces,
            }),
        ))
    }
}

impl Display for InterfaceDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "interface {}", self.name.fragment())?;

        if !self.base_interfaces.is_empty() {
            write!(f, ": ")?;
            for (index, base_interface) in self.base_interfaces.iter().enumerate() {
                write!(f, "{}", base_interface)?;
                if index < self.base_interfaces.len() - 1 {
                    write!(f, ", ")?;
                }
            }
        }

        write!(f, " {{")?;
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
    pub fn parse(
        input: Span<'_>,
    ) -> IResult<Span<'_>, InterfaceDefinitionFieldAst<'_>, TimuParserError<'_>> {
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
