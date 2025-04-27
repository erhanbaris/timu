use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, map, peek};
use nom::error::context;
use nom::multi::{separated_list0, separated_list1};
use nom::{IResult, Parser, sequence::delimited};

use crate::ast::{
    BodyStatementAst, ExpressionAst, FunctionCallAst, FunctionCallPathAst, TypeNameAst
};
use crate::nom_tools::{Span, cleanup};
use crate::parser::ident;

use super::TimuParserError;

impl FunctionCallAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, FunctionCallAst<'_>, TimuParserError<'_>> {
        let (input, paths) = separated_list1(tag("."), 
            alt((
                TypeNameAst::parse_for_function_path,
                Self::ident_for_function_path
            ))
        ).parse(input)?;
        let (input, _) = peek(cleanup(char('('))).parse(input)?;
        let (input, arguments) =
            map(delimited(char('('), cleanup(separated_list0(char(','), ExpressionAst::parse)), context("Missing ')'", cut(char(')')))), |items| {
                items
            })
            .parse(input)?;
        Ok((
            input,
            FunctionCallAst {
                paths,
                arguments,
            },
        ))
    }

    pub fn parse_body_statement(input: Span<'_>) -> IResult<Span<'_>, BodyStatementAst<'_>, TimuParserError<'_>> {
        let (input, function_call) = Self::parse(input)?;
        Ok((input, BodyStatementAst::FunctionCall(function_call)))
    }

    pub fn parse_for_expression(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst<'_>, TimuParserError<'_>> {
        let (input, function_call) = Self::parse(input)?;
        Ok((input, ExpressionAst::FunctionCall(function_call)))
    }

    fn ident_for_function_path(input: Span<'_>) -> IResult<Span<'_>, FunctionCallPathAst<'_>, TimuParserError<'_>> {
        let (input, path) = ident().parse(input)?;
        Ok((
            input,
            FunctionCallPathAst::Ident(path),
        ))
    }
}

impl Display for FunctionCallAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, path) in self.paths.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", path)?;
        }
        write!(f, "(")?;
        for (i, arg) in self.arguments.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl Display for FunctionCallPathAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionCallPathAst::Ident(ident) => write!(f, "{}", ident.fragment()),
            FunctionCallPathAst::TypeName(type_name) => write!(f, "{}", type_name),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use rstest::rstest;

    use crate::{ast::FunctionCallAst, file::SourceFile, nom_tools::{Span, State}};

    #[rstest]
    #[case("test()", "test()")]
    #[case("  test   (   )   ", "test()")]
    #[case(" test1. test2   (   )   ", "test1.test2()")]
    #[case(" this. test2   (  a,b,c )   ", "this.test2(a, b, c)")]
    #[case("a(b())", "a(b())")]
    #[case("  a ( b ( ) ) ", "a(b())")]
    #[case("a(b(1,2,3,4,true,false,1.2,2.2, c()))", "a(b(1, 2, 3, 4, true, false, 1.2, 2.2, c()))")]
    fn function_call_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(state.file.code(), state);
        let (_, response) = FunctionCallAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{}", code);
    }
}