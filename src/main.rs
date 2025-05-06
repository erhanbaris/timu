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

use ast::FileAst;
use error::{handle_builder, handle_parser, ParseError};
use file::SourceFile;
use nom::Finish;
use nom_tools::State;
use tir::TirError;

fn process_code(path: Vec<String>, code: &'_ str) -> Result<FileAst<'_>, ParseError<'_>> {
    let file = Rc::new(SourceFile::new(path, code));
    let state = State {
        file,
    };

    let response = parser::parse(state).finish();
    handle_parser(response)
}

fn process_ast(files: Vec<Rc<FileAst>>) -> Result<(), TirError<'_>> {
    let response = crate::tir::build(files);
    handle_builder(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    let ast_1 = process_code(vec!["source1".to_string()], " class testclass1 {} ")?;
    let ast_2 = process_code(vec!["source2".to_string()], "use source1; use source1.testclass1;")?;

    let ast_3 = process_code(vec!["sub".to_string(), "source3".to_string()], "class testclass2 {}")?;
    let ast_4 = process_code(vec!["sub".to_string(), "source4".to_string()], "use source1; use source1.testclass1;")?;
    let ast_5 = process_code(
        vec!["sub".to_string(), "source5".to_string()],
        "use source1; use source1.testclass1;",
    )?;
    let ast_6 = process_code( 
        vec!["sub".to_string(), "source6".to_string()],
        "use sub.source3; use sub.source3.testclass2;",
    )?;
    let ast_7 = process_code(
        vec!["sub".to_string(), "source7".to_string()],
        "use source1; use source1.testclass1; use sub.source3; use sub.source3.testclass2;",
    )?;
    let ast_8 = process_code(vec!["sub".to_string(), "source8".to_string()], "class testclass1 {}")?;
    let ast_9 = process_code(
        vec!["sub".to_string(), "source9".to_string()],
        "use source1; use source1.testclass1; use sub.source3; use sub.source3.testclass2; use sub.source8; use sub.source8.testclass1;",
    )?;

    process_ast(vec![ast_1.into(), ast_2.into(), ast_3.into(), ast_4.into(), ast_5.into(), ast_6.into(), ast_7.into(), ast_8.into(), ast_9.into()])?;

    Ok(())
}
