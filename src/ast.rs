use crate::parser::TimuTypeField;

#[derive(Debug, PartialEq)]
pub enum TimuAst<'a> {
    Import {
        path: Vec<&'a str>,
        name: &'a str,
    },
    File {
        statements: Vec<Box<TimuAst<'a>>>,
    },
    Ident(&'a str),
    Primitive(PrimitiveType<'a>),
    Unary(UnaryType, Box<TimuAst<'a>>),
    FunctionCall {
        compiler: bool,
        name: &'a str,
        args: Vec<Box<TimuAst<'a>>>,
    },
    BinaryOperation {
        left: Box<TimuAst<'a>>,
        operator: char,
        right: Box<TimuAst<'a>>,
    },
    FunctionDefinition {
        access: AccessType,
        name: &'a str,
        args: Vec<FuncArg<'a>>,
        return_type: &'a str,
        body: Box<TimuAst<'a>>,
    },
    Block {
        statements: Vec<Box<TimuAst<'a>>>,
    },
    DefAssignment {
        r#type: VariableType,
        type_annotation: Vec<&'a str>,
        name: &'a str,
        data: Box<TimuAst<'a>>,
    },
    Assignment {
        name: &'a str,
        data: Box<TimuAst<'a>>,
    },
    TypeDefinition {
        name: &'a str,
        fields: Vec<TimuTypeField<'a>>,
        functions: Vec<Box<TimuAst<'a>>>,
    },
}

#[derive(PartialEq, Debug)]
pub struct FuncArg<'a> {
    pub name: &'a str,
    pub arg_type: Vec<&'a str>,
}

#[derive(PartialEq, Debug)]
pub enum AccessType {
    Public,
    Private,
}

#[derive(PartialEq, Debug)]
pub enum PrimitiveType<'a> {
    String(&'a str),
    Bool(bool),
    Array(Vec<Box<TimuAst<'a>>>),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    Float(f32),
}

#[derive(PartialEq, Debug)]
pub enum VariableType {
    Const,
    Var,
}

#[derive(PartialEq, Debug)]
pub enum UnaryType {
    Plus,
    Minus,
    LogicalNot,
}
