use ariadne::{Color, Label, Report, ReportKind, Source};
use nom_language::error::VerboseErrorKind;
use crate::tir::error::CustomError;
use crate::{
    ast::FileAst,
    file::SourceFile,
    nom_tools::{State, ToRange},
    tir::{TirContext, TirError},
};

pub type ParseError<'base> = nom_language::error::VerboseError<nom_locate::LocatedSpan<&'base str, State>>;
pub type ParseResult<'base> = Result<(nom_locate::LocatedSpan<&'base str, State>, FileAst<'base>), ParseError<'base>>;

pub type TirResult<'base> = Result<TirContext<'base>, TirError>;

pub fn print_error(error_type: &str, error_message: &str, span_range: std::ops::Range<usize>, source_file: SourceFile) {
    let file_name = source_file.path().join("/");
    Report::build(ReportKind::Error, (file_name.as_str(), 12..12))
        .with_code(1)
        .with_message(error_type)
        .with_label(Label::new((file_name.as_str(), span_range)).with_message(error_message).with_color(Color::Red))
        .finish()
        .print((file_name.as_str(), Source::from(source_file.code())))
        .unwrap();
}

#[allow(clippy::result_unit_err)]
pub fn handle_builder(result: TirResult<'_>) -> Result<TirContext<'_>, ()> {
    match result {
        Ok(context) => Ok(context),
        Err(error) => {
            let errors = error.get_errors("01");

            let error = errors.first().unwrap();
            print_error("Definition issue", &error.message, error.position.clone(), error.file.clone());
            Err(())
        }
    }
}

#[allow(clippy::result_unit_err)]
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
    use nom::Finish;

    use crate::{file::SourceFile, nom_tools::State, parser};

    use super::handle_parser;

    #[test]
    #[should_panic]
    fn error_test() {
        let source_file = SourceFile::new(vec!["<memory>".into()], "interface Myinterface : erhan {".to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let response = parser::parse(&state).finish();
        handle_parser(response).unwrap();
    }
}
