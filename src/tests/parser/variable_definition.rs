use std::{cell::RefCell, rc::Rc};

use nom_language::error::VerboseError;
use pretty_assertions::{assert_eq, assert_ne};
use rstest::*;

use crate::ast::VariableDefinitionAst;
use crate::nom_parser;
use crate::nom_tools::Span;
use crate::{file::SourceFile, nom_tools::State};

#[rstest]
#[case("var a = false;", "var a = false;")]
#[case("var a = 100;", "var a = 100;")]
#[case("var a = 100;", "var a = 100;")]
#[case("var _a = 200;", "var _a = 200;")]
#[case("const a = 100;", "const a = 100;")]
#[case("const _123a = 100;", "const _123a = 100;")]
#[case("const a_123 = 100;", "const a_123 = 100;")]
#[case("const a_123_____ = 100;", "const a_123_____ = 100;")]
#[case("const a = 1.0;", "const a = 1.0;")]
#[case("const a = 1.2;", "const a = 1.2;")]
fn custom_variable_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
    let source_file = Rc::new(SourceFile::new("<memory>".into(), code));

    let mut state = State {
        file: source_file.clone(),
    };

    let input = Span::new_extra(code, state);
    let result = VariableDefinitionAst::parse::<VerboseError<Span>>(input);
    assert!(result.is_ok(), "Failed to parse type name: {:?}", result.err());
    let (_, parsed) = result.unwrap();

    assert_eq!(parsed.to_string(), expected);
}
