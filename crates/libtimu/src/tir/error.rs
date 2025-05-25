use std::{borrow::Cow, error::Error, fmt::Display, ops::Range, rc::Rc};

use crate::file::SourceFile;

use super::resolver::statement::FunctionCallError;

#[derive(Debug)]
pub enum TirError<'base> {
    ImportNotFound { module: Cow<'base, str>, #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    ModuleAlreadyDefined { source: Rc<SourceFile<'base>> },
    AstModuleAlreadyDefined { position: Range<usize>, source: Rc<SourceFile<'base>> },
    TypeNotFound { #[allow(dead_code)] source: Rc<SourceFile<'base>>, #[allow(dead_code)] position: Range<usize> },
    AlreadyDefined { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    ExtraAccessibilityIdentifier { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    InvalidType { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    InterfaceFieldNotDefined { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    TypesDoNotMatch { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    ExtraFieldInInterface { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    ThisNeedToDefineInClass { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    ThisArgumentMustBeFirst { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: Rc<SourceFile<'base>> },
    FunctionCall(Box<FunctionCallError<'base>>),
}

pub struct InnerError<'base> {
    pub position: Range<usize>,
    #[allow(dead_code)]
    pub message: String,
    pub file: Rc<SourceFile<'base>>
}

pub trait CustomError {
    fn get_error(&self) -> Vec<InnerError<'_>>;
}

impl Display for TirError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TirError::ImportNotFound {
                module,
                position: _,
                source: _,
            } => write!(f, "Import not found: {}", module),
            TirError::FunctionCall(error) => write!(f, "{}", error),
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
            TirError::ThisNeedToDefineInClass {
                source: _,
                position: _,
            } => write!(f, "This need to define in class"),
            TirError::ThisArgumentMustBeFirst {
                source: _,
                position: _,
            } => write!(f, "This need to define in class"),
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

    pub fn this_argument_must_be_first(position: Range<usize>, source: Rc<SourceFile<'base>>) -> Self {
        TirError::ThisArgumentMustBeFirst {
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
            TirError::FunctionCall(error) => (error.get_error()[0].position.clone(), format!("{}", error), error.get_error()[0].file.clone()),
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
            TirError::ThisNeedToDefineInClass {
                source,
                position,
            } => (position.clone(), format!("{}", self), source.clone()),
            TirError::ThisArgumentMustBeFirst {
                source,
                position,
            } => (position.clone(), format!("{}", self), source.clone()),
        }
    }
}
