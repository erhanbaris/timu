#![allow(warnings)]

mod ast;
mod file;
// mod parser;
//mod span;
//mod sitter_parser;
#[rustfmt::skip]
mod nom_parser;
mod nom_tools;

#[cfg(test)]
mod tests;

use std::{cell::RefCell, error::Error, rc::Rc};

use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};
use file::SourceFile;
use nom::Finish;
use nom_language::error::{VerboseError, VerboseErrorKind};
use nom_locate::LocatedSpan;
use nom_tools::{Span, State, ToRange};

fn print_error<'a>(ctx: &'static str, span_range: std::ops::Range<usize>, source_file: Rc<SourceFile<'a>>) {
    println!("{}", source_file.code());
    let file_name = format!("{:?}", source_file.path());
    let mut report = Report::build(ReportKind::Error, (file_name.as_str(), 12..12))
        .with_code(1)
        .with_message("Syntax error")
        .with_label(Label::new((file_name.as_str(), span_range)).with_message(ctx).with_color(Color::Red))
        .finish()
        .print((file_name.as_str(), Source::from(source_file.code())))
        .unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let source_file = Rc::new(SourceFile::new(
        "<memory>".into(),
        r#"
class test { 
    func init(a: int): MyType {
        var test = 1.1;
    }
}
    "#,
    ));

    let mut state = State {
        file: source_file.clone(),
    };

    let response = nom_parser::parse(state).finish();

    match response {
        Ok((remaining, parsed)) => {
            for ast in parsed.statements.iter() {
                println!("{}", ast);
            }
        }
        Err(error) => {
            error.errors.iter().for_each(|e| {
                if let VerboseErrorKind::Context(context) = e.1 {
                    println!("Context: {:?}", context);
                    print_error(context, e.0.to_range(), source_file.clone());
                }
            });
        }
    }
    Ok(())
}
