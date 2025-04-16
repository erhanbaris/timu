#![allow(warnings)]

mod ast;
mod file;
mod parser;
mod span;
//mod sitter_parser;
mod nom_parser;

#[cfg(test)]
mod tests;

use std::{cell::RefCell, error::Error, rc::Rc};

use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};
use file::SourceFile;
use nom_language::error::VerboseError;
use nom_locate::LocatedSpan;
use nom_parser::{Span, State};

fn print_error<'a>(message: &str, span: VerboseError<Span<'a>>) {
    span.errors.iter().for_each(|error| {
        println!("Error: {:?}", error);
    });

    let file_name = format!("{:?}", "test.timu");

    Report::build(ReportKind::Error, (file_name.as_str(), 12..12))
        .with_code(1)
        .with_message(message)
        .with_label(Label::new((file_name.as_str(), 1..2)).with_message("Invalid syntax").with_color(Color::Red))
        .finish()
        .print((file_name.as_str(), Source::from(message)))
        .unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let source_file = Rc::new(SourceFile::new("test.timu".into(), r#"class MyType a { a: ?string.base  ad }"#));
    let errors = RefCell::new(vec![]);

    let mut state = State {
        errors: &errors,
        file: source_file.clone(),
    };

    let response = nom_parser::parse::<VerboseError<Span>>(state);

    match response {
        Ok((remaining, parsed)) => {
            println!("Parsed successfully: {:#?}", parsed);
            println!("Remaining: '{}'", remaining);
        }
        Err(error) => {
            println!("Parsing failed: {:?}", error);
            error.map(|input| {
                print_error("Parsing failed", input);
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
