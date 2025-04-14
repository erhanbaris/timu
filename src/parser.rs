use std::rc::Rc;

/* **************************************************************************************************************** */
/* **************************************************** MODS ****************************************************** */
/* *************************************************** IMPORTS **************************************************** */
use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest::{
    error::LineColLocation,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::ast::{
    AccessType, FuncArg, PrimitiveType, TimuAst, TimuBodyBlock, TimuFileAst, TimuFileStatementAst,
    TimuFunctionDefinitionAst, TimuTypeDefinitionAst, UnaryType, VariableType,
};
use crate::file::SourceFile;
use crate::span::Spanned;

/* ******************************************** STATICS/CONSTS/TYPES ********************************************** */
lazy_static::lazy_static! {
    pub static ref PRATT: PrattParser<Rule> = build_pratt();
}

static I8_RANGE: std::ops::Range<i128> = (i8::MIN as i128)..(i8::MAX as i128);
static U8_RANGE: std::ops::Range<i128> = (u8::MIN as i128)..(u8::MAX as i128);

static I16_RANGE: std::ops::Range<i128> = (i16::MIN as i128)..(i16::MAX as i128);
static U16_RANGE: std::ops::Range<i128> = (u16::MIN as i128)..(u16::MAX as i128);

static I32_RANGE: std::ops::Range<i128> = (i32::MIN as i128)..(i32::MAX as i128);
static U32_RANGE: std::ops::Range<i128> = (u32::MIN as i128)..(u32::MAX as i128);

static I64_RANGE: std::ops::Range<i128> = (i64::MIN as i128)..(i64::MAX as i128);
static U64_RANGE: std::ops::Range<i128> = (u64::MIN as i128)..(u64::MAX as i128);

/* **************************************************** MACROS **************************************************** */
/* *************************************************** STRUCTS **************************************************** */
#[derive(Parser)]
#[grammar = "../assets/timu.pest"]
pub struct TimuParser;
impl TimuParser {}

#[derive(Debug, PartialEq)]
pub struct TimuParserError {
    pub line: usize,
    pub column: usize,
    pub error: String,
}

#[derive(Debug, PartialEq)]
pub struct TimuTypeField<'a> {
    pub is_pub: bool,
    pub name: &'a str,
    pub nullable: bool,
    pub type_name: TimuTypeUsageName<'a>,
}

#[derive(Debug, PartialEq)]
pub struct TimuFunctionArg<'a> {
    pub name: &'a str,
    pub nullable: bool,
    pub type_name: &'a str,
}

type TimuTypeUsageName<'a> = Vec<&'a str>;

/* **************************************************** ENUMS ***************************************************** */
/* ************************************************** FUNCTIONS *************************************************** */
fn build_pratt() -> PrattParser<Rule> {
    PrattParser::new()
        .op(Op::infix(Rule::logical_or, Assoc::Left))
        .op(Op::infix(Rule::logical_and, Assoc::Left))
        .op(Op::infix(Rule::equal, Assoc::Right) | Op::infix(Rule::not_equal, Assoc::Right))
        .op(Op::infix(Rule::greater_than_or_equal, Assoc::Left)
            | Op::infix(Rule::less_than_or_equal, Assoc::Left)
            | Op::infix(Rule::greater_than, Assoc::Left)
            | Op::infix(Rule::less_than, Assoc::Left))
        .op(Op::infix(Rule::bitwise_xor, Assoc::Left) | Op::infix(Rule::bitwise_or, Assoc::Left))
        .op(Op::infix(Rule::bitwise_and, Assoc::Left))
        .op(Op::infix(Rule::shift_right, Assoc::Left) | Op::infix(Rule::shift_left, Assoc::Left))
        .op(Op::infix(Rule::plus, Assoc::Left) | Op::infix(Rule::minus, Assoc::Left))
        .op(Op::infix(Rule::modulo, Assoc::Left)
            | Op::infix(Rule::divide, Assoc::Left)
            | Op::infix(Rule::multiply, Assoc::Left))
        .op(Op::infix(Rule::exponent, Assoc::Right))
}

fn check_rule(pair: &Pair<'_, Rule>, expected_rule: Rule) -> Result<(), TimuParserError> {
    let actual_rule = pair.as_rule();

    if actual_rule != expected_rule {
        return Err(TimuParserError::new(
            &pair,
            format!(
                "Expected '{:?}', but found '{:?}'",
                expected_rule, actual_rule
            ),
        ));
    }

    Ok(())
}

