use std::rc::Rc;

use pest::error::InputLocation;
use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest::{
    Parser,
    error::LineColLocation,
    iterators::{Pair, Pairs},
};
use pest_derive::Parser;

use crate::ast::{
    AccessType, FuncArg, PrimitiveType, TimuAst, TimuBodyBlock, TimuFileAst, TimuFileStatementAst, TimuFunctionDefinitionAst, TimuTypeDefinitionAst, UnaryType,
    VariableType,
};
use crate::file::SourceFile;
use crate::span::{Span, Spanned};

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

#[derive(Parser)]
#[grammar = "../assets/timu.pest"]
pub struct TimuParser;
impl TimuParser {}

#[derive(Debug, PartialEq)]
pub struct TimuParserError<'a> {
    pub message: String,
    pub span: Span<'a>,
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
        .op(Op::infix(Rule::modulo, Assoc::Left) | Op::infix(Rule::divide, Assoc::Left) | Op::infix(Rule::multiply, Assoc::Left))
        .op(Op::infix(Rule::exponent, Assoc::Right))
}

fn check_rule<'a>(pair: &Pair<'a, Rule>, expected_rule: Rule, file: Rc<SourceFile<'a>>) -> Result<(), TimuParserError<'a>> {
    let actual_rule = pair.as_rule();

    if actual_rule != expected_rule {
        return Err(TimuParserError::new(format!("Expected '{:?}', but found '{:?}'", expected_rule, actual_rule), Span::new(file, pair.as_span())));
    }

    Ok(())
}

fn to_string_vector<'a>(rule: Pair<'a, Rule>) -> Vec<&'a str> {
    rule.into_inner().map(|item| item.as_str()).collect()
}

fn parse_import<'a>(pair: Pair<'a, Rule>) -> Result<Box<TimuAst<'a>>, TimuParserError<'a>> {
    let mut inner_rules = pair.into_inner();

    let path = to_string_vector(inner_rules.next().unwrap());
    let name = match inner_rules.next() {
        Some(rule) => rule.as_str(),
        None => path.last().unwrap(),
    };

    Ok(Box::new(TimuAst::Import {
        path,
        name,
    }))
}

fn parse_variable_definition<'a>(pair: Pair<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<Box<TimuAst<'a>>, TimuParserError<'a>> {
    let mut inner_rules = pair.into_inner();
    let mut pair = inner_rules.next().unwrap();

    let var_type = match pair.as_str() {
        "var" => VariableType::Var,
        "const" => VariableType::Const,
        _type => {
            return Err(TimuParserError::new(format!("Unsupported variable type ({})", _type), Span::new(file, pair.as_span())));
        }
    };

    let name = inner_rules.next().unwrap().as_str();
    let type_annotation = to_string_vector(inner_rules.next().unwrap());

    Ok(Box::new(TimuAst::DefAssignment {
        r#type: var_type,
        type_annotation,
        name,
        data: parse_rule(inner_rules.next().unwrap(), file)?,
    }))
}

fn parse_variable_assign<'a>(pair: Pair<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<Box<TimuAst<'a>>, TimuParserError<'a>> {
    let mut inner_rules = pair.into_inner();
    let name = inner_rules.next().unwrap().as_str();

    Ok(Box::new(TimuAst::Assignment {
        name,
        data: parse_rule(inner_rules.next().unwrap(), file)?,
    }))
}

fn parse_function_args<'a>(pair: Pair<'a, Rule>) -> Result<Vec<TimuFunctionArg>, TimuParserError<'a>> {
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

fn parse_define_type_field<'a>(pair: Pair<'a, Rule>) -> Result<TimuTypeField, TimuParserError<'a>> {
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

