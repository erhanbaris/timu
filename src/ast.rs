
pub type TimuAstType = Vec<String>;

#[derive(Debug)]
pub enum TimuAst {
    Import {
        path: Vec<String>,
        name: String
    },
    File {
        statements: Vec<Box<TimuAst>>
    },
    Ident(String),
    Primative(PrimativeType),
    Unary(UnaryType, Box<TimuAst>),
    FunctionCall {
        compiler: bool,
        name: String,
        args: Vec<Box<TimuAst>>
    },
    BinaryOperation {
        left: Box<TimuAst>,
        operator: char,
        right: Box<TimuAst>
    },
    FunctionDefinition {
        access: AccessType,
        name: String,
        args: Vec<FuncArg>,
        return_type: TimuAstType,
        body: Box<TimuAst>
    },
    Block {
        statements: Vec<Box<TimuAst>>
    },
    DefAssignment {
        r#type: VariableType,
        type_annotation: Vec<String>,
        name: String,
        data: Box<TimuAst>
    },
    Assignment {
        name: String,
        data: Box<TimuAst>
    }
}

#[derive(Debug)]
pub struct FuncArg {
    pub name: String,
    pub arg_type: TimuAstType
}

#[derive(Debug)]
pub enum AccessType {
    Public,
    Private
}

#[derive(Debug)]
pub enum PrimativeType {
    String(String),
    Bool(bool),
    Array(Vec<Box<TimuAst>>),
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

#[derive(Debug)]
pub enum VariableType {
    Immutable,
    Mutable
}

#[derive(Debug)]
pub enum UnaryType {
    Plus,
    Minus,
    LogicalNot
}
