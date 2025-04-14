use pretty_assertions::{assert_eq, assert_ne};
use rstest::*;

use crate::ast::{
    PrimitiveType, TimuAst, TimuFileStatementAst, TimuTypeDefinitionAst, VariableType,
};

use crate::file::SourceFile;
use crate::parser::{parser, TimuParserError, TimuTypeField};
use crate::span::Spanned;

#[rstest]
#[case("type MyType {}", TimuTypeDefinitionAst { name: "MyType".into(), fields: Vec::new(), functions: Vec::new() })]
#[case("type ___MyType___ {}",  TimuTypeDefinitionAst { name: "___MyType___".into(), fields: Vec::new(), functions: Vec::new() })]
#[case("type MyType { a: string }",  TimuTypeDefinitionAst { name: "MyType".into(), fields: vec![TimuTypeField { is_pub: false, name: "a", nullable: false, type_name: vec!["string"], }, ], functions: Vec::new() })]
#[case("type MyType { pub a: string }",  TimuTypeDefinitionAst { name: "MyType".into(), fields: vec![TimuTypeField { is_pub: true, name: "a", nullable: false, type_name: vec!["string"], }, ], functions: Vec::new() })]
#[case("type MyType { pub a: ?string }",  TimuTypeDefinitionAst { name: "MyType".into(), fields: vec![TimuTypeField { is_pub: true, name: "a", nullable: true, type_name: vec!["string"], }, ], functions: Vec::new() })]
#[case("type MyType { a: ?string }",  TimuTypeDefinitionAst { name: "MyType".into(), fields: vec![TimuTypeField { is_pub: false, name: "a", nullable: true, type_name: vec!["string"], }, ], functions: Vec::new() })]
#[case("type MyType { a: ?string.base }",  TimuTypeDefinitionAst { name: "MyType".into(), fields: vec![TimuTypeField { is_pub: false, name: "a", nullable: true, type_name: vec!["string", "base"], }, ], functions: Vec::new() })]
#[case("type MyType { a: string, b: string }",  TimuTypeDefinitionAst { name: "MyType".into(), fields: vec![TimuTypeField { is_pub: false, name: "a", nullable: false, type_name: vec!["string"], }, TimuTypeField { is_pub: false, name: "b", nullable: false, type_name: vec!["string"], }, ], functions: Vec::new() })]
fn custom_type_test(
    #[case] code: &'_ str,
    #[case] expected: TimuTypeDefinitionAst,
) -> Result<(), TimuParserError> {
    let file = parser(SourceFile::new("<MEMORY>".into(), code).into())?;

    if let TimuFileStatementAst::TypeDefinition(definition) = &file.statements[0].value {
        assert_eq!(&definition.value, &expected, "{}", code);
    } else {
        panic!("Expected File");
    }
    Ok(())
}
