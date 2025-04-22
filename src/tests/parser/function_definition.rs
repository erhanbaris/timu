use std::rc::Rc;

use pretty_assertions::assert_eq;
use rstest::*;

use crate::nom_parser;
use crate::{file::SourceFile, nom_tools::State};

#[rstest]
#[case("func init(): MyType {}", "func init(): MyType {}")]
#[case("    \r\n\tfunc \r\n\tinit \r\n\t( \r\n\t) \r\n\t: \r\n\tMyType \r\n\t{ \r\n\t} \r\n\t", "func init(): MyType {}")]
#[case("func init(a: int): MyType {}", "func init(a: int): MyType {}")]
#[case("func init(a: int, b: c): MyType {}", "func init(a: int, b: c): MyType {}")]
#[case("func init(a: ?int.c, b: c): MyType {}", "func init(a: ?int.c, b: c): MyType {}")]
#[case("func init(): MyType {} func init(): MyType {}", "func init(): MyType {}func init(): MyType {}")]
#[case("func init(): MyType {} func init(): MyType {var test = 123456789.0e+7;}", "func init(): MyType {}func init(): MyType {var test = 1234567890000000.0;}")]
#[case("func init(): MyType {} func init(): MyType {test = 123456789.0e+7;}", "func init(): MyType {}func init(): MyType {test = 1234567890000000.0;}")]
fn custom_function_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
    let source_file = Rc::new(SourceFile::new("<memory>".into(), code));

    let state = State {
        file: source_file.clone(),
    };

    let (_, response) = nom_parser::parse(state).unwrap();
    assert_eq!(response.to_string(), expected, "{}", code);
}