fn to_string_vector(rule: Pair<'_, Rule>) -> Vec<&'_ str> {
    rule.into_inner().map(|item| item.as_str()).collect()
}

fn parse_import(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    let mut inner_rules = pair.into_inner();

    let path = to_string_vector(inner_rules.next().unwrap());
    let name = match inner_rules.next() {
        Some(rule) => rule.as_str(),
        None => path.last().unwrap(),
    };

    Ok(Box::new(TimuAst::Import { path, name }))
}

fn parse_variable_definition(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    let mut inner_rules = pair.into_inner();
    let mut pair = inner_rules.next().unwrap();

    let var_type = match pair.as_str() {
        "var" => VariableType::Var,
        "const" => VariableType::Const,
        _type => {
            return Err(TimuParserError::new(
                &pair,
                format!("Unsupported variable type ({})", _type),
            ))
        }
    };

    let name = inner_rules.next().unwrap().as_str();
    let type_annotation = to_string_vector(inner_rules.next().unwrap());

    Ok(Box::new(TimuAst::DefAssignment {
        r#type: var_type,
        type_annotation,
        name,
        data: parse_rule(inner_rules.next().unwrap())?,
    }))
}

fn parse_variable_assign(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    let mut inner_rules = pair.into_inner();
    let name = inner_rules.next().unwrap().as_str();

    Ok(Box::new(TimuAst::Assignment {
        name,
        data: parse_rule(inner_rules.next().unwrap())?,
    }))
}

fn parse_function_args(pair: Pair<'_, Rule>) -> Result<Vec<TimuFunctionArg>, TimuParserError> {
    let mut inner_rules = pair.clone().into_inner();
    let mut args: Vec<_> = Vec::new();

    for item in inner_rules.into_iter() {
        println!("item: {:?}", item);
    }

    Ok(args)
}

fn check_and_consume<'a>(pairs: &mut Pairs<'a, Rule>, expected: Rule) -> Option<Pair<'a, Rule>> {
    match pairs.peek().map(|item| item.as_rule()) {
        Some(rule) if rule == expected => pairs.next(),
        _ => None,
    }
}

fn parse_define_type_field(pair: Pair<'_, Rule>) -> Result<TimuTypeField, TimuParserError> {
    let mut inner = pair.into_inner();

    let is_pub = check_and_consume(&mut inner, Rule::pub_visibility).is_some();
    let name = inner.next().unwrap().as_str();
    let nullable = check_and_consume(&mut inner, Rule::nullable).is_some();
    let type_name = to_string_vector(inner.next().unwrap());

    Ok(TimuTypeField {
        is_pub,
        name,
        nullable,
        type_name,
    })
}

fn parse_type_definition<'a>(
    pair: Pair<'a, Rule>,
    file: Rc<SourceFile<'a>>,
) -> Result<Spanned<'a, TimuTypeDefinitionAst<'a>>, TimuParserError> {
    let span = pair.as_span();
    let mut inner_rules = pair.into_inner();
    let mut fields = Vec::new();
    let mut functions = Vec::new();
    let name_pair = inner_rules.next().unwrap();
    let name = Spanned::new(name_pair.as_str(), file.clone(), name_pair.as_span());

    /* Fields or function definitions on the type */
    if let Some(fields_rule) = inner_rules.next() {
        let mut type_fields = fields_rule.into_inner();

        for item in type_fields.into_iter() {
            match item.as_rule() {
                Rule::define_type_field => {
                    fields.push(parse_define_type_field(item)?);
                }
                Rule::define_function => {
                    functions.push(parse_function_definition(item, file.clone())?);
                }
                _ => {
                    return Err(TimuParserError::new(
                        &item,
                        "Not valid for type definition".to_string(),
                    ))
                }
            }
        }
    }

    Ok(Spanned::new(
        TimuTypeDefinitionAst {
            name,
            fields,
            functions,
        },
        file,
        span,
    ))
}

fn parse_file<'a>(
    pairs: Pairs<'a, Rule>,
    file: Rc<SourceFile<'a>>,
) -> Result<TimuFileAst<'a>, TimuParserError> {
    let mut statements = Vec::new();

    for pair in pairs {
        /* End of file */
        if pair.as_rule() == Rule::EOI {
            continue;
        }

        statements.push(parse_file_statement(pair, file.clone())?);
    }

    Ok(TimuFileAst { statements, file })
}

fn parse_string(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    check_rule(&pair, Rule::primitive_type_string)?;

    let mut inner_rules = pair.into_inner();
    let string_data = inner_rules.next().unwrap();

    Ok(Box::new(TimuAst::Primitive(PrimitiveType::String(
        string_data.as_str(),
    ))))
}

