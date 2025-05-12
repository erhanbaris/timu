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
    let source_1 = process_code(vec!["lib".into()], " class testclass1 {} ")?;
    let source_2 = process_code(vec!["main".into()],
        r#"use lib.testclass1 as test;
func main(a: test): test {}"#,
    )?;

    let context = process_ast(vec![source_2.into(), source_1.into()])?;

    Ok(())
}
