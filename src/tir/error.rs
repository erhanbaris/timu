use std::{borrow::Cow, error::Error, fmt::Display, ops::Range, rc::Rc};

use crate::file::SourceFile;

use super::{ast_signature::AstSignatureValue, module::ModuleRef, signature::Signature};

#[derive(Debug)]
pub enum TirError<'base> {
    AstSignatureNotFound { signature: Rc<Signature<'base, AstSignatureValue<'base>, ModuleRef<'base>>>, source: Rc<SourceFile<'base>> },
    ImportNotFound { module: Cow<'base, str>, position: Range<usize>, source: Rc<SourceFile<'base>> },
    ModuleAlreadyDefined { source: Rc<SourceFile<'base>> },
    AstModuleAlreadyDefined { position: Range<usize>, source: Rc<SourceFile<'base>> },
    TypeNotFound { source: Rc<SourceFile<'base>>, position: Range<usize> },
    AlreadyDefined { position: Range<usize>, source: Rc<SourceFile<'base>> },
}

impl Display for TirError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TirError::AstSignatureNotFound {
                signature,
                source: _,
            } => write!(f, "AST signature not found, signature: {:?}", signature.value),
            TirError::ImportNotFound {
                module,
                position: _,
                source: _,
            } => write!(f, "Import not found: {}", module),
            TirError::ModuleAlreadyDefined {
                source: _,
            } => write!(f, "Module already defined"),
            TirError::AstModuleAlreadyDefined {
                position: _,
                source: _,
            } => write!(f, "Ast Module already defined"),
            TirError::AlreadyDefined {
                position: _,
                source: _,
            } => write!(f, "Already defined"),
            TirError::TypeNotFound {
                source: _,
                position: _,
            } => write!(f, "Type not found"),
        }
    }
}

impl Error for TirError<'_> {}

impl<'base> TirError<'base> {
    pub fn already_defined(position: Range<usize>, source: Rc<SourceFile<'base>>) -> Self {
        TirError::AlreadyDefined {
            position,
            source,
        }
    }

    pub fn get_error(&self) -> (Range<usize>, String, Rc<SourceFile<'_>>) {
        match self {
            TirError::AstSignatureNotFound {
                signature: _,
                source,
            } => (0..0, format!("{}", self), source.clone()),
            TirError::ImportNotFound {
                module: _,
                position,
                source,
            } => (position.clone(), format!("{}", self), source.clone()),
            TirError::ModuleAlreadyDefined {
                source,
            } => (0..0, format!("{}", self), source.clone()),
            TirError::AstModuleAlreadyDefined {
                position,
                source,
            } => (position.clone(), format!("{}", self), source.clone()),
            TirError::AlreadyDefined {
                position,
                source,
            } => (position.clone(), format!("{}", self), source.clone()),
            TirError::TypeNotFound {
                source,
                position,
            } => (position.clone(), format!("{}", self), source.clone()),
        }
    }
}
