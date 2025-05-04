use std::fmt::{Display, Formatter};

use nom::bytes::complete::tag;
use nom::combinator::{cut, opt};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::{IResult, Parser};

use crate::ast::{BodyAst, BodyStatementAst, ExpressionAst, IfConditionAst};
use crate::nom_tools::{cleanup, Span};

use super::TimuParserError;

impl IfConditionAst<'_> {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, IfConditionAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup(tag("if")).parse(input)?;
        let (input, expression) = context("Missing if expression", cut(ExpressionAst::parse)).parse(input)?;
        let (input, true_body) = context("Missing true body", cut(BodyAst::parse)).parse(input)?;

        let (input, else_ifs) = many0(
            preceded(
                (cleanup(tag("else")), cleanup(tag("if"))), 
                (context("Missing 'else if' expression", cut(ExpressionAst::parse)), context("Missing 'else if' body", cut(BodyAst::parse)))
            )
        ).parse(input)?;
        let (input, false_body) = match cleanup(opt(tag("else"))).parse(input)? {
            (input, Some(_)) => {
                let (input, false_body) = context("Missing false body", cut(BodyAst::parse)).parse(input)?;
                (input, Some(false_body))
            },
            (input, None) => (input, None),
        };

        Ok((
            input,
            IfConditionAst {
                expression,
                true_body,
                else_ifs,
                false_body,
            },
        ))
    }

    pub fn parse_body_statement(input: Span<'_>) -> IResult<Span<'_>, BodyStatementAst<'_>, TimuParserError<'_>> {
        let (input, if_condition) = Self::parse(input)?;
        Ok((input, BodyStatementAst::IfCondition(if_condition)))
    }
}

impl Display for IfConditionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {}", self.expression, self.true_body)?;
        for (expression, body) in self.else_ifs.iter() {
            write!(f, " else if {} {}", expression, body)?;
        }
        if let Some(false_body) = &self.false_body {
            write!(f, " else {}", false_body)?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::rc::Rc;

    use rstest::rstest;

    use crate::{
        ast::IfConditionAst,
        file::SourceFile, nom_tools::State,
    };

    use super::Span;

    #[rstest]
    #[case(r#"if true {}"#, r#"if true {}"#)]
    #[case(r#"if true {if true {if true {if true {}}}}"#, r#"if true {if true {if true {if true {}}}}"#)]
    #[case(r#"if call(true) {}"#, r#"if call(true) {}"#)]
    #[case(r#"if call(true) { var a = 1;var a = 1;var a = 1;}"#, r#"if call(true) {var a = 1; var a = 1; var a = 1;}"#)]
    #[case(r#"if call(true) + 20 == 100 {}"#, r#"if ((call(true) + 20) == 100) {}"#)]
    #[case(r#"if true || false {}"#, r#"if (true || false) {}"#)]
    #[case(r#"if true || false {} else {}"#, r#"if (true || false) {} else {}"#)]
    #[case(r#"if true || false {var a = 20;} else {var b = 30;}"#, r#"if (true || false) {var a = 20;} else {var b = 30;}"#)]
    #[case(r#"if true || false {}
else if false {}
"#, r#"if (true || false) {} else if false {}"#)]
    #[case(r#"if true || false {}
    else if false {}
    else if false {}
    else if false {}
    else {}
    "#, r#"if (true || false) {} else if false {} else if false {} else if false {} else {}"#)]
    fn if_condition_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
        let source_file = Rc::new(SourceFile::new("<memory>", "<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(state.file.code(), state);
        let (_, response) = IfConditionAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{}", code);
    }
}
