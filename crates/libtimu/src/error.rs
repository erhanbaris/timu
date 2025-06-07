use codespan_reporting::{diagnostic::{Diagnostic, Label}, files::SimpleFiles, term::{self, termcolor::StandardStream}};
use libtimu_macros_core::traits::TimuErrorTrait;
use nom_language::error::VerboseErrorKind;
use simplelog::ColorChoice;
use crate::{
    ast::FileAst,
    nom_tools::{State, ToRange},
    tir::{new_error::SyntaxErrorItem, TirContext, TirError},
};

pub type ParseError<'base> = nom_language::error::VerboseError<nom_locate::LocatedSpan<&'base str, State>>;
pub type ParseResult<'base> = Result<(nom_locate::LocatedSpan<&'base str, State>, FileAst<'base>), ParseError<'base>>;

pub type TirResult<'base> = Result<TirContext<'base>, TirError>;



pub trait ReportGenerator {
    fn generate(error: TirError);
}

pub struct CodeSpanReportGenerator;

impl ReportGenerator for CodeSpanReportGenerator {
    fn generate(error: TirError) {

        let mut diagnostics = Vec::new();
        let mut files = SimpleFiles::new();
        let errors = error.errors();

        let mut diagnostic: Diagnostic<usize> = Diagnostic::error().with_message(error.to_string());

        if let Some(source_code) = error.source_code()  {
            let file_id = files.add(source_code.name, source_code.source);
            
             if let Some(labels) = error.labels() {
                let labels = labels.into_iter().map(|label| Label::primary(file_id, label.position).with_message(label.label)).collect::<Vec<_>>();
                diagnostic = diagnostic.with_labels(labels);
            }
        }

        diagnostics.push(diagnostic);

        if let Some(errors) = errors {
            for inner_error in errors {
                let mut diagnostic: Diagnostic<usize> = Diagnostic::error().with_message(inner_error.to_string());
                let source_code = inner_error.source_code();

                if let Some(source_code) = source_code  {
                    let file_id = files.add(source_code.name, source_code.source);
                    
                    if let Some(inner_labels) = inner_error.labels() {
                        let labels = inner_labels.into_iter().map(|label| Label::primary(file_id, label.position).with_message(label.label)).collect::<Vec<_>>();
                        diagnostic = diagnostic.with_labels(labels);
                    }
                }
                
                diagnostics.push(diagnostic);
            }
        }

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        for diagnostic in diagnostics.into_iter() {
            term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
        }
    }
}

#[allow(clippy::result_unit_err)]
pub fn handle_parser(result: ParseResult<'_>) -> Result<FileAst<'_>, TirError> {
    match result {
        Ok((_, parsed)) => Ok(parsed),
    Err(error) => {
        let mut errors =  Vec::new();
            error.errors.iter().for_each(|(span, error_kind)| {
                if let VerboseErrorKind::Context(error_message) = error_kind {
                    errors.push(SyntaxErrorItem {
                        position: span.to_range(),
                        code: span.extra.file.clone().into(),
                        message: error_message
                    });
                }
            });
            Err(TirError::syntax_error(errors))
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
