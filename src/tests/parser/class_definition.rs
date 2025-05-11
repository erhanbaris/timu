use std::rc::Rc;

use pretty_assertions::assert_eq;
use rstest::*;

use crate::{file::SourceFile, nom_tools::State};

#[rstest]
#[case("class Myclass {}", "class Myclass {}")]
#[case("    class     Myclass    \r\n\t{} ", "class Myclass {}")]
#[case("    class     Myclass    \r\n\t{\r\n\t} ", "class Myclass {}")]
#[case("class ___MyType___ {}", "class ___MyType___ {}")]
#[case("class Myclass { a: string; }", "class Myclass {a: string;}")]
#[case("class Myclass { \r\n\ta\r\n\t: \r\n\tstring ;\r\n\t}", "class Myclass {a: string;}")]
#[case("class Myclass { \r\n\t\r\n\t\r\n\t\r\n\t}", "class Myclass {}")]
#[case("class Myclass { pub a: string; }", "class Myclass {pub a: string;}")]
#[case("class Myclass { pub a: ?string; }", "class Myclass {pub a: ?string;}")]
#[case("class Myclass { a: ?string; }", "class Myclass {a: ?string;}")]
#[case("class Myclass { a: ?string.base; }", "class Myclass {a: ?string.base;}")]
#[case("class Myclass { a: string; b: string; }", "class Myclass {a: string;b: string;}")]
#[case("class Myclass { func init(): MyType {} }", "class Myclass {func init(): MyType {}}")]
#[case("class Myclass { func init(): MyType {} func init(): MyType {} }", "class Myclass {func init(): MyType {}func init(): MyType {}}")]
#[case(
    "class Myclass { a: ?string.base; func init(): MyType {} func init(): MyType {} }",
    "class Myclass {a: ?string.base;func init(): MyType {}func init(): MyType {}}"
)]
#[case("class Myclass { func init(): MyType { var a = 1;test();test1.test2();} }", "class Myclass {func init(): MyType {var a = 1; test(); test1.test2();}}")]
#[case(
    "class Myclass { func init(): MyType { var a = test(1,2,3);} func test(): MyType {} }",
    "class Myclass {func init(): MyType {var a = test(1, 2, 3);}func test(): MyType {}}"
)]
#[case("class Myclass { func init(): MyType { var a = 1; var b = &a;} }", "class Myclass {func init(): MyType {var a = 1; var b = &a;}}")]
#[case(
    "class Myclass { func init(a:a, b:b): MyType { var a = 1; var b = &a; a = 20;} }",
    "class Myclass {func init(a: a, b: b): MyType {var a = 1; var b = &a; a = 20;}}"
)]
#[case(
    "class Myclass { func init(): MyType {if (true || false) {} else if false {} else if false {} else if false {} else {}} }",
    "class Myclass {func init(): MyType {if (true || false) {} else if false {} else if false {} else if false {} else {}}}"
)]
fn custom_class_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
    let source_file = Rc::new(SourceFile::new(vec!["<memory>".into()], code));

    let state = State {
        file: source_file.clone(),
    };

    let (_, response) = crate::parser::parse(state).unwrap();
    assert_eq!(response.to_string(), expected, "{}", code);
}