fn parse_bool(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    check_rule(&pair, Rule::primitive_type_boolean)?;
    Ok(Box::new(TimuAst::Primitive(PrimitiveType::Bool(
        match pair.as_str() {
            "true" => true,
            _ => false,
        },
    ))))
}

fn parse_integer(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    check_rule(&pair, Rule::primitive_type_integer)?;

    let integer_rule = pair.into_inner().next().unwrap();
    let integer = match integer_rule.as_rule() {
        Rule::integer_decimal => integer_rule
            .as_str()
            .replace("_", "")
            .parse::<i128>()
            .unwrap(),
        Rule::integer_binary => i128::from_str_radix(integer_rule.as_str(), 2).unwrap_or_default(),
        Rule::integer_hexadecimal => {
            i128::from_str_radix(integer_rule.as_str(), 16).unwrap_or_default()
        }
        Rule::integer_octal => i128::from_str_radix(integer_rule.as_str(), 8).unwrap_or_default(),
        Rule::integer_zero => 0,
        _ => 0,
    };

    if I8_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::I8(
            integer as i8,
        ))))
    } else if U8_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::U8(
            integer as u8,
        ))))
    } else if I16_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::I16(
            integer as i16,
        ))))
    } else if U16_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::U16(
            integer as u16,
        ))))
    } else if I32_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::I32(
            integer as i32,
        ))))
    } else if U32_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::U32(
            integer as u32,
        ))))
    } else if I64_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::I64(
            integer as i64,
        ))))
    } else if U64_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::U64(
            integer as u64,
        ))))
    } else {
        Err(TimuParserError::new(
            &integer_rule,
            "Integer type not supported".to_string(),
        ))
    }
}

fn parse_float(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    check_rule(&pair, Rule::primitive_type_float)?;
    Ok(Box::new(TimuAst::Primitive(PrimitiveType::Float(
        pair.as_str()
            .replace("_", "")
            .parse::<f32>()
            .unwrap_or_default(),
    ))))
}

fn parse_unary(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    check_rule(&pair, Rule::unary)?;
    let mut into_inner = pair.into_inner();

    let unary = into_inner.next().unwrap();
    let unary = match unary.as_str() {
        "-" => UnaryType::Minus,
        "+" => UnaryType::Plus,
        "!" => UnaryType::LogicalNot,
        _ => {
            return Err(TimuParserError::new(
                &unary,
                "Unsupported unary type".to_string(),
            ))
        }
    };

    let expression = parse_rule(into_inner.next().unwrap())?;
    Ok(Box::new(TimuAst::Unary(unary, expression)))
}

fn parse_rule(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    let rule = pair.as_rule();
    match rule {
        Rule::import => parse_import(pair),

        // Primitive
        Rule::primitive_type_string => parse_string(pair),
        Rule::primitive_type_boolean => parse_bool(pair),
        Rule::primitive_type_integer => parse_integer(pair),
        Rule::primitive_type_float => parse_float(pair),

        // Unary
        Rule::unary => parse_unary(pair),
        Rule::infix => parse_binary_operation(pair),

        Rule::function_call => parse_function_call(pair),

        // Assignment
        Rule::define_variable => parse_variable_definition(pair),
        Rule::assign_variable => parse_variable_assign(pair),

        // Types
        //Rule::define_type => parse_type_definition(pair),
        _ => return Err(TimuParserError::new(&pair, "unknown syntax".to_string())),
    }
}

fn parse_file_statement<'a>(
    pair: Pair<'a, Rule>,
    file: Rc<SourceFile<'a>>,
) -> Result<Spanned<'a, TimuFileStatementAst<'a>>, TimuParserError> {
    let rule = pair.as_rule();
    let span = pair.as_span();
    let value = match rule {
        // Rule::import => parse_import(pair),
        // Rule::define_variable =>  parse_variable_definition(pair),
        Rule::define_type => TimuFileStatementAst::TypeDefinition(parse_type_definition(pair, file.clone())?),
        Rule::define_function => TimuFileStatementAst::FunctionDefinition(parse_function_definition(pair, file.clone())?),
        _ => return Err(TimuParserError::new(&pair, "unknown syntax".to_string())),
    };

    Ok(Spanned::new(value, file, span))
}

