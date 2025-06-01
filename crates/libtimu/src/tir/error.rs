use std::ops::Range;

use miette::{Diagnostic, NamedSource, SourceSpan};
use strum::EnumProperty;
use strum_macros::{EnumDiscriminants, EnumProperty};

use crate::file::SourceFile;

use super::{resolver::ResolverError, TirContext};

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("'{module}' not found")]
#[diagnostic(code(timu::error::import_not_found), help("try to remove the import or check the module name"))]
pub struct ImportNotFound {
    pub module: String,

    #[label("no external or internal module found with this name")]
    pub position: SourceSpan,
    
    #[source_code]
    pub code: NamedSource<String>
}

pub static TYPE_NOT_FOUND_HELP: &str = "try to import the type, or maybe you need to define it in the current file";

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("'{type_name}' type not found")]
#[diagnostic(code(timu::error::type_not_found))]
pub struct TypeNotFound {
    pub type_name: String,

    #[label("type is not imported or defined in the current file")]
    pub position: SourceSpan,
    
    #[source_code]
    pub code: NamedSource<String>,

    #[help]
    pub advice: String,
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(code(timu::error::module_already_imported), help("try to remove one of the import"))]
#[error("Module already defined")]
pub struct ModuleAlreadyImported {
    #[label("Already imported here")]
    pub old_position: SourceSpan,

    #[label("But it is imported again here")]
    pub new_position: SourceSpan,

    #[source_code]
    pub code: NamedSource<String>,
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("Already defined")]
#[diagnostic(code(timu::error::already_imported), help("change one of the names or remove the definition"))]
pub struct AlreadyDefined {
    #[label("Already defined here")]
    pub old_position: SourceSpan,

    #[label("But it is defined again here")]
    pub new_position: SourceSpan,
    
    #[source_code]
    pub code: NamedSource<String>,
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("Extra accessibility identifier")]
pub struct ExtraAccessibilityIdentifier { #[allow(dead_code)] pub position: Range<usize>, #[allow(dead_code)] pub source: SourceFile }

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("Invalid type")]
pub struct InvalidType { #[allow(dead_code)] pub position: Range<usize>, #[allow(dead_code)] pub source: SourceFile }

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("Interface field not defined")]
pub struct InterfaceFieldNotDefined { #[allow(dead_code)] pub position: Range<usize>, #[allow(dead_code)] pub source: SourceFile }

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("Types do not match")]
pub struct TypesDoNotMatch { #[allow(dead_code)] pub position: Range<usize>, #[allow(dead_code)] pub source: SourceFile }

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("Extra field in interface")]
pub struct ExtraFieldInInterface { #[allow(dead_code)] position: Range<usize>, #[allow(dead_code)] source: SourceFile }

#[derive(Debug, Diagnostic, thiserror::Error, EnumDiscriminants, EnumProperty)]
pub enum TirError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    ImportNotFound(Box<ImportNotFound>),
    
    #[error(transparent)]
    #[diagnostic(transparent)]
    ModuleAlreadyImported(Box<ModuleAlreadyImported>),
    
    #[error(transparent)]
    #[diagnostic(transparent)]
    TypeNotFound(Box<TypeNotFound>),
    
    #[error(transparent)]
    #[diagnostic(transparent)]
    AlreadyDefined(Box<AlreadyDefined>),
    
    #[error(transparent)]
    #[diagnostic(transparent)]
    ExtraAccessibilityIdentifier(Box<ExtraAccessibilityIdentifier>),
    
    #[error(transparent)]
    #[diagnostic(transparent)]
    InvalidType(Box<InvalidType>),
    
    #[error(transparent)]
    #[diagnostic(transparent)]
    InterfaceFieldNotDefined(Box<InterfaceFieldNotDefined>) ,
    
    #[error(transparent)]
    #[diagnostic(transparent)]
    TypesDoNotMatch(Box<TypesDoNotMatch>),
    
    #[error(transparent)]
    #[diagnostic(transparent)]
    ExtraFieldInInterface(Box<ExtraFieldInInterface>),

    #[error(transparent)]
    #[diagnostic(transparent)]
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
    fn get_errors(&self, _: &str) -> Vec<ErrorReport> {
       panic!("Please use get_error() for TirError, not get_errors()");
    }
    
    fn get_error_code(&self) -> i64 {
        self.get_int("code").unwrap()
    }
}

impl TirError {
    pub fn already_defined(new_position: Range<usize>, old_position: Range<usize>, source: SourceFile) -> Self {
        TirError::AlreadyDefined(AlreadyDefined {
            new_position: new_position.into(),
            old_position: old_position.into(),
            code: source.into(),
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

    pub fn type_not_found(context: &TirContext, missing_type_name: String, position: Range<usize>, source: SourceFile) -> Self {

        let mut similar_types = Vec::new();

        for (type_name, _) in context.types_scope.iter() {
            if type_name.ends_with(missing_type_name.as_str()) {
                similar_types.push(type_name.to_string());
            }
        }
        
        let advice = if !similar_types.is_empty() {
            let type_list = similar_types.iter().map(|item| format!(" - {}", item)).collect::<Vec<_>>().join("\n");
            format!("The following types are similar:\n{}", type_list)
        } else {
            TYPE_NOT_FOUND_HELP.to_string()
        };

        TirError::TypeNotFound(TypeNotFound {
            position: position.into(),
            code: source.into(),
            type_name: missing_type_name,
            advice,
        }.into())
    }

    pub fn invalid_type(position: Range<usize>, source: SourceFile) -> Self {
        TirError::InvalidType(InvalidType {
            position,
            source,
        }.into())
    }
}
