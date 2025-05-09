mod ast;
mod file;

#[rustfmt::skip]
mod parser;
mod error;
mod nom_tools;
mod tir;

#[cfg(test)]
mod tests;

use std::rc::Rc;

use ast::FileAst;
use error::{handle_builder, handle_parser};
use file::SourceFile;
use nom::Finish;
use nom_tools::State;
use simplelog::*;

fn process_code(path: Vec<String>, code: &'_ str) -> Result<FileAst<'_>, ()> {
    let file = Rc::new(SourceFile::new(path, code));
    let state = State {
        file,
    };

    let response = parser::parse(state).finish();
    handle_parser(response)
}

fn process_ast(files: Vec<Rc<FileAst>>) -> Result<(), ()> {
    let response = crate::tir::build(files);
    handle_builder(response)
}

fn main() -> Result<(), ()> {
    CombinedLogger::init(vec![TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)]).unwrap();

    let ast_1 = process_code(vec!["base1".to_string(), "test1".to_string(), "source1".to_string()], " class testclass1 {} ")?;
    let ast_9 = process_code(
        vec!["sub".to_string(), "source9".to_string()],
        r#"use base1.test1.source1.testclass1 as test;
func testfunction1(a: test): test {}"#,
    )?;

    process_ast(vec![ast_1.into(), ast_9.into()])?;

    Ok(())
}
