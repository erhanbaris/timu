use std::borrow::Cow;

use indexmap::IndexMap;
use strum::EnumProperty;
use strum_macros::{EnumDiscriminants, EnumProperty};

use crate::nom_tools::{Span, ToRange};

use super::{error::CustomError, module::ModuleRef, resolver::{ResolverError, TypeLocation}, signature::LocationTrait, TirContext, TirError};


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ScopeLocation(#[allow(dead_code)]pub usize);

impl ScopeLocation {
    pub const UNDEFINED: Self = ScopeLocation(usize::MAX);
}

impl From<usize> for ScopeLocation {
    fn from(signature_location: usize) -> Self {
        ScopeLocation(signature_location)
    }
}

impl LocationTrait for ScopeLocation {
    fn get(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Scope<'base> {
    pub module_ref: ModuleRef<'base>,
    variables: IndexMap<Cow<'base, str>, TypeLocation>,
    pub parent_scope: Option<ScopeLocation>,
    pub parent_type: Option<TypeLocation>,
    pub current_type: TypeLocation,
    pub location: ScopeLocation,
}

impl<'base> Scope<'base> {
    pub fn new(module_ref: ModuleRef<'base>, parent_scope: Option<ScopeLocation>,  parent_type: Option<TypeLocation>, location: ScopeLocation) -> Self {
        Self {
            module_ref,
            variables: IndexMap::new(),
            parent_scope,
            parent_type,
            location,
            current_type: TypeLocation::UNDEFINED
        }
    }

    pub fn get_variable<T: AsRef<str>>(&self, context: &TirContext<'base>, name: T) -> Option<TypeLocation> {
        if let Some(variable) = self.variables.get(name.as_ref()) {
            return Some(*variable);
        }

        if let Some(type_location) = self.parent_scope.and_then(|parent_location| context.get_scope(parent_location)).and_then(|parent_scope| parent_scope.get_variable(context, name.as_ref())) {
            return Some(type_location);
        }

        let module = self.module_ref.upgrade(context).unwrap();
        if let Some(type_location) = module.types.get(name.as_ref()) {
            return Some(*type_location);
        }

        if let Some(module_ref) = module.modules.get(name.as_ref()) {
            if let Some(type_location) = module.types.get(module_ref.0.as_ref()) {
                return Some(*type_location);
            }
        }
        
        None
    }

    pub fn add_variable(&mut self, name: Span<'base>, location: TypeLocation) -> Result<(), TirError<'base>> {
        simplelog::debug!("Adding variable: <u><b><on-green>{}</></b></u>, location <u><b>{:?}</b></u>", name.fragment(), location);
        if self.variables.insert((*name.fragment()).into(), location).is_some() {
            return Err(TirError::already_defined(name.to_range(), name.extra.file.clone()));
        }
        Ok(())
    }

    pub fn set_current_type(&mut self, type_location: TypeLocation) {
        self.current_type = type_location;
    }
}

#[derive(Debug, thiserror::Error, EnumDiscriminants, EnumProperty)]
pub enum ScopeError<'base> {
    #[error("Variable already defined: {0}")]
    #[strum(props(code=1))]
    VariableAlreadyDefined(Span<'base>),
}

impl<'base> From<ScopeError<'base>> for TirError<'base> {
    fn from(value: ScopeError<'base>) -> Self {
        ResolverError::Scope(Box::new(value)).into()
    }
}

impl CustomError for ScopeError<'_> {
    fn get_errors(&self, parent_error_code: &str) -> Vec<crate::tir::error::ErrorReport<'_>> {
        match self {
            ScopeError::VariableAlreadyDefined(span) => {
                vec![crate::tir::error::ErrorReport {
                    position: span.to_range(),
                    message: format!("{}", self),
                    file: span.extra.file.clone(),
                    error_code: self.build_error_code(parent_error_code),
                }]
            }
        }
    }
    
    fn get_error_code(&self) -> i64 {
        self.get_int("code").unwrap()
    }
}
