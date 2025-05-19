use std::{borrow::Cow, rc::Rc};

use crate::{
    file::SourceFile,
    nom_tools::{Span, ToRange},
    parser::splited_path::SplitedPath,
};

#[derive(PartialEq, Debug, Clone)]
pub enum PrimitiveType<'base> {
    String(Cow<'base, str>),
    Bool(bool),
    //Array(Vec<Box<TimuAst<'base>>>),
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
pub enum VariableDefinitionType {
    Const,
    Var,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ExpressionOperatorType {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Equal,
    NotEqual,
    GreaterEqualThan,
    GreaterThan,
    LessEqualThan,
    LessThan,
    Xor,
    LogicalOr,
    LogicalAnd,
    BitwiseShiftLeft,
    BitwiseShiftRight,
}

#[derive(Debug)]
pub struct FileAst<'base> {
    pub file: Rc<SourceFile<'base>>,
    pub statements: Vec<FileStatementAst<'base>>,
}

#[derive(Debug)]
pub enum FileStatementAst<'base> {
    Class(Rc<ClassDefinitionAst<'base>>),
    Function(Rc<FunctionDefinitionAst<'base>>),
    Interface(Rc<InterfaceDefinitionAst<'base>>),
    Extend(Rc<ExtendDefinitionAst<'base>>),
    Use(Rc<UseAst<'base>>),
}

#[derive(Debug)]
pub struct UseAst<'base> {
    pub alias: Option<Span<'base>>,
    pub import: SplitedPath<'base>,
}

impl<'base> UseAst<'base> {
    pub fn name(&self) -> Span<'base> {
        self.import.paths.last().unwrap().clone()
    }
}

#[derive(Debug)]
pub struct ClassDefinitionAst<'base> {
    pub name: Span<'base>,
    pub fields: Vec<ClassDefinitionFieldAst<'base>>,
}

#[derive(Debug)]
pub struct InterfaceDefinitionAst<'base> {
    pub name: Span<'base>,
    pub fields: Vec<InterfaceDefinitionFieldAst<'base>>,
    pub base_interfaces: Vec<TypeNameAst<'base>>,
}

#[derive(Debug)]
pub enum InterfaceDefinitionFieldAst<'base> {
    Function(InterfaceFunctionDefinitionAst<'base>),
    Field(FieldAst<'base>),
}

#[derive(Debug)]
pub struct ExtendDefinitionAst<'base> {
    pub name: TypeNameAst<'base>,
    pub fields: Vec<ExtendDefinitionFieldAst<'base>>,
    pub base_interfaces: Vec<TypeNameAst<'base>>,
}

#[derive(Debug)]
pub enum ExtendDefinitionFieldAst<'base> {
    Function(FunctionDefinitionAst<'base>),
    Field(FieldAst<'base>),
}

#[derive(Debug)]
pub struct TypeNameAst<'base> {
    pub nullable: bool,
    pub names: Vec<Span<'base>>,
}

impl ToRange for TypeNameAst<'_> {
    fn to_range(&self) -> std::ops::Range<usize> {
        let start = self.names.first().map_or(0, |path| path.location_offset());
        let end = self.names.last().map_or(0, |path| path.location_offset() + path.fragment().len());
        start..end
    }
}

#[derive(Debug)]
pub struct RefAst<'base> {
    pub names: Vec<Span<'base>>,
}

#[derive(Debug)]
pub struct FunctionArgumentAst<'base> {
    pub name: Span<'base>,
    pub field_type: TypeNameAst<'base>,
}

#[derive(Debug)]
pub enum BodyStatementAst<'base> {
    VariableDefinition(VariableDefinitionAst<'base>),
    VariableAssign(VariableAssignAst<'base>),
    FunctionCall(FunctionCallAst<'base>),
    IfCondition(IfConditionAst<'base>),
}

#[derive(Debug)]
pub struct BodyAst<'base> {
    pub statements: Vec<BodyStatementAst<'base>>,
}

#[derive(Debug)]
pub enum FunctionDefinitionLocationAst<'base> {
    Class(Span<'base>),
    #[allow(dead_code)]
    Module,
}

#[derive(Debug)]
pub struct FunctionDefinitionAst<'base> {
    pub is_public: Option<Span<'base>>,
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgumentAst<'base>>,
    pub return_type: TypeNameAst<'base>,
    pub body: BodyAst<'base>,
    pub location: FunctionDefinitionLocationAst<'base>,
}

#[derive(Debug)]
pub struct FunctionCallAst<'base> {
    pub paths: Vec<FunctionCallPathAst<'base>>,
    pub arguments: Vec<ExpressionAst<'base>>,
}

#[derive(Debug)]
pub enum FunctionCallPathAst<'base> {
    Ident(Span<'base>),
    TypeName(TypeNameAst<'base>),
}

#[derive(Debug)]
pub struct InterfaceFunctionDefinitionAst<'base> {
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgumentAst<'base>>,
    pub return_type: TypeNameAst<'base>,
}

#[derive(Debug)]
pub enum ClassDefinitionFieldAst<'base> {
    Field(FieldAst<'base>),
    Function(FunctionDefinitionAst<'base>),
}

#[derive(Debug)]
pub struct FieldAst<'base> {
    pub is_public: Option<Span<'base>>,
    pub name: Span<'base>,
    pub field_type: TypeNameAst<'base>,
}

#[derive(Debug)]
pub enum ExpressionAst<'base> {
    Primitive(PrimitiveType<'base>),
    Ref(RefAst<'base>),
    Not(Box<ExpressionAst<'base>>),
    Ident(Span<'base>),
    FunctionCall(FunctionCallAst<'base>),
    Operation { left: Box<ExpressionAst<'base>>, operator: ExpressionOperatorType, right: Box<ExpressionAst<'base>> },
}

#[derive(Debug)]
pub struct IfConditionAst<'base> {
    pub expression: ExpressionAst<'base>,
    pub true_body: BodyAst<'base>,
    pub else_ifs: Vec<(ExpressionAst<'base>, BodyAst<'base>)>,
    pub false_body: Option<BodyAst<'base>>,
}

#[derive(Debug)]
pub struct VariableDefinitionAst<'base> {
    pub variable_definition_type: VariableDefinitionType,
    pub name: Span<'base>,
    pub expected_type: Option<TypeNameAst<'base>>,
    pub expression: Option<ExpressionAst<'base>>,
}

#[derive(Debug)]
pub struct VariableAssignAst<'base> {
    pub name: Span<'base>,
    pub expression: ExpressionAst<'base>,
}
