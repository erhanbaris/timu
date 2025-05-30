use std::{borrow::Cow, error::Error, fmt::Display, ops::Range, rc::Rc};

use strum::EnumProperty;
use strum_macros::{EnumDiscriminants, EnumProperty};

use crate::file::SourceFile;

use super::resolver::ResolverError;

#[derive(Debug, EnumDiscriminants, EnumProperty)]
pub enum TirError<'base> {
    #[strum(props(code=1))]
    ImportNotFound { module: Cow<'base, str>, #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    
    #[strum(props(code=2))]
    ModuleAlreadyDefined { source: Rc<SourceFile<'base>> },
    
    #[strum(props(code=3))]
    AstModuleAlreadyDefined { position: Range<usize>, source: Rc<SourceFile<'base>> },
    
    #[strum(props(code=4))]
    TypeNotFound { #[allow(dead_code)] source: Rc<SourceFile<'base>>, #[allow(dead_code)] position: Range<usize> },
    
    #[strum(props(code=5))]
    AlreadyDefined { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    
    #[strum(props(code=6))]
    ExtraAccessibilityIdentifier { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    
    #[strum(props(code=7))]
    InvalidType { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    
    #[strum(props(code=8))]
    InterfaceFieldNotDefined { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    
    #[strum(props(code=9))]
    TypesDoNotMatch { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    
    #[strum(props(code=10))]
    ExtraFieldInInterface { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },

    #[strum(props(code=11))]
    ResolverError(Box<ResolverError<'base>>),
}

pub struct ErrorReport<'base> {
    #[allow(dead_code)]
    pub position: Range<usize>,
    #[allow(dead_code)]
    pub message: String,
    #[allow(dead_code)]
    pub file: Rc<SourceFile<'base>>,
    #[allow(dead_code)]
    pub error_code: String,
}

pub trait CustomError {
    fn get_errors(&self, parent_error_code: &str) -> Vec<ErrorReport<'_>>;
    fn get_error_code(&self) -> i64;
    fn build_error_code(&self, parent_error_code: &str) -> String {
        format!("{}-{}", parent_error_code, self.get_error_code())
    }
}

impl CustomError for TirError<'_> {
    fn get_errors(&self, parent_error_code: &str) -> Vec<ErrorReport<'_>> {
        match self {
            TirError::ResolverError(error) => error.get_errors(&self.build_error_code(parent_error_code)),
            _ => {
                let error = self.get_error();
                vec![ErrorReport {
                    position: error.0,
                    message: error.1,
                    file: error.2,
                    error_code: self.build_error_code(parent_error_code),
                }]
            },
        }
    }
    
    fn get_error_code(&self) -> i64 {
        self.get_int("code").unwrap()
    }
}

impl Display for TirError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TirError::ImportNotFound {
                module,
                position: _,
                source: _,
            } => write!(f, "Import not found: {}", module),
            TirError::ResolverError(error) => write!(f, "{}", error),
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
            TirError::InterfaceFieldNotDefined {
                source: _,
                position: _,
            } => write!(f, "Interface field not defined"),
            TirError::TypesDoNotMatch {
                source: _,
                position: _,
            } => write!(f, "Types do not match"),
            TirError::ExtraFieldInInterface {
                source: _,
                position: _,
            } => write!(f, "Extra field in interface"),
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

    pub fn interface_field_not_defined(position: Range<usize>, source: Rc<SourceFile<'base>>) -> Self {
        TirError::InterfaceFieldNotDefined {
            position,
            source,
        }
    }

    pub fn types_do_not_match(position: Range<usize>, source: Rc<SourceFile<'base>>) -> Self {
        TirError::TypesDoNotMatch {
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

    pub fn extra_field_in_interface(position: Range<usize>, source: Rc<SourceFile<'base>>) -> Self {
        TirError::ExtraFieldInInterface {
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
            TirError::ImportNotFound {
                module: _,
                position,
                source,
            } => (position.clone(), format!("{}", self), source.clone()),
            TirError::ResolverError(_) => unimplemented!("Please use get_errors() for ScopeError"),
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
            TirError::InterfaceFieldNotDefined {
                source,
                position,
            } => (position.clone(), format!("{}", self), source.clone()),
            TirError::TypesDoNotMatch {
                source,
                position,
            } => (position.clone(), format!("{}", self), source.clone()),
            TirError::ExtraFieldInInterface {
                source,
                position,
            } => (position.clone(), format!("{}", self), source.clone()),
        }
    }
}
