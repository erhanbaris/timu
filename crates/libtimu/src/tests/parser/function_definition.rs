use pretty_assertions::assert_eq;
use rstest::*;

use crate::{file::SourceFile, nom_tools::State};

#[rstest]
#[case("func init(): string {}", "func init(): string {}")]
#[case("func init(): string {} func init(): string {}", "func init(): string {}\nfunc init(): string {}")]
#[case(
    "func init(): string {} func init(): string {}",
    "func init(): string {}\nfunc init(): string {}"
)]
#[case("func init(): string { var a = 1;test();test1.test2();}", "func init(): string {var a = 1; test(); test1.test2();}")]
#[case(
    "func init(): string { var a = test(1,2,3);} func test(): string {}",
    "func init(): string {var a = test(1, 2, 3);}\nfunc test(): string {}"
)]
#[case("func init(): string { var a = 1; var b = ref a;}", "func init(): string {var a = 1; var b = ref a;}")]
#[case(
    "func init(a:a, b:b): string { var a = 1; var b = ref a; a = 20;}",
    "func init(a: a, b: b): string {var a = 1; var b = ref a; a = 20;}"
)]
#[case(
    "func init(this): string {if (true || false) {} else if false {} else if false {} else if false {} else {}}",
    "func init(this): string {if (true || false) {} else if false {} else if false {} else if false {} else {}}"
)]
#[case("func init(a: ref ?string): string {}", "func init(a: ref ?string): string {}")]
fn custom_function_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
    let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

    let state = State {
        file: source_file.clone(),
        indexer: Default::default(),
    };

    let (_, response) = crate::parser::parse(&state).unwrap();
    assert_eq!(response.to_string(), expected, "{}", code);
}