fn parse_type_definition<'a>(pair: Pair<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<Spanned<'a, TimuTypeDefinitionAst<'a>>, TimuParserError<'a>> {
    let span = pair.as_span();
    let mut inner_rules = pair.into_inner();
    let mut fields = Vec::new();
    let mut functions = Vec::new();
    let name_pair = inner_rules.next().unwrap();
    let name = Spanned::new(name_pair.as_str(), file.clone(), name_pair.as_span());

    /* Fields or function definitions on the type */
    if let Some(fields_rule) = inner_rules.next() {
        let mut type_fields = fields_rule.into_inner();

        for pair in type_fields.into_iter() {
            match pair.as_rule() {
                Rule::define_type_field => fields.push(parse_define_type_field(pair)?),
                Rule::define_function => functions.push(parse_function_definition(pair, file.clone())?),
                _ => {
                    return Err(TimuParserError::new("Not valid for type definition".to_string(), Span::new(file, pair.as_span())));
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

fn parse_file<'a>(pairs: Pairs<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<TimuFileAst<'a>, TimuParserError<'a>> {
    let mut statements = Vec::new();

    for pair in pairs {
        /* End of file */
        if pair.as_rule() == Rule::EOI {
            continue;
        }

        statements.push(parse_file_statement(pair, file.clone())?);
    }

    Ok(TimuFileAst {
        statements,
        file,
    })
}

fn parse_string<'a>(pair: Pair<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<Box<TimuAst<'a>>, TimuParserError<'a>> {
    check_rule(&pair, Rule::primitive_type_string, file)?;

    let mut inner_rules = pair.into_inner();
    let string_data = inner_rules.next().unwrap();

    Ok(Box::new(TimuAst::Primitive(PrimitiveType::String(string_data.as_str()))))
}

fn parse_bool<'a>(pair: Pair<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<Box<TimuAst<'a>>, TimuParserError<'a>> {
    check_rule(&pair, Rule::primitive_type_boolean, file)?;
    Ok(Box::new(TimuAst::Primitive(PrimitiveType::Bool(match pair.as_str() {
        "true" => true,
        _ => false,
    }))))
}

fn parse_integer<'a>(pair: Pair<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<Box<TimuAst<'a>>, TimuParserError<'a>> {
    check_rule(&pair, Rule::primitive_type_integer, file.clone())?;

    let integer_rule = pair.into_inner().next().unwrap();
    let integer = match integer_rule.as_rule() {
        Rule::integer_decimal => integer_rule.as_str().replace("_", "").parse::<i128>().unwrap(),
        Rule::integer_binary => i128::from_str_radix(integer_rule.as_str(), 2).unwrap_or_default(),
        Rule::integer_hexadecimal => i128::from_str_radix(integer_rule.as_str(), 16).unwrap_or_default(),
        Rule::integer_octal => i128::from_str_radix(integer_rule.as_str(), 8).unwrap_or_default(),
        Rule::integer_zero => 0,
        _ => 0,
    };

    if I8_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::I8(integer as i8))))
    } else if U8_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::U8(integer as u8))))
    } else if I16_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::I16(integer as i16))))
    } else if U16_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::U16(integer as u16))))
    } else if I32_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::I32(integer as i32))))
    } else if U32_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::U32(integer as u32))))
    } else if I64_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::I64(integer as i64))))
    } else if U64_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primitive(PrimitiveType::U64(integer as u64))))
    } else {
        Err(TimuParserError::new("Integer type not supported".to_string(), Span::new(file, integer_rule.as_span())))
    }
}

fn parse_float<'a>(pair: Pair<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<Box<TimuAst<'a>>, TimuParserError<'a>> {
    check_rule(&pair, Rule::primitive_type_float, file)?;
    Ok(Box::new(TimuAst::Primitive(PrimitiveType::Float(pair.as_str().replace("_", "").parse::<f32>().unwrap_or_default()))))
}

fn parse_unary<'a>(pair: Pair<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<Box<TimuAst<'a>>, TimuParserError<'a>> {
    check_rule(&pair, Rule::unary, file.clone())?;
    let mut into_inner = pair.into_inner();

    let unary = into_inner.next().unwrap();
    let unary = match unary.as_str() {
        "-" => UnaryType::Minus,
        "+" => UnaryType::Plus,
        "!" => UnaryType::LogicalNot,
        _ => {
            return Err(TimuParserError::new("Unsupported unary type".to_string(), Span::new(file.clone(), unary.as_span())));
        }
    };

    let expression = parse_rule(into_inner.next().unwrap(), file)?;
    Ok(Box::new(TimuAst::Unary(unary, expression)))
}

