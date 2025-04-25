use std::fmt::{Display, Formatter};

use nom::character::complete::char;
use nom::sequence::terminated;
use nom::{IResult, Parser, error::ParseError};

use crate::ast::{ClassDefinitionFieldAst, FieldAst, InterfaceDefinitionFieldAst, TypeNameAst};
use crate::nom_tools::{cleanup, Span};

use super::{ident, is_public};

impl FieldAst<'_> {
    pub fn parse_field<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, FieldAst<'a>, E> {
        let (input, (is_public, name, field_type, _)) =
            (is_public, cleanup(terminated(ident(), cleanup(char(':')))), cleanup(TypeNameAst::parse), cleanup(char(';'))).parse(input)?;

        Ok((
            input,
            FieldAst {
                is_public,
                name,
                field_type,
            },
        ))
    }

    pub fn parse_class_field<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, ClassDefinitionFieldAst<'a>, E> {
        let (input, field) = Self::parse_field(input)?;
        Ok((input, ClassDefinitionFieldAst::ClassField(field)))
    }

    pub fn parse_interface_field<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, InterfaceDefinitionFieldAst<'a>, E> {
        let (input, field) = Self::parse_field(input)?;
        Ok((input, InterfaceDefinitionFieldAst::Field(field)))
    }
}

impl Display for FieldAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}: {};",
            match self.is_public {
                true => "pub ",
                false => "",
            },
            self.name.fragment(),
            self.field_type
        )
    }
}
