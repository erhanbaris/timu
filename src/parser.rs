use core::panic;

use pest::{Parser, iterators::{Pair, Pairs}, error::LineColLocation};
use pest_derive::Parser;

use crate::ast::{AccessType, FuncArg, TimuAst};

#[derive(Parser)]
#[grammar = "../assets/timu.pest"]
pub struct TimuParser;

#[derive(Debug)]
pub struct TimuParserError {
    pub line: usize,
    pub column: usize,
    pub error: &'static str
}

impl TimuParserError {
    pub fn new(pair: Pair<'_, Rule>, error: &'static str) -> Self {
        let (line, column) = pair.line_col();
        Self {
            line,
            column,
            error
        }
    }

    pub fn new_with_info(line: usize, column: usize, error: &'static str) -> Self {
        Self {
            line,
            column,
            error
        }
    }

    pub fn print(&self, code: &str) {
        let mut lines = code.lines();
        if let Some(code_line) = lines.nth(self.line-1) {
            println!("Error at {}:{}", self.line, self.column);
            println!("-> {}", self.error);
            println!("");
            println!("Code: ");
            println!("{}", code_line);
            println!("{}^", "-".repeat(self.column-1));
        }
    }
}

fn parse_import(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    let inner_rules = pair.into_inner();
    let mut path = Vec::new();
    let mut name = String::new();

    for rule in inner_rules {
        match rule.as_rule() {
            Rule::importproperties => {
                for property in rule.into_inner() {
                    path.push(property.as_str().to_string())
                }
            },
            Rule::importname  => name = rule.as_str().to_string(),
            _ => return Err(TimuParserError::new(rule, "could not parse import statement"))
        }
    }

    if name.is_empty() {
        name = path.last().unwrap().to_string();
    }

    Ok(Box::new(TimuAst::Import { path, name }))
}

fn parse_file(pairs: Pairs<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    let mut statements = Vec::new();

    println!("{:#?}", &pairs);

    for pair in pairs {
        match pair.as_rule() {
            Rule::def => statements.push(parse_func(pair)?),
            Rule::import => statements.push(parse_import(pair)?),
            Rule::EOI => {},
            //Rule::assign_stmt => statements.push(parse_assignment(pair)?),
            _ => return Err(TimuParserError::new(pair, "unknown syntax"))
        }
    }

    Ok(Box::new(TimuAst::File { statements }))
}
/*
fn parse_assignment(pairs: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError> {
    let mut rule = pairs.into_inner();
    let variable = rule.next().unwrap();
    let data = parse_expression(rule.next().unwrap())?;

    let variable = match variable.as_rule() {
        Rule::assign_name => Box::new(TimuAst::Ident(variable.as_str().to_string())),
        _ => return Err(TimuParserError::new(variable, "variable name could not parsed"))
    };
    
    Ok(Box::new(TimuAst::Assignment { variable, data }))
}

fn parse_primative(rule: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError>  {
    let rule = rule.into_inner().next().unwrap();
    match rule.as_rule() {
        Rule::primative_integer => {
            Ok(Box::new(TimuAst::I32(rule.as_str().parse::<i32>().unwrap())))
        },
        Rule::primative_string => {
            let rule = rule.into_inner().peek().unwrap();
            Ok(Box::new(TimuAst::String(rule.as_str().to_string())))
        },
        Rule::primative_string_inner => {
            Ok(Box::new(TimuAst::String(rule.as_str().to_string())))
        },
        Rule::primative_bool => {
            Ok(Box::new(TimuAst::Bool(match rule.as_str() {
                "true" => true,
                "false" => false,
                _ => return Err(TimuParserError::new(rule, "could not parsed bool type"))
            })))
        },
        _ => Err(TimuParserError::new(rule, "primative type could not parsed"))
    }
}

fn parse_expression(rule: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError>  {

    match rule.as_rule() {
        Rule::primative => parse_primative(rule),
        Rule::func_call => parse_function_call(rule),
        Rule::binary_operation => {
            let mut inner = rule.into_inner();

            let left = parse_expression(inner.next().unwrap())?;
            let operator = inner.next().unwrap().as_str().chars().next().unwrap();
            let right = parse_expression(inner.next().unwrap())?;
    
            Ok(Box::new(TimuAst::BinaryOperation{ left, operator, right }))
        }
        Rule::expression => parse_expression(rule.into_inner().next().unwrap()),
        _ => Err(TimuParserError::new(rule, "expression could not parsed"))
    }
}

fn parse_function_call(rules: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError>  {
    let mut is_compiler_function = false;
    let mut name = String::new();
    let mut args = Vec::new();

    for rule in rules.into_inner() {
        match rule.as_rule() {
            Rule::func_call_compiler => {
                is_compiler_function = true;
            },
            Rule::var_name => {
                name = rule.as_str().to_string();
            },
            Rule::func_call_args => {
                let args_inner = rule.into_inner();
                for arg in args_inner {
                    args.push(parse_expression(arg)?);
                }
            },
            _ => panic!("123")
        }
    }

    Ok(Box::new(TimuAst::FunctionCall {
        compiler: is_compiler_function,
        name,
        args
    }))
}

fn parse_statement(rule: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError>  {
    let rule_type = rule.as_rule();
    
    match rule_type {
        Rule::func_call => parse_function_call(rule),
        Rule::assign_stmt => parse_assignment(rule),
        _ => panic!("parse_function_call, {:#?}", rule)
    }
}
*/

fn parse_type_annotation(pair: Pair<'_, Rule>) -> Vec<String>  {
    let mut type_info = Vec::new();

    for type_rule in pair.into_inner() {
        type_info.push(type_rule.as_str().to_string());
    }

    type_info
}

fn parse_func(pair: Pair<'_, Rule>) -> Result<Box<TimuAst>, TimuParserError>  {
    let inner_rules = pair.into_inner();
    let mut access = AccessType::Private;
    let mut name = String::new();
    let mut return_type = Vec::new();
    let mut args = Vec::new();
    let mut statements = Vec::new();

    for rule in inner_rules {

        match rule.as_rule() {
            Rule::maybe_type_annotation => return_type = parse_type_annotation(rule),
            Rule::ident => name = rule.as_str().to_string(),
            //Rule::keyword_public => access = AccessType::Public,
            Rule::defarguments => {
                
                for arg_rule in rule.into_inner() {
                    println!("{:?}", arg_rule);                    
                }
            }
            Rule::body_block => {
                let rules = rule.into_inner();
                
                for rule in rules {
                    println!("{:?}", rule);
                    /*match rule.as_rule() {
                        Rule::statement => statements.push(parse_statement(rule.into_inner().peek().unwrap())?),
                        _ => ()
                    }*/
                }
            }
            _ => {}
        }
    }

    let body = Box::new(TimuAst::Block { statements });
    Ok(Box::new(TimuAst::FunctionDefinition { access, name, args, body, return_type }))
}

pub fn parser(code: &str) -> Result<Box<TimuAst>, TimuParserError> {
    match TimuParser::parse(Rule::file, code) {
        Ok(parse) => parse_file(parse),
        Err(err) => {
            let (line, column) = match err.line_col {
                LineColLocation::Pos((line, column)) => (line, column),
                LineColLocation::Span((line, column), _) => (line, column)
            };

            Err(TimuParserError::new_with_info(line, column, "could not parsed"))
        }
    }
}
