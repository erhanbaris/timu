use std::{borrow::Cow, rc::Rc};
use strum_macros::EnumIs;
use crate::{
    file::SourceFile,
    nom_tools::{Span, ToRange},
    parser::splited_path::SplitedPath, tir::PrimitiveType,
};

#[derive(PartialEq, Debug, Clone)]
pub enum PrimitiveValue<'base> {
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

impl PrimitiveValue<'_> {
    pub fn to_type(&self) -> PrimitiveType {
       match self {
            PrimitiveValue::String(_) => PrimitiveType::String,
            PrimitiveValue::Bool(_) => PrimitiveType::Bool,
            PrimitiveValue::I8(_) => PrimitiveType::I8,
            PrimitiveValue::U8(_) => PrimitiveType::U8,
            PrimitiveValue::I16(_) => PrimitiveType::I16,
            PrimitiveValue::U16(_) => PrimitiveType::U16,
            PrimitiveValue::I32(_) => PrimitiveType::I32,
            PrimitiveValue::U32(_) => PrimitiveType::U32,
            PrimitiveValue::I64(_) => PrimitiveType::I64,
            PrimitiveValue::U64(_) => PrimitiveType::U64,
            PrimitiveValue::Float(_, _) => PrimitiveType::Float,
            PrimitiveValue::Double(_, _) => PrimitiveType::Double,
       }
    }
}

impl<'base> AsRef<PrimitiveValue<'base>> for PrimitiveValue<'base> {
    fn as_ref(&self) -> &PrimitiveValue<'base> {
        self
    }
}

impl<'base> AsMut<PrimitiveValue<'base>> for PrimitiveValue<'base> {
    fn as_mut(&mut self) -> &mut PrimitiveValue<'base> {
        self
    }
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

#[derive(EnumIs, Debug)]
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
    pub fn ast_name(&self) -> Span<'base> {
        self.import.paths.last().unwrap().clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct ClassDefinitionAst<'base> {
    pub name: Span<'base>,
    pub fields: Vec<ClassDefinitionFieldAst<'base>>,
}

#[derive(Debug, PartialEq)]
pub struct InterfaceDefinitionAst<'base> {
    pub name: Span<'base>,
    pub fields: Vec<InterfaceDefinitionFieldAst<'base>>,
    pub base_interfaces: Vec<TypeNameAst<'base>>,
}

#[derive(Debug, PartialEq)]
pub enum InterfaceDefinitionFieldAst<'base> {
    Function(InterfaceFunctionDefinitionAst<'base>),
    Field(FieldAst<'base>),
}

#[derive(Debug, PartialEq)]
pub struct ExtendDefinitionAst<'base> {
    pub name: TypeNameAst<'base>,
    pub fields: Vec<ExtendDefinitionFieldAst<'base>>,
    pub base_interfaces: Vec<TypeNameAst<'base>>,
}

#[derive(Debug, PartialEq)]
pub enum ExtendDefinitionFieldAst<'base> {
    Function(FunctionDefinitionAst<'base>),
    Field(FieldAst<'base>),
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct RefAst<'base> {
    pub names: Vec<Span<'base>>,
}

#[derive(Debug, PartialEq)]
pub enum FunctionArgumentAst<'base> {
    This(Span<'base>),
    Argument {
        name: Span<'base>,
        field_type: TypeNameAst<'base>
    }
}

#[derive(Debug, PartialEq)]
pub enum BodyStatementAst<'base> {
    VariableDefinition(VariableDefinitionAst<'base>),
    VariableAssign(VariableAssignAst<'base>),
    FunctionCall(FunctionCallAst<'base>),
    IfCondition(IfConditionAst<'base>),
}

#[derive(Debug, PartialEq)]
pub struct BodyAst<'base> {
    pub statements: Vec<BodyStatementAst<'base>>,
}

#[derive(Debug, PartialEq)]
pub enum FunctionDefinitionLocationAst<'base> {
    Class(Span<'base>),
    #[allow(dead_code)]
    Module,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDefinitionAst<'base> {
    pub is_public: Option<Span<'base>>,
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgumentAst<'base>>,
    pub return_type: TypeNameAst<'base>,
    pub body: BodyAst<'base>,
    pub location: FunctionDefinitionLocationAst<'base>,
}

#[derive(Debug, PartialEq)]
pub enum FunctionCallType<'base> {
    This(Vec<Span<'base>>),
    Direct(Vec<Span<'base>>),
}

#[derive(Debug, PartialEq)]
pub struct FunctionCallAst<'base> {
    pub call_span: Span<'base>,
    pub path: FunctionCallType<'base>,
    //pub paths: Vec<Span<'base>>,
    pub arguments: Vec<ExpressionAst<'base>>,
}

impl FunctionCallType<'_> {
    pub fn is_this(&self) -> bool {
        matches!(self, FunctionCallType::This(_))
    }

    pub fn is_direct(&self) -> bool {
        matches!(self, FunctionCallType::Direct(_))
    }

    pub fn call(&self) -> String {
        match self {
            FunctionCallType::This(path) => format!("this.{}", path.iter().map(|p| *p.fragment()).collect::<Vec<_>>().join(".")),
            FunctionCallType::Direct(path) => path.iter().map(|p| *p.fragment()).collect::<Vec<_>>().join("."),
        }
    }

    pub fn get_path(&self) -> &Vec<Span<'_>> {
        match self {
            FunctionCallType::This(path) => path,
            FunctionCallType::Direct(path) => path,
        }
    }
}

#[derive(Debug)]
pub enum FunctionCallPathAst<'base> {
    Ident(Span<'base>),
    TypeName(TypeNameAst<'base>),
}

#[derive(Debug, PartialEq)]
pub struct InterfaceFunctionDefinitionAst<'base> {
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgumentAst<'base>>,
    pub return_type: TypeNameAst<'base>,
}

#[derive(Debug, PartialEq)]
pub enum ClassDefinitionFieldAst<'base> {
    Field(FieldAst<'base>),
    Function(FunctionDefinitionAst<'base>),
}

#[derive(Debug, PartialEq)]
pub struct FieldAst<'base> {
    pub is_public: Option<Span<'base>>,
    pub name: Span<'base>,
    pub field_type: TypeNameAst<'base>,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionAst<'base> {
    Primitive { 
        span: Span<'base>,
        value: PrimitiveValue<'base>
    },
    Ref(RefAst<'base>),
    Not(Box<ExpressionAst<'base>>),
    Ident(Span<'base>),
    FunctionCall(FunctionCallAst<'base>),
    Operation { left: Box<ExpressionAst<'base>>, operator: ExpressionOperatorType, right: Box<ExpressionAst<'base>> },
}

#[derive(Debug, PartialEq)]
pub struct IfConditionAst<'base> {
    pub expression: ExpressionAst<'base>,
    pub true_body: BodyAst<'base>,
    pub else_ifs: Vec<(ExpressionAst<'base>, BodyAst<'base>)>,
    pub false_body: Option<BodyAst<'base>>,
}

#[derive(Debug, PartialEq)]
pub struct VariableDefinitionAst<'base> {
    pub variable_definition_type: VariableDefinitionType,
    pub name: Span<'base>,
    pub expected_type: Option<TypeNameAst<'base>>,
    pub expression: Option<ExpressionAst<'base>>,
}

#[derive(Debug, PartialEq)]
pub struct VariableAssignAst<'base> {
    pub name: Span<'base>,
    pub expression: ExpressionAst<'base>,
}
