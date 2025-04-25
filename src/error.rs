use std::rc::Rc;

use ariadne::{Color, Label, Report, ReportKind, Source};
use nom_language::error::VerboseErrorKind;

use crate::{ast::FileAst, file::SourceFile, nom_tools::{State, ToRange}};

fn print_error(ctx: &'static str, span_range: std::ops::Range<usize>, source_file: Rc<SourceFile<'_>>) {
    println!("{}", source_file.code());
    let file_name = format!("{:?}", source_file.path());
    Report::build(ReportKind::Error, (file_name.as_str(), 12..12))
        .with_code(1)
        .with_message("Syntax error")
        .with_label(Label::new((file_name.as_str(), span_range)).with_message(ctx).with_color(Color::Red))
        .finish()
        .print((file_name.as_str(), Source::from(source_file.code())))
        .unwrap();
}

pub fn handle_parser<'a>(result: Result<(nom_locate::LocatedSpan<&'a str, State<'a>>, FileAst<'a>), nom_language::error::VerboseError<nom_locate::LocatedSpan<&'a str, State<'a>>>>) -> Result<FileAst<'a>, nom_language::error::VerboseError<nom_locate::LocatedSpan<&'a str, State<'a>>>> {
    match result {
        Ok((_, parsed)) => Ok(parsed),
        Err(error) => {
            error.errors.iter().for_each(|(input, error_kind)| {
                if let VerboseErrorKind::Context(context) = error_kind {
                    println!("Context: {:?}", context);
                    print_error(context, input.to_range(), input.extra.file.clone());
                }
            });
            Err(error)
        }
    }
}
