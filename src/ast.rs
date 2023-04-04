pub trait Ast { }

#[derive(Debug)]
pub struct FuncArg {
    pub name: String,
    pub arg_type: String
}

#[derive(Debug)]
pub enum AccessType {
    Public,
    Private
}

#[derive(Debug)]
pub enum PrimativeAst {
    String(String),
    Bool(bool),
    I32(i32)
}

#[derive(Debug)]
pub struct FunctionCallAst {
    pub compiler: bool,
    pub name: String,
    pub args: Vec<Box<ExpressionAst>>
}

#[derive(Debug)]
pub struct BinaryOperationAst {
    pub left: Box<ExpressionAst>,
    pub operator: char,
    pub right: Box<ExpressionAst>
}

#[derive(Debug)]
pub enum ExpressionAst {
    Primative(Box<PrimativeAst>),
    FunctionCall(Box<FunctionCallAst>),
    BinaryOperation(Box<BinaryOperationAst>)
}

#[derive(Debug)]
pub enum StatementAst {
    Primative(Box<PrimativeAst>),
    FunctionCall(Box<FunctionCallAst>)
}

#[derive(Debug)]
pub struct FunctionDefinitionAst {
    pub access: AccessType,
    pub name: String,
    pub args: Vec<FuncArg>,
    pub return_type: String,
    pub body: Box<BlockAst>
}

#[derive(Debug)]
pub struct BlockAst {
    pub statements: Vec<Box<StatementAst>>
}

#[derive(Debug)]
pub struct FileAst {
    pub functions: Vec<Box<FunctionDefinitionAst>>
}

impl Ast for PrimativeAst { }

impl Ast for FunctionCallAst { }

impl Ast for FunctionDefinitionAst { }

impl Ast for FileAst { }

impl Ast for BlockAst { }

impl Ast for ExpressionAst { }

impl Ast for StatementAst { }

impl Ast for BinaryOperationAst { }