fn parse_type_annotation(pair: Pair<'_, Rule>) -> Vec<&'_ str> {
    let mut type_info = Vec::new();

    for type_rule in pair.into_inner() {
        type_info.push(type_rule.as_str());
    }

    type_info
}

fn parse_function_definition<'a>(
    pair: Pair<'a, Rule>,
    file: Rc<SourceFile<'a>>,
) -> Result<Spanned<'a, TimuFunctionDefinitionAst<'a>>, TimuParserError> {
    let span = pair.as_span();
    let mut access = AccessType::Private;
    let mut name = Spanned::new("", file.clone(), pair.as_span());
    let mut return_type = Spanned::new("", file.clone(), pair.as_span());
    let mut args = Vec::new();
    let mut statements = Vec::new();
    let inner_rules = pair.into_inner();

    for pair in inner_rules {
        match pair.as_rule() {
            Rule::pub_visibility => access = AccessType::Public,
            Rule::define_function_return_type => return_type = Spanned::new(pair.as_str(), file.clone(), pair.as_span()),
            Rule::ident => name = Spanned::new(pair.as_str(), file.clone(), pair.as_span()),
            Rule::define_function_arguments => {
                for arg_rule in pair.into_inner() {
                    let mut arg_rule = arg_rule.into_inner();

                    let mut func_argument = FuncArg {
                        name: arg_rule.next().unwrap().as_str(),
                        arg_type: Default::default(),
                    };

                    for item in arg_rule {
                        func_argument.arg_type = parse_type_annotation(item)
                    }

                    args.push(func_argument);
                }
            }
            Rule::body_block => {
                let pairs = pair.into_inner();

                for pair in pairs {
                    statements.push(parse_rule(pair)?);
                }
            }
            _ => {}
        }
    }

    let body = TimuBodyBlock { statements };
    Ok(Spanned::new(
        TimuFunctionDefinitionAst {
            access,
            name,
            args,
            body,
            return_type,
        },
        file,
        span,
    ))
}

fn parse_function_call(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    let mut inner_rules = pair.into_inner();
    let name = inner_rules.next().unwrap().as_str();
    let mut args = Vec::new();

    while let Some(rule) = inner_rules.next() {
        args.push(parse_rule(rule)?);
    }

    Ok(Box::new(TimuAst::FunctionCall {
        compiler: false,
        name,
        args,
    }))
}

fn build_binary<'a>(
    lhs: Result<Box<TimuAst<'a>>, TimuParserError>,
    op: Pair<'a, Rule>,
    rhs: Result<Box<TimuAst<'a>>, TimuParserError>,
) -> Result<Box<TimuAst<'a>>, TimuParserError> {
    Ok(Box::new(TimuAst::BinaryOperation {
        left: lhs?,
        operator: op.as_str().chars().nth(0).unwrap(),
        right: rhs?,
    }))
}

fn parse_binary_operation(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    let node = PRATT
        .map_primary(|rule| parse_rule(rule))
        .map_infix(build_binary)
        .parse(pair.into_inner());

    Ok(node?)
}

pub fn parser<'a>(file: Rc<SourceFile<'a>>) -> Result<TimuFileAst<'a>, TimuParserError> {
    match TimuParser::parse(Rule::file, file.code()) {
        Ok(pairs) => parse_file(pairs, file),
        Err(err) => {
            let (line, column) = match err.line_col {
                LineColLocation::Pos((line, column)) => (line, column),
                LineColLocation::Span((line, column), _) => (line, column),
            };

            Err(TimuParserError::new_with_info(
                line,
                column,
                "could not parsed".to_string(),
            ))
        }
    }
}

/* *************************************************** TRAITS ***************************************************** */
/* ************************************************* IMPLEMENTS *************************************************** */

impl TimuParserError {
    pub fn new(pair: &Pair<'_, Rule>, error: String) -> Self {
        let (line, column) = pair.line_col();
        Self {
            line,
            column,
            error,
        }
    }

    pub fn new_with_info(line: usize, column: usize, error: String) -> Self {
        Self {
            line,
            column,
            error,
        }
    }

    pub fn print(&self, code: &str) {
        let mut lines = code.lines();
        if let Some(code_line) = lines.nth(self.line - 1) {
            println!("Error at {}:{}", self.line, self.column);
            println!("-> {}", self.error);
            println!("");
            println!("Code: ");
            println!("{}", code_line);
            println!("{}^", "-".repeat(self.column - 1));
        }
    }
}

/* ********************************************** TRAIT IMPLEMENTS ************************************************ */
/* ************************************************* MACROS CALL ************************************************** */
/* ************************************************** UNIT TESTS ************************************************** */
/* **************************************************************************************************************** */
