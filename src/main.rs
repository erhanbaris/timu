#![allow(warnings)]

mod ast;
mod file;
mod parser;
mod span;
//mod sitter_parser;
mod nom_parser;

#[cfg(test)]
mod tests;

use std::error::Error;

use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};
use file::SourceFile;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{:?}", nom_parser::parse("type erhanbaris { pub a : ? string.base,ba:?string.base}".into()));

    let source_file = SourceFile::new("test.timu".into(), r#"type MyType { a: ?string.base  ad }"#);
    let parsed = parser::parse(source_file.into());

    match parsed {
        Ok(file) => {
            println!("Parsed successfully: {:#?}", file);
        }
        Err(error) => {
            let file_path = format!("{:?}", error.span.src().path());

            Report::build(ReportKind::Error, (file_path.as_str(), 12..12))
                .with_code(3)
                .with_message(error.message)
                .with_label(
                    Label::new((file_path.as_str(), error.span.start()..error.span.end()))
                        .with_message(format!("This is of type {}", "Nat".fg(Color::Red)))
                        .with_color(Color::Red),
                )
                .finish()
                .print((file_path.as_str(), Source::from(error.span.src().code())))
                .unwrap();
        }
    }
    Ok(())
}
