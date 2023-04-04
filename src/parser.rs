use core::panic;

use pest::{Parser, iterators::{Pair, Pairs}, error::LineColLocation};
use pest_derive::Parser;

use crate::ast::{AccessType, FuncArg, FileAst, FunctionDefinitionAst, FunctionCallAst, BlockAst, PrimativeAst, ExpressionAst, StatementAst, BinaryOperationAst};

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
        if let Some(line) = lines.nth(self.line-1) {
            println!("Error: ");
            println!("{}", line);
            println!("{}^", "-".repeat(self.column-1));
        }
    }
}

fn parse_file(pairs: Pair<'_, Rule>) -> Result<FileAst, TimuParserError> {
    let mut functions = Vec::new();
    let inner_rules = pairs.into_inner();

    for pair in inner_rules {
        match pair.as_rule() {
            Rule::func_def => {
                functions.push(parse_func(pair));
            },
            _ => return Err(TimuParserError::new(pair, "Unknown "))
        }
    }

    Ok(FileAst { functions })
}

fn parse_primative(rule: Pair<'_, Rule>) -> Box<ExpressionAst> {
    let rule = rule.into_inner().next().unwrap();
    match rule.as_rule() {
        Rule::primative_integer => {
            Box::new(ExpressionAst::Primative(Box::new(PrimativeAst::I32(rule.as_str().parse::<i32>().unwrap()))))
        },
        Rule::primative_string => {
            let rule = rule.into_inner().peek().unwrap();
            Box::new(ExpressionAst::Primative(Box::new(PrimativeAst::String(rule.as_str().to_string()))))
        },
        Rule::primative_string_inner => {
            Box::new(ExpressionAst::Primative(Box::new(PrimativeAst::String(rule.as_str().to_string()))))
        },
        Rule::primative_bool => {
            Box::new(ExpressionAst::Primative(Box::new(PrimativeAst::Bool(match rule.as_str() {
                "true" => true,
                "false" => false,
                _ => panic!("Could not parsed bool type")
            }))))
        },
        _ => panic!("parse_expression could not parsed, {:#?}", rule)
    }
}

fn parse_expression(rule: Pair<'_, Rule>) -> Box<ExpressionAst> {

    match rule.as_rule() {
        Rule::primative => parse_primative(rule),
        Rule::func_call => {
            Box::new(ExpressionAst::FunctionCall(parse_function_call(rule.into_inner())))
        },
        Rule::binary_operation => {
            let mut inner = rule.into_inner();

            let left = parse_expression(inner.next().unwrap());
            let operator = inner.next().unwrap().as_str().chars().next().unwrap();
            let right = parse_expression(inner.next().unwrap());
    
            Box::new(ExpressionAst::BinaryOperation(Box::new(BinaryOperationAst { left, operator, right })))
        }
        Rule::expression => parse_expression(rule.into_inner().next().unwrap()),
        _ => panic!("parse_expression could not parsed, {:#?}", rule)
    }
}

fn parse_function_call(rules: Pairs<'_, Rule>) -> Box<FunctionCallAst> {
    let mut is_compiler_function = false;
    let mut name = String::new();
    let mut args = Vec::new();

    for rule in rules {
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

                    println!("{:#?}", arg);
                    args.push(parse_expression(arg));
                }
            },
            _ => panic!("123")
        }
    }

    Box::new(FunctionCallAst {
        compiler: is_compiler_function,
        name,
        args
    })
}

fn parse_statement(rule: Pair<'_, Rule>) -> Box<StatementAst> {
    let rule_type = rule.as_rule();
    
    match rule_type {
        Rule::func_call => Box::new(StatementAst::FunctionCall(parse_function_call(rule.into_inner()))),
        _ => panic!("parse_function_call, {:#?}", rule)
    }
}

fn parse_func(pairs: Pair<'_, Rule>) -> Box<FunctionDefinitionAst> {
    let inner_rules = pairs.into_inner();
    let mut access = AccessType::Private;
    let mut name = String::new();
    let mut return_type = "_".to_string();
    let mut args = Vec::new();
    let mut statements = Vec::new();

    for rule in inner_rules {

        match rule.as_rule() {
            Rule::func_return_type => return_type = rule.as_str().to_string(),
            Rule::func_name => name = rule.as_str().to_string(),
            Rule::keyword_public => access = AccessType::Public,
            Rule::func_arg => {
                let mut rules = rule.into_inner();
                
                args.push(FuncArg {
                    name: rules.next().unwrap().as_str().to_string(),
                    arg_type: rules.next().unwrap().as_str().to_string()
                })
            }
            Rule::func_body => {
                let rules = rule.into_inner();
                
                for rule in rules {
                    match rule.as_rule() {
                        Rule::statement => statements.push(parse_statement(rule.into_inner().peek().unwrap())),
                        _ => ()
                    }
                }
            }
            _ => {}
        }
    }

    let body = Box::new(BlockAst { statements });
    Box::new(FunctionDefinitionAst { access, name, args, body, return_type })
}

pub fn parser(code: &str) -> Result<FileAst, TimuParserError> {
    match TimuParser::parse(Rule::file, code) {
        Ok(mut parse) => parse_file(parse.nth(0).unwrap()),
        Err(err) => {
            let (line, column) = match err.line_col {
                LineColLocation::Pos((line, column)) => (line, column),
                LineColLocation::Span((line, column), _) => (line, column)
            };

            Err(TimuParserError::new_with_info(line, column, "error"))
        }
    }
}
