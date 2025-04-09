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
    AccessType, FuncArg, PrimativeType, TimuAst, TimuAstType, UnaryType, VariableType,
};

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

#[derive(Debug, PartialEq)]
pub struct TimuParserError {
    pub line: usize,
    pub column: usize,
    pub error: String,
}

#[derive(Debug, PartialEq)]
pub struct TimuTypeField {
    pub is_pub: bool,
    pub name: String,
    pub nullable: bool,
    pub type_name: String,
}

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

fn to_string_vector(rule: Pair<Rule>) -> Vec<String> {
    rule.into_inner()
        .map(|item| item.as_str().to_string())
        .collect()
}

fn parse_import(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    let mut inner_rules = pair.into_inner();

    let path = to_string_vector(inner_rules.next().unwrap());
    let name = match inner_rules.next() {
        Some(rule) => rule.as_str().to_string(),
        None => path.last().unwrap().to_string(),
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

    let name = inner_rules.next().unwrap().as_str().to_string();
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
    let name = inner_rules.next().unwrap().as_str().to_string();

    Ok(Box::new(TimuAst::Assignment {
        name,
        data: parse_rule(inner_rules.next().unwrap())?,
    }))
}

fn parse_type_definition(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    let mut inner_rules = pair.into_inner();
    let name = inner_rules.next().unwrap().as_str().to_string();
    let mut type_fields = inner_rules.next().unwrap().into_inner();

    let mut fields = Vec::new();

    for item in type_fields.into_iter() {
        match item.as_rule() {
            Rule::deftypefield => {
                let mut inner = item.into_inner();
                let is_pub = match inner.next() {
                    Some(rule) => rule.as_str() == "pub",
                    None => false,
                };

                let name = inner.next().unwrap().as_str().to_string();
                let nullable = if let Some(Rule::nullable) = inner.peek().map(|item| item.as_rule())
                {
                    let _ = inner.next();
                    true
                } else {
                    false
                };

                let type_name = inner.next().unwrap().as_str().to_string();
                fields.push(TimuTypeField {
                    is_pub,
                    name,
                    nullable,
                    type_name,
                });
            }
            Rule::deffunc => {}
            _ => {
                return Err(TimuParserError::new(
                    &item,
                    "Not valid for type definition".to_string(),
                ))
            }
        }
    }

    Ok(Box::new(TimuAst::TypeDefinition { name, fields }))
}

fn parse_file(pairs: Pairs<'_, Rule>) -> Result<TimuAst, TimuParserError> {
    let mut statements = Vec::new();

    for pair in pairs {
        /* End of file */
        if pair.as_rule() == Rule::EOI {
            continue;
        }

        statements.push(parse_rule(pair)?);
    }

    Ok(TimuAst::File { statements })
}

fn parse_string(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    check_rule(&pair, Rule::string)?;

    let mut inner_rules = pair.into_inner();
    let string_data = inner_rules.next().unwrap();

    Ok(Box::new(TimuAst::Primative(PrimativeType::String(
        string_data.as_str().to_string(),
    ))))
}

fn parse_bool(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    check_rule(&pair, Rule::boolean)?;
    Ok(Box::new(TimuAst::Primative(PrimativeType::Bool(
        match pair.as_str() {
            "true" => true,
            _ => false,
        },
    ))))
}

fn parse_integer(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    check_rule(&pair, Rule::integer)?;

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
        Ok(Box::new(TimuAst::Primative(PrimativeType::I8(
            integer as i8,
        ))))
    } else if U8_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primative(PrimativeType::U8(
            integer as u8,
        ))))
    } else if I16_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primative(PrimativeType::I16(
            integer as i16,
        ))))
    } else if U16_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primative(PrimativeType::U16(
            integer as u16,
        ))))
    } else if I32_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primative(PrimativeType::I32(
            integer as i32,
        ))))
    } else if U32_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primative(PrimativeType::U32(
            integer as u32,
        ))))
    } else if I64_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primative(PrimativeType::I64(
            integer as i64,
        ))))
    } else if U64_RANGE.contains(&integer) {
        Ok(Box::new(TimuAst::Primative(PrimativeType::U64(
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
    check_rule(&pair, Rule::float)?;
    Ok(Box::new(TimuAst::Primative(PrimativeType::Float(
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
    match pair.as_rule() {
        Rule::import => parse_import(pair),

        // Primative
        Rule::string => parse_string(pair),
        Rule::boolean => parse_bool(pair),
        Rule::integer => parse_integer(pair),
        Rule::float => parse_float(pair),

        // Unary
        Rule::unary => parse_unary(pair),
        Rule::infix => parse_binary_operation(pair),

        Rule::deffunc => parse_function_definition(pair),
        Rule::func_call => parse_function_call(pair),

        // Assignment
        Rule::defvariable => parse_variable_definition(pair),
        Rule::assignvariable => parse_variable_assign(pair),

        // Types
        Rule::deftype => parse_type_definition(pair),

        _ => return Err(TimuParserError::new(&pair, "unknown syntax".to_string())),
    }
}

fn parse_type_annotation(pair: Pair<'_, Rule>) -> Vec<String> {
    let mut type_info = Vec::new();

    for type_rule in pair.into_inner() {
        type_info.push(type_rule.as_str().to_string());
    }

    type_info
}

fn parse_function_definition(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    let inner_rules = pair.into_inner();
    let access = AccessType::Private;
    let mut name = String::new();
    let mut return_type = Vec::new();
    let mut args = Vec::new();
    let mut statements = Vec::new();

    for rule in inner_rules {
        match rule.as_rule() {
            Rule::maybe_type_annotation => return_type = parse_type_annotation(rule),
            Rule::ident => name = rule.as_str().to_string(),
            Rule::deffuncarguments => {
                for arg_rule in rule.into_inner() {
                    let mut arg_rule = arg_rule.into_inner();

                    let mut func_argument = FuncArg {
                        name: arg_rule.next().unwrap().as_str().to_string(),
                        arg_type: TimuAstType::new(),
                    };

                    for item in arg_rule {
                        func_argument.arg_type = parse_type_annotation(item)
                    }

                    args.push(func_argument);
                }
            }
            Rule::body_block => {
                let pairs = rule.into_inner();

                for pair in pairs {
                    statements.push(parse_rule(pair)?);
                }
            }
            _ => {}
        }
    }

    let body = Box::new(TimuAst::Block { statements });
    Ok(Box::new(TimuAst::FunctionDefinition {
        access,
        name,
        args,
        body,
        return_type,
    }))
}

fn parse_function_call(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    let mut inner_rules = pair.into_inner();
    let name = inner_rules.next().unwrap().as_str().to_string();
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

fn build_binary(
    lhs: Result<Box<TimuAst>, TimuParserError>,
    op: Pair<'_, Rule>,
    rhs: Result<Box<TimuAst>, TimuParserError>,
) -> Result<Box<TimuAst>, TimuParserError> {
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

pub fn parser(code: &str) -> Result<TimuAst, TimuParserError> {
    match TimuParser::parse(Rule::file, code) {
        Ok(parse) => parse_file(parse),
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

#[cfg(test)]
mod test {
    use rstest::*;

    use crate::ast::{PrimativeType, TimuAst, VariableType};

    use super::TimuParserError;

    #[macro_export]
    macro_rules! auto_parse {
        ($name:ident, $code: expr) => {
            #[test]
            fn $name() -> Result<(), super::TimuParserError> {
                super::parser($code)?;
                Ok(())
            }
        };
    }

    /* Import tests */
    // auto_parse!(import_test_1, "@use(test);");
    // auto_parse!(import_test_2, "@use(test.rrr);");
    // auto_parse!(import_test_3, "@use(test.rrr.ffff);");
    // auto_parse!(import_test_4, "@use(test.rrr.ffff) as test;");

    /* Function tests */
    // auto_parse!(func_test_1, "func data() {}");
    // auto_parse!(func_test_2, "func data(test:i32) {}");
    // auto_parse!(func_test_3, "func data(test1:i32, test2:i32) {}");
    // auto_parse!(func_test_4, "func data(): i32 {}");
    // auto_parse!(func_test_5, "func data(test2:i32): i32 {}");
    // auto_parse!(func_test_6, "func data(test1:i32, test2:i32): i32 {}");

    /* Variable tests */
    #[rstest]
    #[case("var $test1 = 123;")]
    #[case("var $test1: i8 = 123;")]
    #[case("var $test1 = \"erhanbaris\";")]
    #[case("const $test1 = 123;")]
    #[case("const $test1: i32 = 123;")]
    #[case("const $test1 = \"erhanbaris\";")]
    #[case("const $test1 = true;")]
    #[case("const $test1 = 123.321;")]
    fn variable_def_test(#[case] code: &'_ str) -> Result<(), TimuParserError> {
        let ast = super::parser(code)?;
        println!("AST: {:?}", ast);
        Ok(())
    }
    #[rstest]
    #[case("var $test1 = 123;", TimuAst::File {
        statements: vec![
            Box::new(TimuAst::DefAssignment {
                r#type: VariableType::Var,
                type_annotation: [].to_vec(),
                name: "$test1".to_string(),
                data: Box::new(TimuAst::Primative(PrimativeType::I8(123)))
            }),
        ],
    })]
    #[case("const $test1 = \"erhanbaris\";", TimuAst::File {
        statements: vec![
            Box::new(TimuAst::DefAssignment {
                r#type: VariableType::Const,
                type_annotation: [].to_vec(),
                name: "$test1".to_string(),
                data: Box::new(TimuAst::Primative(PrimativeType::String("erhanbaris".to_string()))),
            }),
        ],
    })]
    fn variable2_def_test(
        #[case] code: &'_ str,
        #[case] expected: TimuAst,
    ) -> Result<(), TimuParserError> {
        let ast = super::parser(code)?;
        println!("AST: {:?}", ast);
        assert_eq!(ast, expected);
        Ok(())
    }
}
