use std::{borrow::Cow, rc::Rc};

use ast::FileAst;
use error::{handle_builder, handle_parser};
use file::SourceFile;
use nom::Finish;
use nom_tools::State;
use tir::TirContext;


pub mod ast;
pub mod file;

#[rustfmt::skip]
pub mod parser;
pub mod error;
pub mod nom_tools;
pub mod tir;
pub mod map;

#[cfg(test)]
mod tests;

#[allow(clippy::result_unit_err)]
pub fn process_code<'base>(path: Vec<Cow<'base, str>>, code: &'base str) -> Result<FileAst<'base>, ()> {
    let file = Rc::new(SourceFile::new(path, code));
    let state = State {
        file,
        indexer: Default::default(),
    };

    let response = parser::parse(state).finish();
    handle_parser(response)
}

#[allow(clippy::result_unit_err)]
pub fn process_ast(files: Vec<Rc<FileAst<'_>>>) -> Result<TirContext<'_>, ()> {
    let response = crate::tir::build(files);
    handle_builder(response)
}
