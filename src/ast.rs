
use crate::nom_tools::Span;

#[derive(PartialEq, Debug, Clone)]
pub enum PrimitiveType {
    String(String),
    Bool(bool),
    //Array(Vec<Box<TimuAst<'a>>>),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    Float(f64, u8),
    Double(f64, u8),
}

#[derive(PartialEq, Debug)]
pub enum VariableType {
    Const,
    Var,
}

#[derive(Debug)]
pub struct FileAst<'a> {
    pub statements: Vec<FileStatementAst<'a>>,
}

#[derive(Debug)]
pub enum FileStatementAst<'a> {
    ClassDefinition(ClassDefinitionAst<'a>),
    FunctionDefinition(FunctionDefinitionAst<'a>),
}

#[derive(Debug)]
pub struct ClassDefinitionAst<'a> {
    pub name: Span<'a>,
    pub fields: Vec<ClassDefinitionFieldAst<'a>>,
}

#[derive(Debug)]
pub struct TypeNameAst<'a> {
    pub nullable: bool,
    pub names: Vec<Span<'a>>,
}

#[derive(Debug)]
pub struct FunctionArgumentAst<'a> {
    pub name: Span<'a>,
    pub field_type: TypeNameAst<'a>,
}

#[derive(Debug)]
pub enum BodyStatementAst<'a> {
    Variable(VariableDefinitionAst<'a>),
}

#[derive(Debug)]
pub struct BodyAst<'a> {
    pub statements: Vec<BodyStatementAst<'a>>,
}

#[derive(Debug)]
pub struct FunctionDefinitionAst<'a> {
    pub is_public: bool,
    pub name: Span<'a>,
    pub arguments: Vec<FunctionArgumentAst<'a>>,
    pub return_type: TypeNameAst<'a>,
    pub body: BodyAst<'a>,
}

#[derive(Debug)]
pub enum ClassDefinitionFieldAst<'a> {
    ClassField(FieldAst<'a>),
    ClassFunction(FunctionDefinitionAst<'a>),
}

#[derive(Debug)]
pub struct FieldAst<'a> {
    pub is_public: bool,
    pub name: Span<'a>,
    pub field_type: TypeNameAst<'a>,
}

#[derive(Debug)]
pub enum ExpressionAst {
    Primitive(PrimitiveType),
}

#[derive(Debug)]
pub struct VariableDefinitionAst<'a> {
    pub variable_type: VariableType,
    pub name: Span<'a>,
    pub expression: ExpressionAst,
}
