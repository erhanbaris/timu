use std::fmt::{Display, Formatter};

use nom::character::complete::char;
use nom::sequence::terminated;
use nom::{IResult, Parser};
use nom_language::error::{VerboseError, VerboseErrorKind};

use crate::ast::{ClassDefinitionFieldAst, ExtendDefinitionFieldAst, FieldAst, InterfaceDefinitionFieldAst, TypeNameAst};
use crate::nom_tools::{cleanup, Span};

use super::{ident, is_public, TimuParserError};

impl FieldAst<'_> {
    pub fn parse_field<'a>(input: Span<'a>) -> IResult<Span<'a>, FieldAst<'a>, TimuParserError<'a>> {
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

    pub fn parse_class_field<'a>(input: Span<'a>) -> IResult<Span<'a>, ClassDefinitionFieldAst<'a>, TimuParserError<'a>> {
        let (input, field) = Self::parse_field(input)?;
        Ok((input, ClassDefinitionFieldAst::ClassField(field)))
    }

    pub fn parse_interface_field<'a>(input: Span<'a>) -> IResult<Span<'a>, InterfaceDefinitionFieldAst<'a>, TimuParserError<'a>> {
        let (input, field) = Self::parse_field(input)?;
        Ok((input, InterfaceDefinitionFieldAst::Field(field)))
    }

    pub fn parse_extend_field<'a>(input: Span<'a>) -> IResult<Span<'a>, ExtendDefinitionFieldAst<'a>, TimuParserError<'a>> {
        let (input, field) = Self::parse_field(input)?;
        if let Some(is_public) = field.is_public {
            let error = VerboseError {
                errors: vec![(is_public, VerboseErrorKind::Context("All extended fields already public"))],
            };
            return Err(nom::Err::Failure(error));
        }
        
        Ok((input, ExtendDefinitionFieldAst::Field(field)))
    }
}

impl Display for FieldAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}: {};",
            match self.is_public {
                Some(_) => "pub ",
                None => "",
            },
            self.name.fragment(),
            self.field_type
        )
    }
}
