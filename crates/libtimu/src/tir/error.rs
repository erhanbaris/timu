use std::ops::Range;

use strum::EnumProperty;
use strum_macros::{EnumDiscriminants, EnumProperty};

use crate::file::SourceFile;

use super::resolver::ResolverError;


#[derive(Debug, thiserror::Error)]
#[error("Import not found")]
pub struct ImportNotFound { pub module: String, #[allow(dead_code)] pub position: Range<usize>, #[allow(dead_code)] pub source: SourceFile }

#[derive(Debug, thiserror::Error)]
#[error("Module already defined")]
pub struct AstModuleAlreadyDefined { pub position: Range<usize>, pub source: SourceFile }

#[derive(Debug, thiserror::Error)]
#[error("Type not found")]
pub struct TypeNotFound { #[allow(dead_code)] pub source: SourceFile, #[allow(dead_code)] pub position: Range<usize> }

#[derive(Debug, thiserror::Error)]
#[error("Already defined")]
pub struct AlreadyDefined { #[allow(dead_code)] pub position: Range<usize>, #[allow(dead_code)] pub source: SourceFile }

#[derive(Debug, thiserror::Error)]
#[error("Extra accessibility identifier")]
pub struct ExtraAccessibilityIdentifier { #[allow(dead_code)] pub position: Range<usize>, #[allow(dead_code)] pub source: SourceFile }

#[derive(Debug, thiserror::Error)]
#[error("Invalid type")]
pub struct InvalidType { #[allow(dead_code)] pub position: Range<usize>, #[allow(dead_code)] pub source: SourceFile }

#[derive(Debug, thiserror::Error)]
#[error("Interface field not defined")]
pub struct InterfaceFieldNotDefined { #[allow(dead_code)] pub position: Range<usize>, #[allow(dead_code)] pub source: SourceFile }

#[derive(Debug, thiserror::Error)]
#[error("Types do not match")]
pub struct TypesDoNotMatch { #[allow(dead_code)] pub position: Range<usize>, #[allow(dead_code)] pub source: SourceFile }

#[derive(Debug, thiserror::Error)]
#[error("Extra field in interface")]
pub struct ExtraFieldInInterface { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: SourceFile }

#[derive(Debug, thiserror::Error)]
#[error("Module already defined")]
pub struct ModuleAlreadyDefined { pub source: SourceFile }

#[derive(Debug, thiserror::Error, EnumDiscriminants, EnumProperty)]
pub enum TirError {
    #[strum(props(code=1))]
    #[error(transparent)]
    ImportNotFound(Box<ImportNotFound>),
    
    #[strum(props(code=2))]
    #[error(transparent)]
    ModuleAlreadyDefined(Box<ModuleAlreadyDefined>),
    
    #[strum(props(code=3))]
    #[error(transparent)]
    AstModuleAlreadyDefined(Box<AstModuleAlreadyDefined>),
    
    #[strum(props(code=4))]
    #[error(transparent)]
    TypeNotFound(Box<TypeNotFound>),
    
    #[strum(props(code=5))]
    #[error(transparent)]
    AlreadyDefined(Box<AlreadyDefined>),
    
    #[strum(props(code=6))]
    #[error(transparent)]
    ExtraAccessibilityIdentifier(Box<ExtraAccessibilityIdentifier>),
    
    #[strum(props(code=7))]
    #[error(transparent)]
    InvalidType(Box<InvalidType>),
    
    #[strum(props(code=8))]
    #[error(transparent)]
    InterfaceFieldNotDefined(Box<InterfaceFieldNotDefined>) ,
    
    #[strum(props(code=9))]
    #[error(transparent)]
    TypesDoNotMatch(Box<TypesDoNotMatch>),
    
    #[strum(props(code=10))]
    #[error(transparent)]
    ExtraFieldInInterface(Box<ExtraFieldInInterface>),

    #[strum(props(code=11))]
    #[error(transparent)]
    ResolverError(#[from] Box<ResolverError>),
}

#[derive(Debug)]
pub struct ErrorReport {
    #[allow(dead_code)]
    pub position: Range<usize>,
    #[allow(dead_code)]
    pub message: String,
    #[allow(dead_code)]
    pub file: SourceFile,
    #[allow(dead_code)]
    pub error_code: String,
}

pub trait CustomError {
    fn get_errors(&self, parent_error_code: &str) -> Vec<ErrorReport>;
    fn get_error_code(&self) -> i64;
    fn build_error_code(&self, parent_error_code: &str) -> String {
        format!("{}-{}", parent_error_code, self.get_error_code())
    }
}

impl CustomError for TirError {
    fn get_errors(&self, parent_error_code: &str) -> Vec<ErrorReport> {
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

impl TirError {
    pub fn already_defined(position: Range<usize>, source: SourceFile) -> Self {
        TirError::AlreadyDefined(AlreadyDefined {
            position,
            source,
        }.into())
    }

    pub fn interface_field_not_defined(position: Range<usize>, source: SourceFile) -> Self {
        TirError::InterfaceFieldNotDefined(InterfaceFieldNotDefined {
            position,
            source,
        }.into())
    }

    pub fn types_do_not_match(position: Range<usize>, source: SourceFile) -> Self {
        TirError::TypesDoNotMatch(TypesDoNotMatch {
            position,
            source,
        }.into())
    }

    pub fn extra_accessibility_identifier(position: Range<usize>, source: SourceFile) -> Self {
        TirError::ExtraAccessibilityIdentifier(ExtraAccessibilityIdentifier {
            position,
            source,
        }.into())
    }

    pub fn extra_field_in_interface(position: Range<usize>, source: SourceFile) -> Self {
        TirError::ExtraFieldInInterface(ExtraFieldInInterface {
            position,
            source,
        }.into())
    }

    pub fn type_not_found(position: Range<usize>, source: SourceFile) -> Self {
        TirError::TypeNotFound(TypeNotFound {
            position,
            source,
        }.into())
    }

    pub fn invalid_type(position: Range<usize>, source: SourceFile) -> Self {
        TirError::InvalidType(InvalidType {
            position,
            source,
        }.into())
    }

    #[allow(dead_code)]
    pub fn get_error(&self) -> (Range<usize>, String, SourceFile) {
        match self {
            TirError::ImportNotFound(error) => (error.position.clone(), format!("{}", error), error.source.clone()),
            TirError::ResolverError(_) => unimplemented!("Please use get_errors() for ScopeError"),
            TirError::ModuleAlreadyDefined(error) => (0..0, format!("{}", error), error.source.clone()),
            TirError::AstModuleAlreadyDefined(error) => (error.position.clone(), format!("{}", error), error.source.clone()),
            TirError::AlreadyDefined(error) => (error.position.clone(), format!("{}", error), error.source.clone()),
            TirError::TypeNotFound(error) => (error.position.clone(), format!("{}", error), error.source.clone()),
            TirError::ExtraAccessibilityIdentifier(error) => (error.position.clone(), format!("{}", error), error.source.clone()),
            TirError::InvalidType(error) => (error.position.clone(), format!("{}", error), error.source.clone()),
            TirError::InterfaceFieldNotDefined(error) => (error.position.clone(), format!("{}", error), error.source.clone()),
            TirError::TypesDoNotMatch(error) => (error.position.clone(), format!("{}", error), error.source.clone()),
            TirError::ExtraFieldInInterface(error) => (error.position.clone(), format!("{}", error), error.source.clone()),
        }
    }
}
