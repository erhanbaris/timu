mod ast;
mod file;
// mod parser;
//mod span;
//mod sitter_parser;
#[rustfmt::skip]
mod parser;
mod error;
mod nom_tools;

#[cfg(test)]
mod tests;

use std::{error::Error, rc::Rc};

use error::handle_parser;
use file::SourceFile;
use nom::Finish;
use nom_tools::State;

fn main() -> Result<(), Box<dyn Error>> {
    let source_file = Rc::new(SourceFile::new("<memory>".into(), "extend Myclass : a, b, c {}"));

    let state = State {
        file: source_file.clone(),
    };

    let response = parser::parse(state).finish();
    let ast = handle_parser(response)?;

    for ast in ast.statements.iter() {
        println!("{}", ast);
    }

    Ok(())
}
