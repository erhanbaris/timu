use std::{cell::RefCell, rc::Rc};

use nom_language::error::VerboseError;
use pretty_assertions::{assert_eq, assert_ne};
use rstest::*;

use crate::ast::{PrimitiveType, VariableDefinitionAst};
use crate::nom_parser::{self, number};
use crate::nom_tools::Span;
use crate::{file::SourceFile, nom_tools::State};

#[rstest]
#[case("1", PrimitiveType::I8(1))]
#[case("-1", PrimitiveType::I8(-1))]
#[case("129", PrimitiveType::U8(129))]
#[case("-129", PrimitiveType::I16(-129))]
#[case("32767", PrimitiveType::U16(32767))]
#[case("32_767", PrimitiveType::U16(32767))]
#[case("12_345_678", PrimitiveType::I32(12345678))]
#[case("-12_345_678", PrimitiveType::I32(-12345678))]
#[case("+12_345_678", PrimitiveType::I32(12345678))]
#[case("true", PrimitiveType::Bool(true))]
#[case("false", PrimitiveType::Bool(false))]
#[case("\"erhan\"", PrimitiveType::String("erhan".to_string()))]
fn parse_primitive_test<'a>(#[case] code: &'a str, #[case] expected: PrimitiveType) {
    let source_file = Rc::new(SourceFile::new("<memory>".into(), code));

    let mut state = State {
        file: source_file.clone(),
    };

    let input = Span::new_extra(code, state);
    let (input, value) = PrimitiveType::parse::<VerboseError<Span>>(input).unwrap();

    assert_eq!(value, expected, "Parsed primitive type does not match expected");
}
