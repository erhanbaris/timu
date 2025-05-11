use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, peek};
use nom::error::context;
use nom::multi::{many0, separated_list1};
use nom::{IResult, Parser, sequence::delimited};

use crate::ast::{ExtendDefinitionAst, ExtendDefinitionFieldAst, FieldAst, FunctionDefinitionAst, TypeNameAst};
use crate::{ast::FileStatementAst, nom_tools::{cleanup, Span}};

use super::{expected_ident, TimuParserError};

impl ExtendDefinitionAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, FileStatementAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup(tag("extend")).parse(input)?;
        let (input, name) = expected_ident("Missing class name", input)?;
        let (input, _) = context("Missing ':'", cut(cleanup(char(':')))).parse(input)?;
        let (input, base_interfaces) = context("Missing interface type(s)", cut(separated_list1(tag(","), TypeNameAst::parse))).parse(input)?;

        let (input, _) = context("Extend's opening '{' missing", cut(peek(cleanup(char('{'))))).parse(input)?;
        let (input, fields) = delimited(
            char('{'),
            cleanup(many0(alt((
                FunctionDefinitionAst::parse_extend_function,
                FieldAst::parse_extend_field
            )))),
            context("Extend's closing '}' missing", cut(char('}'))),
        )
        .parse(input)?;

        Ok((
            input,
            FileStatementAst::Extend(ExtendDefinitionAst {
                name,
                fields,
                base_interfaces,
            }),
        ))
    }
}

impl Display for ExtendDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "extend {}", self.name.fragment())?;

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
                ExtendDefinitionFieldAst::Function(function) => {
                    write!(f, "{}", function)?;
                }
                ExtendDefinitionFieldAst::Field(field) => {
                    write!(f, "{}", field)?;
                }
            }
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use nom::Finish;
    use nom_language::error::VerboseErrorKind;
    use rstest::rstest;

    use crate::{file::SourceFile, nom_tools::State};


    #[rstest]
    #[case("extend Myclass: a {}", "extend Myclass: a {}")]
    #[case("    extend     Myclass : a,b   \r\n\t{} ", "extend Myclass: a, b {}")]
    #[case("    extend     Myclass: a    \r\n\t{\r\n\t} ", "extend Myclass: a {}")]
    #[case("extend ___MyType___: a {}", "extend ___MyType___: a {}")]
    #[case("extend Myclass: a { a: string; }", "extend Myclass: a {a: string;}")]
    #[case("extend Myclass: a { \r\n\ta\r\n\t: \r\n\tstring ;\r\n\t}", "extend Myclass: a {a: string;}")]
    #[case("extend Myclass: a { \r\n\t\r\n\t\r\n\t\r\n\t}", "extend Myclass: a {}")]
    #[case("extend Myclass: a { a: string; }", "extend Myclass: a {a: string;}")]
    #[case("extend Myclass: a { a: ?string; }", "extend Myclass: a {a: ?string;}")]
    #[case("extend Myclass: a { a: ?string; }", "extend Myclass: a {a: ?string;}")]
    #[case("extend Myclass: a { a: ?string.base; }", "extend Myclass: a {a: ?string.base;}")]
    #[case("extend Myclass: a { a: string; b: string; }", "extend Myclass: a {a: string;b: string;}")]
    #[case("extend Myclass: a { func init(): MyType {} }", "extend Myclass: a {func init(): MyType {}}")]
    #[case("extend Myclass: a { func init(): MyType {} func init(): MyType {} }", "extend Myclass: a {func init(): MyType {}func init(): MyType {}}")]
    #[case(
        "extend Myclass: a { a: ?string.base; func init(): MyType {} func init(): MyType {} }",
        "extend Myclass: a {a: ?string.base;func init(): MyType {}func init(): MyType {}}"
    )]
    fn extend_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = Rc::new(SourceFile::new(vec!["<memory>".into()], code));

        let state = State {
            file: source_file.clone(),
        };

        let (_, response) = crate::parser::parse(state).finish().unwrap();
        assert_eq!(response.statements[0].to_string(), expected, "{}", code);
    }

    #[rstest]
    #[case("extend Myclass: a { pub a: string; }", "All extended fields already public")]
    #[case("extend Myclass: a { pub func init(): MyType {} }", "All extended functions already public")]
    fn alread_public<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = Rc::new(SourceFile::new(vec!["<memory>".into()], code));

        let state = State {
            file: source_file.clone(),
        };

        let error = crate::parser::parse(state).finish().unwrap_err();
        if let VerboseErrorKind::Context(ctx) = error.errors[0].1 {
            assert_eq!(ctx, expected, "{}", code);
        } else {
            panic!("Expected an error, but got: {:?}", error);
        }
    }
}
