use std::fmt::{Display, Formatter};

use nom::{character::complete::char, combinator::not, IResult, Parser};

use crate::{ast::{ExpressionAst, RefAst}, nom_tools::{cleanup, NomSpan}};

use super::TimuParserError;

impl RefAst<'_> {
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, RefAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup((char('&'), not(char('&')))).parse(input)?;
        let (input, ast) = ExpressionAst::parse(input)?;
        Ok((
            input,
            RefAst {
                expression: ast.into(),
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
        write!(f, "&")?;
        write!(f, "{}", self.expression)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{ast::RefAst, file::SourceFile, nom_tools::{NomSpan, State}};

    #[rstest]
    #[case("&erha_1n", "&erha_1n")]
    #[case("&a.b", "&a.b")]
    #[case(" & a ", "&a")]
    #[case("&a . b  ", "&a.b")]
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