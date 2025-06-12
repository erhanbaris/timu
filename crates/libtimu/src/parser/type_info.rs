use std::fmt::{Display, Formatter};

use nom::{character::complete::char, combinator::{consumed, map}, multi::separated_list1, IResult, Parser};

use crate::{ast::{FunctionCallPathAst, TypeNameAst}, nom_tools::NomSpan, parser::is_reference};

use super::{ident, is_nullable, TimuParserError};


impl TypeNameAst<'_> {
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, TypeNameAst<'_>, TimuParserError<'_>> {
        let (input, reference) = is_reference(input)?;
        let (input, nullable) = is_nullable(input)?;
        let (input, (names_span, names)) = consumed(map(separated_list1(char('.'), ident()), |items| items)).parse(input)?;
        Ok((
            input,
            TypeNameAst {
                reference,
                nullable,
                names: names.into_iter().map(|item| item.into()).collect::<Vec<_>>(),
                names_span: names_span.into(),
            },
        ))
    }

    pub fn parse_for_function_path(input: NomSpan<'_>) -> IResult<NomSpan<'_>, FunctionCallPathAst<'_>, TimuParserError<'_>> {
        let (input, path) = Self::parse(input)?;
        Ok((
            input,
            FunctionCallPathAst::TypeName(path),
        ))
    }
}

impl Display for TypeNameAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.reference {
            write!(f, "ref ")?;
        }

        if self.nullable {
            write!(f, "?")?;
        }

        for (i, name) in self.names.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", name.text)?;
        }
        Ok(())
    }
}
