use std::fmt::{Display, Formatter};

use nom::{character::complete::char, combinator::map, multi::separated_list0, IResult, Parser};

use crate::{ast::{FunctionCallPathAst, TypeNameAst}, nom_tools::Span};

use super::{ident, is_nullable, TimuParserError};


impl TypeNameAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, TypeNameAst<'_>, TimuParserError<'_>> {
        let (input, nullable) = is_nullable(input)?;
        let (input, names) = map(separated_list0(char('.'), ident()), |items| items).parse(input)?;
        Ok((
            input,
            TypeNameAst {
                nullable,
                names,
            },
        ))
    }

    pub fn parse_for_function_path(input: Span<'_>) -> IResult<Span<'_>, FunctionCallPathAst<'_>, TimuParserError<'_>> {
        let (input, path) = Self::parse(input)?;
        Ok((
            input,
            FunctionCallPathAst::TypeName(path),
        ))
    }
}

impl Display for TypeNameAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.nullable {
            write!(f, "?")?;
        }

        for (i, name) in self.names.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", name.fragment())?;
        }
        Ok(())
    }
}
