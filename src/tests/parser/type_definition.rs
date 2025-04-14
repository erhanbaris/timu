use pretty_assertions::{assert_eq, assert_ne};
use rstest::*;

use crate::ast::{PrimitiveType, TimuAst, VariableType};

use crate::parser::{parser, TimuParserError, TimuTypeField};

#[rstest]
#[case("type MyType {}", TimuAst::TypeDefinition { name: "MyType", fields: Vec::new(), functions: Vec::new() })]
#[case("type ___MyType___ {}", TimuAst::TypeDefinition { name: "___MyType___", fields: Vec::new(), functions: Vec::new() })]
#[case("type MyType { a: string }", TimuAst::TypeDefinition { name: "MyType", fields: vec![TimuTypeField { is_pub: false, name: "a", nullable: false, type_name: vec!["string"], }, ], functions: Vec::new() })]
#[case("type MyType { pub a: string }", TimuAst::TypeDefinition { name: "MyType", fields: vec![TimuTypeField { is_pub: true, name: "a", nullable: false, type_name: vec!["string"], }, ], functions: Vec::new() })]
#[case("type MyType { pub a: ?string }", TimuAst::TypeDefinition { name: "MyType", fields: vec![TimuTypeField { is_pub: true, name: "a", nullable: true, type_name: vec!["string"], }, ], functions: Vec::new() })]
#[case("type MyType { a: ?string }", TimuAst::TypeDefinition { name: "MyType", fields: vec![TimuTypeField { is_pub: false, name: "a", nullable: true, type_name: vec!["string"], }, ], functions: Vec::new() })]
#[case("type MyType { a: ?string.base }", TimuAst::TypeDefinition { name: "MyType", fields: vec![TimuTypeField { is_pub: false, name: "a", nullable: true, type_name: vec!["string", "base"], }, ], functions: Vec::new() })]
#[case("type MyType { a: string, b: string }", TimuAst::TypeDefinition { name: "MyType", fields: vec![TimuTypeField { is_pub: false, name: "a", nullable: false, type_name: vec!["string"], }, TimuTypeField { is_pub: false, name: "b", nullable: false, type_name: vec!["string"], }, ], functions: Vec::new() })]
fn custom_type_test(
    #[case] code: &'_ str,
    #[case] expected: TimuAst,
) -> Result<(), TimuParserError> {
    let ast = parser(code)?;

    if let TimuAst::File { statements } = ast {
        assert_eq!(statements[0].as_ref(), &expected, "{}", code);
    } else {
        panic!("Expected File");
    }
    Ok(())
}
