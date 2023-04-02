use pest::{Parser, iterators::{Pair, Pairs}};
use pest_derive::Parser;

use crate::ast::{AccessType, Ast, FuncArg, FileAst, FunctionDefinitionAst, FunctionCallAst, BlockAst, PrimativeAst};

#[derive(Parser)]
#[grammar = "../assets/timu.pest"]
pub struct TimuParser;


fn parse_file(pairs: Pair<'_, Rule>) -> Box<FileAst> {
    let mut functions = Vec::new();
    let inner_rules = pairs.into_inner();

    for pair in inner_rules {
        match pair.as_rule() {
            Rule::func_def => {
                functions.push(parse_func(pair));
            },
            _ => {}
        }
    }

    Box::new(FileAst { functions })
}

fn parse_expression(rule: Pair<'_, Rule>) -> Box<dyn Ast> {
    match rule.as_rule() {
        Rule::term_string => {
            let rule = rule.into_inner().peek().unwrap();
            Box::new(PrimativeAst::String(rule.as_str().to_string()))
        },
        Rule::func_call => {
            parse_function_call(rule.into_inner())
        },
        _ => panic!("parse_expression")
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

fn parse_statement(rule: Pair<'_, Rule>) -> Box<dyn Ast> {
    let rule_type = rule.as_rule();
    
    match rule_type {
        Rule::func_call => parse_function_call(rule.into_inner()),
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

pub fn parser(code: &str) -> Box<FileAst> {
    let mut parse = TimuParser::parse(Rule::file, code).unwrap();
    parse_file(parse.nth(0).unwrap())
}
