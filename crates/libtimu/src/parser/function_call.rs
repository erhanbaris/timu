use std::fmt::{Display, Formatter};

use nom::character::complete::char;
use nom::combinator::{consumed, cut, map, peek};
use nom::error::context;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::terminated;
use nom::{IResult, Parser, sequence::delimited};

use crate::ast::{
    BodyStatementAst, ExpressionAst, FunctionCallAst, FunctionCallPathAst
};
use crate::nom_tools::{Span, cleanup};
use crate::parser::ident;

use super::TimuParserError;

impl FunctionCallAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, FunctionCallAst<'_>, TimuParserError<'_>> {
        //let (input, this) = opt(tag("this")).parse(input)?;
        
        let (input, (call_span, paths)) = consumed(terminated(
            separated_list1(char('.'), ident()),
            peek(cleanup(char('('))))).parse(input)?;
        
        let (input, arguments) =
            map(delimited(char('('), cleanup(separated_list0(char(','), ExpressionAst::parse)), context("Missing ')'", cut(char(')')))), |items| {
                items
            })
            .parse(input)?;
        Ok((
            input,
            FunctionCallAst {
                call_span,
                paths,
                arguments,
            },
        ))
    }

    pub fn parse_body_statement(input: Span<'_>) -> IResult<Span<'_>, BodyStatementAst<'_>, TimuParserError<'_>> {
        let (input, function_call) = Self::parse(input)?;
        let (input, _) = context("Missing ';'", cleanup(char(';'))).parse(input)?;
        Ok((input, BodyStatementAst::FunctionCall(function_call)))
    }

    pub fn parse_for_expression(input: Span<'_>) -> IResult<Span<'_>, ExpressionAst<'_>, TimuParserError<'_>> {
        let (input, function_call) = Self::parse(input)?;
        Ok((input, ExpressionAst::FunctionCall(function_call)))
    }

    #[allow(dead_code)]
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
        /*
                match &self.path {
            FunctionCallType::This(paths) => {
                if paths.is_empty() {
                    "this".to_string()
                } else {
                    format!("this.{}", paths.iter().map(|p| *p.fragment()).collect::<Vec<_>>().join("."))
                }
            }
            FunctionCallType::Direct(paths) => paths.iter().map(|p| *p.fragment()).collect::<Vec<_>>().join("."),
        };
         */
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
    fn function_call_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = Rc::new(SourceFile::new(vec!["<memory>".into()], code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(state.file.code(), state);
        let (_, response) = FunctionCallAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{}", code);
    }
}