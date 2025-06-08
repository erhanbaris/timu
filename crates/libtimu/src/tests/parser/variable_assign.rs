use pretty_assertions::assert_eq;
use rstest::*;

use crate::ast::VariableAssignAst;
use crate::nom_tools::NomSpan;
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
#[case("a = b(-1.2);", "a = b(-1.2);")]
fn custom_variable_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
    let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

    let state = State {
        file: source_file.clone(),
        indexer: Default::default(),
    };

    let input = NomSpan::new_extra(code, state);
    let result = VariableAssignAst::parse(input);
    assert!(result.is_ok(), "Failed to parse type name: {:?}", result.err());
    let (_, parsed) = result.unwrap();

    assert_eq!(parsed.to_string(), expected);
}
