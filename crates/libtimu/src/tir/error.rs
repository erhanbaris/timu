use std::ops::Range;

use strum_macros::{EnumDiscriminants, EnumProperty};

use libtimu_macros::TimuError;
use libtimu_macros_core::{traits::LabeledSpan, SourceCode};

use crate::{file::SourceFile, tir::{resolver::ResolverError, TirContext}};

#[derive(Clone, Debug, thiserror::Error, TimuError)]
#[error("'{module}' not found")]
#[diagnostic(code("timu::error::import_not_found"), help("try to remove the import or check the module name"))]
pub struct ImportNotFound {
    pub module: String,

    #[label("no external or internal module found with this name")]
    pub position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode
}

pub static TYPE_NOT_FOUND_HELP: &str = "try to import the type, or maybe you need to define it in the current file";

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("'{type_name}' type not found")]
#[diagnostic(code("timu::error::type_not_found"))]
pub struct TypeNotFound {
    pub type_name: String,

    #[label("type is not imported or defined in the current file")]
    pub position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode,

    #[help]
    pub advice: String,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[diagnostic(code("timu::error::module_already_imported"), help("try to remove one of the import"))]
#[error("Module already defined")]
pub struct ModuleAlreadyImported {
    #[label("Already imported here")]
    pub old_position: Range<usize>,

    #[label("But it is imported again here")]
    pub new_position: Range<usize>,

    #[source_code]
    pub code: SourceCode,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Already defined")]
#[diagnostic(code("timu::error::already_imported"), help("change one of the names or remove the definition"))]
pub struct AlreadyDefined {
    #[label("Already defined here")]
    pub old_position: Range<usize>,

    #[label("But it is defined again here")]
    pub new_position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Extra accessibility identifier")]
#[diagnostic(code("timu::error::extra_accessibility_identifier"), help("remove pub"))]
pub struct ExtraAccessibilityIdentifier { 
    #[label("pub identifier is not allowed here")]
    pub position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Invalid type")]
#[diagnostic(code("timu::error::invalid_type"))]
pub struct InvalidType {
    #[label(collection, "")]
    pub position: Vec<LabeledSpan>,
    
    #[source_code]
    pub code: SourceCode,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Circular reference detected")]
#[diagnostic(code("timu::error::circular_reference"), help("to fix this, you need to remove the circular reference"))]
pub struct CircularReference {
    #[label("Has a circular reference here")]
    pub position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("ooops, multiple errors detected")]
pub struct ErrorCollection {
    #[errors]
    pub errors: Vec<TirError>
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("{} syntax error(s) detected", .errors.len())]
pub struct SyntaxError {
    #[errors]
    pub errors: Vec<SyntaxErrorItem>
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Syntax error")]
pub struct SyntaxErrorItem {
    #[label("{message}")]
    pub position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode,

    pub message: &'static str,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Interface field(s) not defined")]
#[diagnostic(code("timu::error::interface_field_not_defined"), help("to fix this, you need to define field(s) in the interface"))]
pub struct InterfaceFieldNotDefined { 
    #[label("Interface field(s) not defined here")]
    pub position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode,
 }

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Types do not match")]
#[diagnostic(code("timu::error::types_do_not_match"), help("to fix this, you need to change the type(s) to match"))]
pub struct TypesDoNotMatch {
    #[label("This type not matching")]
    pub position: Range<usize>,

    #[source_code]
    pub code: SourceCode
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Extra field in interface")]
#[diagnostic(code("timu::error::extra_field_in_interface"), help("remove the field(s) not defined in the interface"))]
pub struct ExtraFieldInExtend { 
    #[label("This field is not defined in the extend")]
    pub position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode,
}

#[derive(Clone, Debug, TimuError, thiserror::Error, EnumDiscriminants, EnumProperty)]
pub enum TirError {
    #[error("Temporary error")]
    #[diagnostic(code("Temporary error"))]
    TemporaryError,

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
    InterfaceFieldNotDefined(Box<InterfaceFieldNotDefined>),
    
    #[error(transparent)]
    #[diagnostic(transparent)]
    TypesDoNotMatch(Box<TypesDoNotMatch>),
    
    #[error(transparent)]
    #[diagnostic(transparent)]
    ExtraFieldInExtend(Box<ExtraFieldInExtend>),

    #[error(transparent)]
    #[diagnostic(transparent)]
    ResolverError(#[from] Box<ResolverError>),

    #[error(transparent)]
    #[diagnostic(transparent)]
    CircularReference(#[from] Box<CircularReference>),

    #[error(transparent)]
    #[diagnostic(transparent)]
    ErrorCollection(#[from] Box<ErrorCollection>),

    #[error(transparent)]
    #[diagnostic(transparent)]
    SyntaxError(#[from] Box<SyntaxError>),
}

impl TirError {
    pub fn already_defined(new_position: Range<usize>, old_position: Range<usize>, source: SourceFile) -> Self {
        TirError::AlreadyDefined(AlreadyDefined {
            new_position,
            old_position,
            code: source.into(),
        }.into())
    }

    pub fn interface_field_not_defined(position: Range<usize>, source: SourceFile) -> Self {
        TirError::InterfaceFieldNotDefined(InterfaceFieldNotDefined {
            position,
            code: source.into(),
        }.into())
    }

    pub fn types_do_not_match(position: Range<usize>, source: SourceFile) -> Self {
        TirError::TypesDoNotMatch(TypesDoNotMatch {
            position,
            code: source.into(),
        }.into())
    }

    pub fn extra_accessibility_identifier(position: Range<usize>, source: SourceFile) -> Self {
        TirError::ExtraAccessibilityIdentifier(ExtraAccessibilityIdentifier {
            position,
            code: source.into(),
        }.into())
    }

    pub fn extra_field_in_extend(position: Range<usize>, source: SourceFile) -> Self {
        TirError::ExtraFieldInExtend(ExtraFieldInExtend {
            position,
            code: source.into(),
        }.into())
    }

    pub fn circular_reference(position: Range<usize>, source: SourceFile) -> Self {
        TirError::CircularReference(CircularReference {
            position,
            code: source.into(),
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
            position,
            code: source.into(),
            type_name: missing_type_name,
            advice,
        }.into())
    }

    pub fn invalid_type(position: Range<usize>, message: &str, source: SourceFile) -> Self {
        TirError::InvalidType(InvalidType {
            position: vec![LabeledSpan::new(message.to_string(), position)],
            code: source.into(),
        }.into())
    }

    pub fn multiple_errors(errors: Vec<TirError>) -> Self {
        TirError::ErrorCollection(ErrorCollection {
            errors
        }.into())
    }

    pub fn syntax_error(errors: Vec<SyntaxErrorItem>) -> Self {
        TirError::SyntaxError(SyntaxError {
            errors
        }.into())
    }
}
