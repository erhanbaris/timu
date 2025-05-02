mod ast;
mod file;

#[rustfmt::skip]
mod parser;
mod error;
mod nom_tools;
mod tir;

#[cfg(test)]
mod tests;

use std::{error::Error, rc::Rc};

use error::handle_parser;
use file::SourceFile;
use nom::Finish;
use nom_tools::State;

fn main() -> Result<(), Box<dyn Error>> {
    let source_file = Rc::new(SourceFile::new("<memory>".into(), "use test1.test2;"));

    let state = State {
        file: source_file.clone(),
    };

    let response = parser::parse(state).finish();
    let file_ast = handle_parser(response)?;

    for ast in file_ast.statements.iter() {
        println!("{}", ast);
    }

    crate::tir::build(vec![file_ast])?;

    Ok(())
}