fn parse_rule<'a>(pair: Pair<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<Box<TimuAst<'a>>, TimuParserError<'a>> {
    let rule = pair.as_rule();
    match rule {
        Rule::import => parse_import(pair),

        // Primitive
        Rule::primitive_type_string => parse_string(pair, file),
        Rule::primitive_type_boolean => parse_bool(pair, file),
        Rule::primitive_type_integer => parse_integer(pair, file),
        Rule::primitive_type_float => parse_float(pair, file),

        // Unary
        Rule::unary => parse_unary(pair, file),
        Rule::infix => parse_binary_operation(pair, file),

        Rule::function_call => parse_function_call(pair, file),

        // Assignment
        Rule::define_variable => parse_variable_definition(pair, file),
        Rule::assign_variable => parse_variable_assign(pair, file),

        // Types
        //Rule::define_type => parse_type_definition(pair),
        _ => {
            return Err(TimuParserError::new("Unknown syntax".to_string(), Span::new(file, pair.as_span())));
        }
    }
}

fn parse_file_statement<'a>(pair: Pair<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<Spanned<'a, TimuFileStatementAst<'a>>, TimuParserError<'a>> {
    let rule = pair.as_rule();
    let span = pair.as_span();
    let value = match rule {
        // Rule::import => parse_import(pair),
        // Rule::define_variable =>  parse_variable_definition(pair),
        Rule::define_type => TimuFileStatementAst::TypeDefinition(parse_type_definition(pair, file.clone())?),
        Rule::define_function => TimuFileStatementAst::FunctionDefinition(parse_function_definition(pair, file.clone())?),
        _ => {
            return Err(TimuParserError::new("unknown syntax".to_string(), Span::new(file, pair.as_span())));
        }
    };

    Ok(Spanned::new(value, file, span))
}

fn parse_type_annotation<'a>(pair: Pair<'a, Rule>) -> Vec<&'a str> {
    let mut type_info = Vec::new();

    for type_rule in pair.into_inner() {
        type_info.push(type_rule.as_str());
    }

    type_info
}

fn parse_function_definition<'a>(pair: Pair<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<Spanned<'a, TimuFunctionDefinitionAst<'a>>, TimuParserError<'a>> {
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
                    statements.push(parse_rule(pair, file.clone())?);
                }
            }
            _ => {}
        }
    }

    let body = TimuBodyBlock {
        statements,
    };
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

fn parse_function_call<'a>(pair: Pair<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<Box<TimuAst<'a>>, TimuParserError<'a>> {
    let mut inner_rules = pair.into_inner();
    let name = inner_rules.next().unwrap().as_str();
    let mut args = Vec::new();

    while let Some(rule) = inner_rules.next() {
        args.push(parse_rule(rule, file.clone())?);
    }

    Ok(Box::new(TimuAst::FunctionCall {
        compiler: false,
        name,
        args,
    }))
}

fn build_binary<'a>(
    lhs: Result<Box<TimuAst<'a>>, TimuParserError<'a>>, op: Pair<'a, Rule>, rhs: Result<Box<TimuAst<'a>>, TimuParserError<'a>>,
) -> Result<Box<TimuAst<'a>>, TimuParserError<'a>> {
    Ok(Box::new(TimuAst::BinaryOperation {
        left: lhs?,
        operator: op.as_str().chars().nth(0).unwrap(),
        right: rhs?,
    }))
}

fn parse_binary_operation<'a>(pair: Pair<'a, Rule>, file: Rc<SourceFile<'a>>) -> Result<Box<TimuAst<'a>>, TimuParserError<'a>> {
    let node = PRATT.map_primary(|rule| parse_rule(rule, file.clone())).map_infix(build_binary).parse(pair.into_inner());

    Ok(node?)
}

pub fn parse<'a>(file: Rc<SourceFile<'a>>) -> Result<TimuFileAst<'a>, TimuParserError<'a>> {
    match TimuParser::parse(Rule::file, file.code()) {
        Ok(pairs) => parse_file(pairs, file.clone()),
        Err(error) => {
            let (start, end) = match error.location {
                InputLocation::Pos(position) => (position, position),
                InputLocation::Span((start, end)) => (start, end),
            };

            Err(TimuParserError::new("Parser issue".to_string(), Span::new(file.clone(), pest::Span::new(file.code(), start, end).unwrap())))
        }
    }
}

impl<'a> TimuParserError<'a> {
    pub fn new(message: String, span: Span<'a>) -> Self {
        Self {
            message,
            span,
        }
    }
}
