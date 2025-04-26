use std::rc::Rc;

use nom_language::error::VerboseErrorKind;
use pretty_assertions::assert_eq;
use rstest::*;

use crate::ast::PrimitiveType;
use crate::nom_tools::Span;
use crate::{file::SourceFile, nom_tools::State};

#[rstest]
#[case("1", PrimitiveType::I8(1))]
#[case("-1", PrimitiveType::I8(-1))]
#[case("129", PrimitiveType::U8(129))]
#[case("-129", PrimitiveType::I16(-129))]
#[case("32767", PrimitiveType::I16(32767))]
#[case("32_767", PrimitiveType::I16(32767))]
#[case("12_345_678", PrimitiveType::I32(12345678))]
#[case("-12_345_678", PrimitiveType::I32(-12345678))]
#[case("+12_345_678", PrimitiveType::I32(12345678))]
#[case("true", PrimitiveType::Bool(true))]
#[case("false", PrimitiveType::Bool(false))]
#[case("\"erhan\"", PrimitiveType::String("erhan".to_string()))]
#[case(r#""
erhan
""#, PrimitiveType::String(r#"
erhan
"#.to_string()))]
#[case(r#""
\"
erhan
""#, PrimitiveType::String("\n\"\nerhan\n".to_string()))]
fn parse_primitive_test<'a>(#[case] code: &'a str, #[case] expected: PrimitiveType) {
    let source_file = Rc::new(SourceFile::new("<memory>".into(), code));

    let state = State {
        file: source_file.clone(),
    };

    let input = Span::new_extra(code, state);
    let (_, value) = PrimitiveType::parse(input).unwrap();

    assert_eq!(value, expected, "Parsed primitive type does not match expected");
}

#[rstest]
#[case("340282366920938463463374607431768211450", "Invalid number length")]
#[case("340282366920938463463374607431768211455", "Invalid number length")]
fn invalid_primitive_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
    let source_file = Rc::new(SourceFile::new("<memory>".into(), code));

    let state = State {
        file: source_file.clone(),
    };

    let input = Span::new_extra(code, state);
    let error = PrimitiveType::parse(input).unwrap_err();

    if let nom::Err::Failure(error) = error {
        if let VerboseErrorKind::Context(ctx) = error.errors[0].1 {
            assert_eq!(ctx, expected, "{}", code);
        } else {
            panic!("Expected an error, but got: {:?}", error);
        }
    } else {
        panic!("Expected an error, but got: {:?}", error);
    }
}
