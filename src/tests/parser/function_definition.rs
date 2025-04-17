use std::{cell::RefCell, rc::Rc};

use pretty_assertions::{assert_eq, assert_ne};
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
fn custom_function_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
    let source_file = Rc::new(SourceFile::new("<memory>".into(), code));
    let errors = Rc::new(RefCell::new(vec![]));

    let mut state = State {
        errors: errors.clone(),
        file: source_file.clone(),
    };

    let (_, response) = nom_parser::parse(state).unwrap();
    assert_eq!(response.to_string(), expected, "{}", code);
}
