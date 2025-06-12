use std::fmt::{Display, Formatter};

use nom::{bytes::complete::tag, character::complete::char, combinator::cut, error::context, multi::separated_list1, IResult, Parser};

use crate::{ast::{ExpressionAst, RefAst}, nom_tools::{cleanup, NomSpan, Span}};

use super::{ident, TimuParserError};

impl RefAst<'_> {
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, RefAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup(tag("ref")).parse(input)?;
        let (input, names) = context("Reference name missing", cut(separated_list1(cleanup(char('.')), ident()))).parse(input)?;
        Ok((
            input,
            RefAst {
                names: names.into_iter().map(Span::from).collect::<Vec<_>>(),
            },
        ))
    }

    pub fn parse_for_expression(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst<'_>, TimuParserError<'_>> {
        let (input, path) = Self::parse(input)?;
        Ok((
            input,
            ExpressionAst::Ref(path),
        ))
    }
}

impl Display for RefAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ref ")?;

        for (i, name) in self.names.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", name.text)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{ast::RefAst, file::SourceFile, nom_tools::{NomSpan, State}};

    #[rstest]
    #[case("ref erha_1n", "ref erha_1n")]
    #[case("ref a.b", "ref a.b")]
    #[case(" ref  a ", "ref a")]
    #[case("ref a . b  ", "ref a.b")]
    fn reference_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(source_file.code().as_str(), state);
        let (_, response) = RefAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{}", code);
    }
}