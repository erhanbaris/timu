use std::{borrow::Cow, error::Error, fmt::Display, ops::Range, rc::Rc};

use crate::file::SourceFile;

use super::{context::AstSignatureHolderType, signature::SignatureError, AstSignature};

#[derive(Debug)]
pub enum TirError<'base> {
    ModuleNotFound { module: Cow<'base, str>, position: Range<usize>, source: Rc<SourceFile<'base>> },
    AstModuleAlreadyDefined { old_signature: Rc<AstSignature<'base>>, source: Rc<SourceFile<'base>> },
    TypeNotFound { source: Rc<SourceFile<'base>> },
    AstError {
        error: SignatureError<'base, AstSignatureHolderType<'base>>,
        source: Rc<SourceFile<'base>>
    },
}

impl Display for TirError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TirError::ModuleNotFound { module, position: _ , source: _ } => write!(f, "Module not found: {}", module),
            TirError::AstModuleAlreadyDefined { old_signature , source: _ } => write!(f, "Module already defined: {:?}", old_signature),
            TirError::TypeNotFound { source: _ } => write!(f, "Type not found"),
            TirError::AstError { error , source: _ } => write!(f, "AST error: {:?}", error),
        }
    }
}

impl Error for TirError<'_> { }

impl TirError<'_> {
    pub fn get_error(&self) -> (Range<usize>, String, Rc<SourceFile<'_>>) {
        match self {
            TirError::ModuleNotFound { module, position, source } => (position.clone(), format!("Module not found: {}", module), source.clone()),
            TirError::AstModuleAlreadyDefined { old_signature, source } => (0..0, format!("Module already defined: {:?}", old_signature), source.clone()),
            TirError::TypeNotFound { source } => (0..0, "Type not found".to_string(), source.clone()),
            TirError::AstError { error, source } => (0..0, format!("AST error: {:?}", error), source.clone()),
            
        }
    }
}