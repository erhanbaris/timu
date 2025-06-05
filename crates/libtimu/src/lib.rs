use std::rc::Rc;

use ast::FileAst;
use error::handle_parser;
use nom::Finish;
use nom_tools::State;
use tir::{TirContext, TirError};


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
pub fn process_code<'base>(state: &'base State) -> Result<FileAst<'base>, TirError> {
    let response = parser::parse(state).finish();
    handle_parser(response)
}

#[allow(clippy::result_unit_err)]
pub fn process_ast(files: Vec<Rc<FileAst<'_>>>) -> Result<TirContext<'_>, TirError> {
    crate::tir::build(files)
}
