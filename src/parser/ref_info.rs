use std::fmt::{Display, Formatter};

use nom::{character::complete::char, combinator::{cut, not}, error::context, multi::separated_list1, IResult, Parser};

use crate::{ast::{ExpressionAst, RefAst}, nom_tools::{cleanup, Span}};

use super::{ident, TimuParserError};

impl RefAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, RefAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup((char('&'), not(char('&')))).parse(input)?;
        let (input, names) = context("Reference name missing", cut(separated_list1(cleanup(char('.')), ident()))).parse(input)?;
        Ok((
            input,
            RefAst {
                names,
            },
        ))
    }

    pub fn parse_for_expression(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst<'_>, TimuParserError<'_>> {
        let (input, path) = Self::parse(input)?;
        Ok((
            input,
            ExpressionAst::Ref(path),
        ))
    }
}

impl Display for RefAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "&")?;

        for (i, name) in self.names.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", name.fragment())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use rstest::rstest;

    use crate::{ast::RefAst, file::SourceFile, nom_tools::{Span, State}};

    #[rstest]
    #[case("&erha_1n", "&erha_1n")]
    #[case("&a.b", "&a.b")]
    #[case(" & a ", "&a")]
    #[case("&a . b  ", "&a.b")]
    fn reference_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
        let source_file = Rc::new(SourceFile::new(vec!["<memory>".into()], code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(state.file.code(), state);
        let (_, response) = RefAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{}", code);
    }
}