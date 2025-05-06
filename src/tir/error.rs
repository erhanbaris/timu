use std::{error::Error, fmt::Display, rc::Rc};

use super::{context::AstSignatureHolderType, signature::SignatureError, AstSignature};

#[derive(Debug)]
pub enum TirError<'base> {
    ModuleNotFound { module: String },
    AstModuleAlreadyDefined { old_signature: Rc<AstSignature<'base>> },
    TypeNotFound { },
    AstError {
        source: SignatureError<AstSignatureHolderType<'base>>
    },
}

impl Display for TirError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TirError::ModuleNotFound { module } => write!(f, "Module not found: {}", module),
            TirError::AstModuleAlreadyDefined { old_signature } => write!(f, "Module already defined: {:?}", old_signature),
            TirError::TypeNotFound { } => write!(f, "Type not found"),
            TirError::AstError { source } => write!(f, "AST error: {:?}", source),
        }
    }
}

impl Error for TirError<'_> {
}