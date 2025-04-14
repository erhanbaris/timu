use core::str;
use std::rc::Rc;

use pest::iterators::Pair;

use crate::{
    file::SourceFile,
    parser::{Rule, TimuTypeField},
    span::Spanned,
};

#[derive(Debug, PartialEq)]
pub struct TimuTypeDefinitionAst<'a> {
    pub name: Spanned<'a, &'a str>,
    pub fields: Vec<TimuTypeField<'a>>,
    pub functions: Vec<Spanned<'a, TimuFunctionDefinitionAst<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct TimuBodyBlock<'a> {
    pub statements: Vec<Box<TimuAst<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct TimuFunctionDefinitionAst<'a> {
    pub access: AccessType,
    pub name: Spanned<'a, &'a str>,
    pub args: Vec<FuncArg<'a>>,
    pub return_type: Spanned<'a, &'a str>,
    pub body: TimuBodyBlock<'a>,
}

#[derive(Debug)]
pub struct TimuFileAst<'a> {
    pub file: Rc<SourceFile<'a>>,
    pub statements: Vec<Spanned<'a, TimuFileStatementAst<'a>>>,
}

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
    FunctionDefinition(TimuFunctionDefinitionAst<'a>),
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
    TypeDefinition(TimuTypeDefinitionAst<'a>),
}

#[derive(Debug, PartialEq)]
pub enum TimuFileStatementAst<'a> {
    FunctionDefinition(Spanned<'a, TimuFunctionDefinitionAst<'a>>),
    TypeDefinition(Spanned<'a, TimuTypeDefinitionAst<'a>>),
}

pub struct TimuAstInfo<'a, T> {
    pub pair: Pair<'a, Rule>,
    pub line: usize,
    pub column: usize,
    pub ast: T,
}

impl<'a, T> TimuAstInfo<'a, T> {
    pub fn new(pair: Pair<'a, Rule>, ast: T) -> Self {
        let (line, column) = pair.line_col();
        Self {
            pair,
            ast,
            line,
            column,
        }
    }

    pub fn ast(&self) -> &T {
        &self.ast
    }
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
