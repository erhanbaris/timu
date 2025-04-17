#![allow(warnings)]

// mod ast;
mod file;
// mod parser;
//mod span;
//mod sitter_parser;
mod nom_parser;
mod nom_tools;

#[cfg(test)]
mod tests;

use std::{cell::RefCell, error::Error, rc::Rc};

use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};
use file::SourceFile;
use nom_language::error::VerboseError;
use nom_locate::LocatedSpan;
use nom_tools::{CustomErrorContext, Span, State, ToRange};

fn print_error<'a>(source_file: Rc<SourceFile<'a>>, span: VerboseError<Span<'a>>, errors: Rc<RefCell<Vec<nom_tools::Error>>>) {
    let file_name = format!("{:?}", source_file.path());
    let mut report = Report::build(ReportKind::Error, (file_name.as_str(), 12..12)).with_code(1).with_message("Syntax error");

    for error in errors.borrow().iter() {
        report = report.with_label(Label::new((file_name.as_str(), error.0.clone())).with_message(error.1.clone()).with_color(Color::Red));
    }

    report.finish().print((file_name.as_str(), Source::from(source_file.code()))).unwrap()
}

impl<'a> CustomErrorContext<'a> for VerboseError<Span<'a>> {
    fn add_error(input: Span<'a>, ctx: &'static str, other: Self) -> Self {
        input.extra.report_error(nom_tools::Error(input.to_range(), ctx.to_string()));
        println!("Error: {:?}", other.errors);
        other
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let source_file = Rc::new(SourceFile::new("<memory>".into(), r#"func init(a: int): MyType {}"#));
    let errors = Rc::new(RefCell::new(vec![]));

    let mut state = State {
        errors: errors.clone(),
        file: source_file.clone(),
    };

    let response = nom_parser::parse(state);

    match response {
        Ok((remaining, parsed)) => {
            for ast in parsed.statements.iter() {
                println!("{}", ast);
            }
            println!("Remaining: '{}'", remaining);
        }
        Err(error) => {
            error.map(|input| {
                print_error(source_file, input, errors);
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
