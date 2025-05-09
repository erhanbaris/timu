use std::rc::Rc;

use ariadne::{Color, Label, Report, ReportKind, Source};
use nom_language::error::VerboseErrorKind;

use crate::{
    ast::FileAst,
    file::SourceFile,
    nom_tools::{State, ToRange},
    tir::TirError,
};

pub type ParseError<'a> = nom_language::error::VerboseError<nom_locate::LocatedSpan<&'a str, State<'a>>>;
pub type ParseResult<'a> = Result<(nom_locate::LocatedSpan<&'a str, State<'a>>, FileAst<'a>), ParseError<'a>>;

pub type TirResult<'a> = Result<(), TirError<'a>>;

pub fn print_error(error_type: &str, error_message: &str, span_range: std::ops::Range<usize>, source_file: Rc<SourceFile<'_>>) {
    let file_name = source_file.path().join("/");
    Report::build(ReportKind::Error, (file_name.as_str(), 12..12))
        .with_code(1)
        .with_message(error_type)
        .with_label(Label::new((file_name.as_str(), span_range)).with_message(error_message).with_color(Color::Red))
        .finish()
        .print((file_name.as_str(), Source::from(source_file.code())))
        .unwrap();
}

pub fn handle_builder(result: TirResult<'_>) -> Result<(), ()> {
    match result {
        Ok(_) => Ok(()),
        Err(error) => {
            let (range, message, source) = error.get_error();
            print_error("Definition issue", &message, range, source);
            Err(())
        }
    }
}

pub fn handle_parser(result: ParseResult<'_>) -> Result<FileAst<'_>, ()> {
    match result {
        Ok((_, parsed)) => Ok(parsed),
        Err(error) => {
            error.errors.iter().for_each(|(input, error_kind)| {
                if let VerboseErrorKind::Context(error_message) = error_kind {
                    print_error("Syntax issue", error_message, input.to_range(), input.extra.file.clone());
                }
            });
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use nom::Finish;

    use crate::{file::SourceFile, nom_tools::State, parser};

    use super::handle_parser;

    #[test]
    #[should_panic]
    fn error_test() {
        let source_file = Rc::new(SourceFile::new(vec!["<memory>".to_string()], "interface Myinterface : erhan {"));

        let state = State {
            file: source_file.clone(),
        };

        let response = parser::parse(state).finish();
        handle_parser(response).unwrap();
    }
}
