use std::rc::Rc;

use nom::Finish;
use pretty_assertions::assert_eq;
use rstest::*;

use crate::{file::SourceFile, nom_tools::State};

#[rstest]
#[case("interface Myinterface {}", "interface Myinterface {}")]
#[case("interface Myinterface: base1, base2   , base3 {}", "interface Myinterface: base1, base2, base3 {}")]
#[case("interface Myinterface:\r\n\tbase1\r\n\t, \r\n\t base2\r\n\t , \r\n\t base3 {}", "interface Myinterface: base1, base2, base3 {}")]
#[case("interface Myinterface { \r\n\t\r\n\t}", "interface Myinterface {}")]
#[case("    interface     Myinterface    \r\n\t{} ", "interface Myinterface {}")]
#[case("    interface     Myinterface    \r\n\t{\r\n\t} ", "interface Myinterface {}")]
#[case("interface ___MyType___ {}", "interface ___MyType___ {}")]
#[case("interface Myinterface { \r\n\ta\r\n\t: \r\n\tstring ;\r\n\t}", "interface Myinterface {a: string;}")]
#[case("interface Myinterface { pub a: string; }", "interface Myinterface {pub a: string;}")]
#[case("interface Myinterface { pub a: ?string; }", "interface Myinterface {pub a: ?string;}")]
#[case("interface Myinterface { a: ?string; }", "interface Myinterface {a: ?string;}")]
#[case("interface Myinterface { a: ?string.base; }", "interface Myinterface {a: ?string.base;}")]
#[case("interface Myinterface { a: string; b: string; }", "interface Myinterface {a: string;b: string;}")]
#[case("interface Myinterface { func init(): MyType; }", "interface Myinterface {func init(): MyType;}")]
#[case("interface Myinterface { func init(): MyType; func init(): MyType; }", "interface Myinterface {func init(): MyType;func init(): MyType;}")]
#[case(
    "interface Myinterface { a: ?string.base; func init(): MyType; func init(): MyType; }",
    "interface Myinterface {a: ?string.base;func init(): MyType;func init(): MyType;}"
)]
fn custom_interface_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
    let source_file = Rc::new(SourceFile::new(vec!["<memory>".to_string()], code));

    let state = State {
        file: source_file.clone(),
    };

    let (_, response) = crate::parser::parse(state).finish().unwrap();
    assert_eq!(response.statements[0].to_string(), expected, "{}", code);
}
