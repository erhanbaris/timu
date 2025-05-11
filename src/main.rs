mod ast;
mod file;

#[rustfmt::skip]
mod parser;
mod error;
mod nom_tools;
mod tir;

#[cfg(test)]
mod tests;

use std::{borrow::Cow, rc::Rc};

use ast::FileAst;
use error::{handle_builder, handle_parser};
use file::SourceFile;
use nom::Finish;
use nom_tools::State;
use simplelog::*;
use tir::TirContext;

fn process_code<'base>(path: Vec<Cow<'base, str>>, code: &'base str) -> Result<FileAst<'base>, ()> {
    let file = Rc::new(SourceFile::new(path, code));
    let state = State {
        file,
    };

    let response = parser::parse(state).finish();
    handle_parser(response)
}

fn process_ast(files: Vec<Rc<FileAst<'_>>>) -> Result<TirContext<'_>, ()> {
    let response = crate::tir::build(files);
    handle_builder(response)
}

fn main() -> Result<(), ()> {
    CombinedLogger::init(vec![TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)]).unwrap();

    let ast_1 = process_code(vec!["base1".into(), "test1".into(), "source1".into()], " class testclass1 {} ")?;
    let ast_9 = process_code(
        vec!["sub".into(), "source9".into()],
        r#"use base1.test1.source1.testclass1 as test;
func testfunction1(a: test): test {}"#,
    )?;

    process_ast(vec![ast_1.into(), ast_9.into()])?;

    Ok(())
}
