use nom_language::error::VerboseErrorKind;
use crate::{
    ast::FileAst,
    nom_tools::State,
    tir::{TirContext, TirError},
};

pub type ParseError<'base> = nom_language::error::VerboseError<nom_locate::LocatedSpan<&'base str, State>>;
pub type ParseResult<'base> = Result<(nom_locate::LocatedSpan<&'base str, State>, FileAst<'base>), ParseError<'base>>;

pub type TirResult<'base> = Result<TirContext<'base>, TirError>;

#[allow(clippy::result_unit_err)]
pub fn handle_builder(result: TirResult<'_>) -> Result<TirContext<'_>, ()> {
    match result {
        Ok(context) => Ok(context),
        Err(error) => {

            let error = miette::Report::msg(error);
            println!("Error: {}", error);
            Err(())
        }
    }
}

#[allow(clippy::result_unit_err)]
pub fn handle_parser(result: ParseResult<'_>) -> Result<FileAst<'_>, ()> {
    match result {
        Ok((_, parsed)) => Ok(parsed),
        Err(error) => {
            error.errors.iter().for_each(|(_, error_kind)| {
                if let VerboseErrorKind::Context(error_message) = error_kind {
                    println!("Syntax Error: {}", error_message);
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
