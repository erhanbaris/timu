use std::rc::Rc;

use ast::FileAst;
use error::{handle_builder, handle_parser};
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
pub fn process_code<'base>(state: &'base State) -> Result<FileAst<'base>, ()> {
    let response = parser::parse(state).finish();
    Ok(handle_parser(response).unwrap())
}

#[allow(clippy::result_unit_err)]
pub fn process_ast(files: Vec<Rc<FileAst<'_>>>) -> Result<TirContext<'_>, ()> {
    let response = crate::tir::build(files);
    handle_builder(response)
}
