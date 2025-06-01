use nom_language::error::VerboseErrorKind;
use pretty_assertions::assert_eq;
use rstest::*;

use crate::ast::PrimitiveValue;
use crate::nom_tools::Span;
use crate::{file::SourceFile, nom_tools::State};

#[rstest]
#[case("1", PrimitiveValue::I8(1))]
#[case("-1", PrimitiveValue::I8(-1))]
#[case("129", PrimitiveValue::U8(129))]
#[case("-129", PrimitiveValue::I16(-129))]
#[case("32767", PrimitiveValue::I16(32767))]
#[case("32_767", PrimitiveValue::I16(32767))]
#[case("12_345_678", PrimitiveValue::I32(12345678))]
#[case("-12_345_678", PrimitiveValue::I32(-12345678))]
#[case("+12_345_678", PrimitiveValue::I32(12345678))]
#[case("true", PrimitiveValue::Bool(true))]
#[case("false", PrimitiveValue::Bool(false))]
#[case("\"erhan\"", PrimitiveValue::String("erhan".into()))]
#[case(r#""
erhan
""#, PrimitiveValue::String(r#"
erhan
"#.into()))]
#[case(r#""
\"
erhan
""#, PrimitiveValue::String("\n\"\nerhan\n".into()))]
fn parse_primitive_test<'base>(#[case] code: &'base str, #[case] expected: PrimitiveValue) {
    let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

    let state = State {
        file: source_file.clone(),
        indexer: Default::default(),
    };

    let input = Span::new_extra(code, state);
    let (_, (_, value)) = PrimitiveValue::parse(input).unwrap();

    assert_eq!(value, expected, "Parsed primitive type does not match expected");
}

#[rstest]
#[case("340282366920938463463374607431768211450", "Invalid number length")]
#[case("340282366920938463463374607431768211455", "Invalid number length")]
fn invalid_primitive_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
    let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

    let state = State {
        file: source_file.clone(),
        indexer: Default::default(),
    };

    let input = Span::new_extra(code, state);
    let error = PrimitiveValue::parse(input).unwrap_err();

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
