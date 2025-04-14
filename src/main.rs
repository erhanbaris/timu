#![allow(warnings)]

mod ast;
mod parser;

#[cfg(test)]
mod tests;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = parser::parser(r#"type MyType { a: ?string.base }"#);
    println!("{:#?}", parsed);
    Ok(())
}
