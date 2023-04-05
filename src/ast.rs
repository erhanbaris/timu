
type TimuAstType = Vec<String>;

#[derive(Debug)]
pub enum TimuAst {
    Ident(String),
    String(String),
    Bool(bool),
    I32(i32),
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
    Import {
        path: Vec<String>,
        name: String
    },
    File {
        statements: Vec<Box<TimuAst>>
    },
    Assignment {
        variable: Box<TimuAst>,
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
