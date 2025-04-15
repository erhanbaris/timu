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
use nom_locate::LocatedSpan;

fn print_error<'a>(file_name: &str, code: &str, message: &str, span: LocatedSpan<&'a str>) {
    Report::build(ReportKind::Error, (file_name, 12..12))
        .with_code(1)
        .with_message(message)
        .with_label(
            Label::new((file_name, span.location_offset()..span.location_offset() + span.fragment().len()))
                .with_message("Invalid syntax")
                .with_color(Color::Red),
        )
        .finish()
        .print((file_name, Source::from(code)))
        .unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let code = "class erhanbaris1 { pub a : ? string.base,ba:string.base }  class erhanbaris2 { pub a : ? string.base,ba:string.base }";
    let response = nom_parser::parse(code.into());

    match response {
        Ok((remaining, parsed)) => {
            println!("Parsed successfully: {:#?}", parsed);
            println!("Remaining: '{}'", remaining);
        }
        Err(error) => {
            println!("Parsing failed: {:?}", error);
            error.map_input(|input| {
                print_error("test.timu", code, "Parsing failed", input);
            });
        }
    }

    /*let source_file = SourceFile::new("test.timu".into(), r#"type MyType { a: ?string.base  ad }"#);
    let parsed = parser::parse(source_file.into());

    match parsed {
        Ok(file) => {
            println!("Parsed successfully: {:#?}", file);
        }
        Err(error) => {
            let file_path = format!("{:?}", error.span.src().path());

            Report::build(ReportKind::Error, (file_path.as_str(), 12..12))
                .with_code(1)
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
    }*/
    Ok(())
}
