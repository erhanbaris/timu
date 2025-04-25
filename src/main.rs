mod ast;
mod file;
// mod parser;
//mod span;
//mod sitter_parser;
#[rustfmt::skip]
mod parser;
mod nom_tools;
mod error;

#[cfg(test)]
mod tests;

use std::{error::Error, rc::Rc};

use error::handle_parser;
use file::SourceFile;
use nom::Finish;
use nom_tools::State;


fn main() -> Result<(), Box<dyn Error>> {
    let source_file = Rc::new(SourceFile::new("<memory>".into(), "interface Myinterface : erhan { \r\n\t\r\n\t}"));

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
