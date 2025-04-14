#![allow(warnings)]

mod ast;
mod file;
mod parser;
mod span;

#[cfg(test)]
mod tests;

use std::error::Error;

use file::SourceFile;

fn main() -> Result<(), Box<dyn Error>> {
    let source_file = SourceFile::new("test.timu".into(), r#"type MyType { a: ?string.base }"#);
    let parsed = parser::parser(source_file.into());
    println!("{:#?}", parsed);
    Ok(())
}
