mod ast;
mod file;

#[rustfmt::skip]
mod parser;
mod error;
mod nom_tools;
mod tir;

#[cfg(test)]
mod tests;

use std::{error::Error, path::PathBuf, rc::Rc};

use ast::FileAst;
use error::{ParseError, handle_parser};
use file::SourceFile;
use nom::Finish;
use nom_tools::State;

fn process_code<'a>(name: &'a str, path: PathBuf, code: &'a str) -> Result<FileAst<'a>, ParseError<'a>> {
    let file = Rc::new(SourceFile::new(name, path, code));

    let state = State {
        file,
    };

    let response = parser::parse(state).finish();
    handle_parser(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    let ast_1 = process_code("source1", "<memory>".into(), " class testclass {} ")?;
    let ast_2 = process_code("source2", "<memory>".into(), "use source1; use source1.testclass; use source1.testclass;")?;
    crate::tir::build(vec![ast_1.into(), ast_2.into()])?;

    Ok(())
}
