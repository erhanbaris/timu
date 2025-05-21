use std::{borrow::Cow, error::Error, fmt::Display, ops::Range, rc::Rc};

use crate::file::SourceFile;

#[derive(Debug)]
pub enum TirError<'base> {
    AstSignatureNotFound { #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    ImportNotFound { module: Cow<'base, str>, #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    ModuleAlreadyDefined { source: Rc<SourceFile<'base>> },
    AstModuleAlreadyDefined { position: Range<usize>, source: Rc<SourceFile<'base>> },
    TypeNotFound { #[allow(dead_code)] source: Rc<SourceFile<'base>>, #[allow(dead_code)] position: Range<usize> },
    AlreadyDefined { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    ExtraAccessibilityIdentifier { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    InvalidType { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
}

impl Display for TirError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TirError::AstSignatureNotFound {
                source: _,
            } => write!(f, "AST signature not found, signature"),
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
            TirError::ExtraAccessibilityIdentifier {
                source: _,
                position: _,
            } => write!(f, "Extra accessibility identifier"),
            TirError::InvalidType {
                source: _,
                position: _,
            } => write!(f, "Invalid type"),
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

    pub fn extra_accessibility_identifier(position: Range<usize>, source: Rc<SourceFile<'base>>) -> Self {
        TirError::ExtraAccessibilityIdentifier {
            position,
            source,
        }
    }

    pub fn type_not_found(position: Range<usize>, source: Rc<SourceFile<'base>>) -> Self {
        TirError::TypeNotFound {
            position,
            source,
        }
    }

    pub fn invalid_type(position: Range<usize>, source: Rc<SourceFile<'base>>) -> Self {
        TirError::InvalidType {
            position,
            source,
        }
    }

    #[allow(dead_code)]
    pub fn get_error(&self) -> (Range<usize>, String, Rc<SourceFile<'_>>) {
        match self {
            TirError::AstSignatureNotFound {
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
            TirError::ExtraAccessibilityIdentifier {
                source,
                position,
            } => (position.clone(), format!("{}", self), source.clone()),
            TirError::InvalidType {
                source,
                position,
            } => (position.clone(), format!("{}", self), source.clone()),
        }
    }
}
