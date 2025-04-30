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
pub struct FileAst<'a> {
    pub statements: Vec<FileStatementAst<'a>>,
}

#[derive(Debug)]
pub enum FileStatementAst<'a> {
    Class(ClassDefinitionAst<'a>),
    Function(FunctionDefinitionAst<'a>),
    Interface(InterfaceDefinitionAst<'a>),
    Extend(ExtendDefinitionAst<'a>),
    Use(UseAst<'a>),
}

#[derive(Debug)]
pub struct UseAst<'a> {
    pub paths: Vec<Span<'a>>,
}

#[derive(Debug)]
pub struct ClassDefinitionAst<'a> {
    pub name: Span<'a>,
    pub fields: Vec<ClassDefinitionFieldAst<'a>>,
}

#[derive(Debug)]
pub struct InterfaceDefinitionAst<'a> {
    pub name: Span<'a>,
    pub fields: Vec<InterfaceDefinitionFieldAst<'a>>,
    pub base_interfaces: Vec<TypeNameAst<'a>>,
}

#[derive(Debug)]
pub enum InterfaceDefinitionFieldAst<'a> {
    Function(InterfaceFunctionDefinitionAst<'a>),
    Field(FieldAst<'a>),
}

#[derive(Debug)]
pub struct ExtendDefinitionAst<'a> {
    pub name: Span<'a>,
    pub fields: Vec<ExtendDefinitionFieldAst<'a>>,
    pub base_interfaces: Vec<TypeNameAst<'a>>,
}

#[derive(Debug)]
pub enum ExtendDefinitionFieldAst<'a> {
    Function(FunctionDefinitionAst<'a>),
    Field(FieldAst<'a>),
}

#[derive(Debug)]
pub struct TypeNameAst<'a> {
    pub nullable: bool,
    pub names: Vec<Span<'a>>,
}

#[derive(Debug)]
pub struct RefAst<'a> {
    pub names: Vec<Span<'a>>,
}

#[derive(Debug)]
pub struct FunctionArgumentAst<'a> {
    pub name: Span<'a>,
    pub field_type: TypeNameAst<'a>,
}

#[derive(Debug)]
pub enum BodyStatementAst<'a> {
    VariableDefinition(VariableDefinitionAst<'a>),
    VariableAssign(VariableAssignAst<'a>),
    FunctionCall(FunctionCallAst<'a>),
    IfCondition(IfConditionAst<'a>),
}

#[derive(Debug)]
pub struct BodyAst<'a> {
    pub statements: Vec<BodyStatementAst<'a>>,
}

#[derive(Debug)]
pub struct FunctionDefinitionAst<'a> {
    pub is_public: Option<Span<'a>>,
    pub name: Span<'a>,
    pub arguments: Vec<FunctionArgumentAst<'a>>,
    pub return_type: TypeNameAst<'a>,
    pub body: BodyAst<'a>,
}

#[derive(Debug)]
pub struct FunctionCallAst<'a> {
    pub paths: Vec<FunctionCallPathAst<'a>>,
    pub arguments: Vec<ExpressionAst<'a>>,
}

#[derive(Debug)]
pub enum FunctionCallPathAst<'a> {
    Ident(Span<'a>),
    TypeName(TypeNameAst<'a>),
}

#[derive(Debug)]
pub struct InterfaceFunctionDefinitionAst<'a> {
    pub name: Span<'a>,
    pub arguments: Vec<FunctionArgumentAst<'a>>,
    pub return_type: TypeNameAst<'a>,
}

#[derive(Debug)]
pub enum ClassDefinitionFieldAst<'a> {
    ClassField(FieldAst<'a>),
    ClassFunction(FunctionDefinitionAst<'a>),
}

#[derive(Debug)]
pub struct FieldAst<'a> {
    pub is_public: Option<Span<'a>>,
    pub name: Span<'a>,
    pub field_type: TypeNameAst<'a>,
}

#[derive(Debug)]
pub enum ExpressionAst<'a> {
    Primitive(PrimitiveType),
    Ref(RefAst<'a>),
    Not(Box<ExpressionAst<'a>>),
    Ident(Span<'a>),
    FunctionCall(FunctionCallAst<'a>),
    Operation { left: Box<ExpressionAst<'a>>, operator: ExpressionOperatorType, right: Box<ExpressionAst<'a>> },
}

#[derive(Debug)]
pub struct IfConditionAst<'a> {
    pub expression: ExpressionAst<'a>,
    pub true_body: BodyAst<'a>,
    pub else_ifs: Vec<(ExpressionAst<'a>, BodyAst<'a>)>,
    pub false_body: Option<BodyAst<'a>>,
}

#[derive(Debug)]
pub struct VariableDefinitionAst<'a> {
    pub variable_definition_type: VariableDefinitionType,
    pub name: Span<'a>,
    pub expected_type: Option<TypeNameAst<'a>>,
    pub expression: Option<ExpressionAst<'a>>,
}

#[derive(Debug)]
pub struct VariableAssignAst<'a> {
    pub name: Span<'a>,
    pub expression: ExpressionAst<'a>,
}
