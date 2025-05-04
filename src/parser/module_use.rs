use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{consumed, cut};
use nom::error::context;
use nom::multi::separated_list1;
use nom::{IResult, Parser};

use crate::ast::{FileStatementAst, UseAst};
use crate::nom_tools::{Span, cleanup};
use crate::parser::ident;

use super::TimuParserError;

impl UseAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, UseAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup(tag("use")).parse(input)?;
        let (input, (import, splited_import)) = context("Module path missing", cut(consumed(cleanup(separated_list1(char('.'), ident()))))).parse(input)?;
        let import = match import.fragment().contains(char::is_whitespace) {
            true => {
                let path = splited_import.iter().map(|path| path.fragment().clone())
                .collect::<Vec<&str>>()
                .join(".");
                Cow::Owned(path)
            }
            false => Cow::Borrowed(import.fragment().clone())
        };
        
        let (input, _) = context("Missing ';'", cut(cleanup(char(';')))).parse(input)?;

        Ok((
            input,
            UseAst {
                import,
                splited_import,
            },
        ))
    }

    pub fn parse_for_file(input: Span<'_>) -> IResult<Span<'_>, FileStatementAst<'_>, TimuParserError<'_>> {
        let (input, import) = Self::parse(input)?;
        Ok((input, FileStatementAst::Use(import)))
    }
}

impl Display for UseAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "use ")?;
        for (i, path) in self.splited_import.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", path.fragment())?;
        }
        write!(f, ";")
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use nom::Finish;
    use rstest::rstest;

    use crate::{file::SourceFile, nom_tools::State};

    #[rstest]
    #[case("use test;", "use test;")]
    #[case(" use test ; ", "use test;")]
    #[case("use test1.test2;", "use test1.test2;")]
    #[case("use test1.test2.test3;", "use test1.test2.test3;")]
    #[case(r#"use foo1.foo2.foo3;
use bar1.bar2.bar3;"#, r#"use foo1.foo2.foo3;
use bar1.bar2.bar3;"#)]
    fn module_use_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), "<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let (_, response) = crate::parser::parse(state).finish().unwrap();
        assert_eq!(response.to_string(), expected, "{}", code);
    }
}