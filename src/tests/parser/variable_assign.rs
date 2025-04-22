use std::rc::Rc;

use nom_language::error::VerboseError;
use pretty_assertions::assert_eq;
use rstest::*;

use crate::ast::VariableAssignAst;
use crate::nom_tools::Span;
use crate::{file::SourceFile, nom_tools::State};

#[rstest]
#[case("a = false;", "a = false;")]
#[case("a = 100;", "a = 100;")]
#[case("a = 100;", "a = 100;")]
#[case("_a = 200;", "_a = 200;")]
#[case("a = 100;", "a = 100;")]
#[case("_123a = 100;", "_123a = 100;")]
#[case("a_123 = 100;", "a_123 = 100;")]
#[case("a_123_____ = 100;", "a_123_____ = 100;")]
#[case("a = 1.0;", "a = 1.0;")]
#[case("a = 1.2;", "a = 1.2;")]
#[case("a = -1.2;", "a = -1.2;")]
fn custom_variable_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
    let source_file = Rc::new(SourceFile::new("<memory>".into(), code));

    let state = State {
        file: source_file.clone(),
    };

    let input = Span::new_extra(code, state);
    let result = VariableAssignAst::parse::<VerboseError<Span>>(input);
    assert!(result.is_ok(), "Failed to parse type name: {:?}", result.err());
    let (_, parsed) = result.unwrap();

    assert_eq!(parsed.to_string(), expected);
}
