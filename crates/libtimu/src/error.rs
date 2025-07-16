//! Error handling and reporting for the Timu language compiler.
//!
//! This module provides comprehensive error handling infrastructure including:
//! - Type aliases for parse and TIR results
//! - Error reporting using `codespan-reporting` for rich diagnostics
//! - Parser error conversion utilities
//! - Integration with the Timu error trait system

use codespan_reporting::{diagnostic::{Diagnostic, Label}, files::SimpleFiles, term::{self, termcolor::StandardStream}};
use libtimu_macros_core::traits::TimuErrorTrait;
use nom_language::error::VerboseErrorKind;
use simplelog::ColorChoice;
use crate::{
    ast::FileAst,
    nom_tools::{State, ToRange},
    tir::{error::SyntaxErrorItem, TirContext, TirError},
};

/// File extension for Timu language source files
pub static TIMU_LANG_EXT: &str = "tim";

/// Error type for parsing operations using nom combinators
pub type ParseError<'base> = nom_language::error::VerboseError<nom_locate::LocatedSpan<&'base str, State>>;

/// Result type for parsing operations that returns the remaining input and parsed AST
pub type ParseResult<'base> = Result<(nom_locate::LocatedSpan<&'base str, State>, FileAst<'base>), ParseError<'base>>;

/// Result type for TIR (Type Intermediate Representation) operations
pub type TirResult<'base> = Result<TirContext<'base>, TirError>;



/// Trait for generating error reports from TIR errors
pub trait ReportGenerator {
    /// Generate a formatted error report for the given TIR error
    fn generate(error: TirError);
}

/// Error report generator using the `codespan-reporting` library for rich diagnostics
pub struct CodeSpanReportGenerator;

impl CodeSpanReportGenerator {
    /// Internal helper to recursively generate diagnostics for errors and their references
    fn inner_generate(files: &mut SimpleFiles<String, String>, diagnostics: &mut Vec<Diagnostic<usize>>, error: &dyn TimuErrorTrait) {
        let mut diagnostic: Diagnostic<usize> = Diagnostic::error().with_message(error.to_string());

        if let Some(source_code) = error.source_code()  {
            let file_id = files.add(format!("{}.{}", source_code.name, TIMU_LANG_EXT), source_code.source);
            
             if let Some(labels) = error.labels() {
                let labels = labels.into_iter().map(|label| Label::primary(file_id, label.position).with_message(label.label)).collect::<Vec<_>>();
                diagnostic = diagnostic.with_labels(labels);
            }
        }

        if let Some(help) = error.help() {
            diagnostic = diagnostic.with_note(help.to_string());
        }

        diagnostics.push(diagnostic);

        if let Some(references) = error.references() {
            for reference in references.into_iter() {
                Self::inner_generate(files, diagnostics, *reference);
            }   
        }

        if let Some(errors) = error.errors() {
            for inner_error in errors {
                Self::inner_generate(files, diagnostics, inner_error);
            }
        }
    }
}

impl ReportGenerator for CodeSpanReportGenerator {
    fn generate(error: TirError) {
        let mut diagnostics = Vec::new();
        let mut files = SimpleFiles::new();

        Self::inner_generate(&mut files, &mut diagnostics, &error);

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        for diagnostic in diagnostics.into_iter() {
            term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
        }
    }
}

/// Converts parser results into TIR-compatible results, handling parser errors
/// 
/// Takes a parser result and either returns the parsed AST or converts
/// parser errors into TIR syntax errors with proper source location information.
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
                        code: (&span.extra.file).into(),
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
