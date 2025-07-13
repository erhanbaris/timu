//! Scope management and variable resolution for the Timu language compiler.
//!
//! This module implements the hierarchical scope system that tracks variables,
//! types, and their locations throughout the compilation process. Scopes form
//! a tree structure where child scopes can access variables from parent scopes.
//!
//! # Overview
//!
//! The scope system provides:
//! - **Variable tracking**: Maps variable names to their type information
//! - **Hierarchical lookup**: Child scopes inherit from parent scopes
//! - **Module integration**: Each scope belongs to a specific module
//! - **Type resolution**: Resolves variable types and module references
//!
//! # Scope Hierarchy
//!
//! ```text
//! Module Scope (top-level)
//! ├── Class Scope
//! │   ├── Method Scope
//! │   │   └── Local Variable Scope
//! │   └── Another Method Scope
//! └── Function Scope
//!     └── Local Variable Scope
//! ```
//!
//! # Variable Resolution Order
//!
//! When resolving a variable name, the system searches in this order:
//! 1. Current scope variables
//! 2. Parent scope variables (recursive)
//! 3. Module-level types and functions
//! 4. Imported modules and their exports
//! 5. Global type registry

use std::{borrow::Cow, fmt::Debug};

use libtimu_macros::TimuError;
use simplelog::debug;
use strum_macros::{EnumDiscriminants, EnumProperty};

use crate::{
    map::{TimuHashMap, ValueTrait}, 
    nom_tools::{Span, SpanInfo}, 
    tir::resolver::{AstSignatureLocation, BuildFullNameLocater, ResolveAst}
};

use super::{
    module::ModuleRef, 
    resolver::{ResolverError, TypeLocation}, 
    signature::LocationTrait, 
    TirContext, 
    TirError
};


/// A unique identifier for a scope within the compilation context.
/// 
/// Scopes are identified by their index in the TIR context's scope vector.
/// This allows for efficient lookup and hierarchy traversal.
/// 
/// # Examples
/// 
/// ```ignore
/// let scope_loc = ScopeLocation(0);  // First scope in the context
/// let undefined = ScopeLocation::UNDEFINED;  // Invalid/uninitialized scope
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ScopeLocation(#[allow(dead_code)]pub usize);

impl ScopeLocation {
    /// Represents an undefined or invalid scope location.
    /// 
    /// This is used as a sentinel value when no valid scope exists.
    pub const UNDEFINED: Self = ScopeLocation(usize::MAX);
}

impl From<usize> for ScopeLocation {
    /// Creates a `ScopeLocation` from a raw index.
    fn from(signature_location: usize) -> Self {
        ScopeLocation(signature_location)
    }
}

impl LocationTrait for ScopeLocation {
    /// Returns the raw index of this scope location.
    fn get(&self) -> usize {
        self.0
    }
}

/// Information about a variable in a scope, including its type and modifiers.
/// 
/// This structure tracks all the metadata needed for a variable including
/// its source location, type information, and various language modifiers.
/// 
/// # Type Parameters
/// 
/// * `'base` - The lifetime of the source code being compiled
/// * `L` - The location type (usually `TypeLocation` or `AstSignatureLocation`)
/// 
/// # Fields
/// 
/// * `span` - Source code location where the variable is defined
/// * `location` - Type or signature location for type resolution
/// * `nullable` - Whether the variable can hold null values
/// * `reference` - Whether this is a reference to another variable
/// * `readonly` - Whether the variable is immutable after initialization
/// 
/// # Examples
/// 
/// ```timu
/// let name: string = "hello";        // Basic variable
/// let count: i32? = null;            // Nullable variable  
/// ref data: &SomeClass = other;      // Reference variable
/// readonly PI: f64 = 3.14159;       // Read-only variable
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct VariableInformation<'base, L: Debug + Clone + PartialEq> {
    /// Source code span where this variable is defined
    pub span: Span<'base>,
    /// Location in the type system (TypeLocation or AstSignatureLocation)
    pub location: L,
    /// Whether this variable can hold null values
    pub nullable: bool,
    /// Whether this is a reference to another variable
    pub reference: bool,
    /// Whether this variable is read-only after initialization
    pub readonly: bool,
}

impl<'base, L> ValueTrait<'base> for VariableInformation<'base, L>
where L: Debug + Clone + PartialEq
{
    fn get_span(&self) -> Span<'base> {
        self.span.clone()
    }
}

impl<'base, L> VariableInformation<'base, L> 
where L: Debug + Clone + PartialEq
{
    /// Creates a new VariableInformation with all properties explicitly specified
    /// 
    /// This constructor allows full control over all variable properties including
    /// nullability, reference semantics, and mutability constraints.
    /// 
    /// # Arguments
    /// * `span` - Source location span for the variable declaration
    /// * `location` - Type location or other identifier for the variable's type
    /// * `nullable` - Whether the variable can hold null/None values
    /// * `reference` - Whether the variable is a reference type
    /// * `readonly` - Whether the variable is immutable after initialization
    /// 
    /// # Returns
    /// A new VariableInformation instance with the specified properties
    /// 
    /// # Usage
    /// This is the most flexible constructor, typically used when you need
    /// precise control over variable semantics, such as when processing
    /// explicit type annotations with modifiers.
    pub fn new(span: Span<'base>, location: L, nullable: bool, reference: bool, readonly: bool) -> Self {
        Self {
            span,
            location,
            nullable,
            reference,
            readonly,
        }
    }

    /// Creates a basic VariableInformation with default properties
    /// 
    /// This convenience constructor creates a variable with the most common
    /// default settings: non-nullable, non-reference, and mutable.
    /// 
    /// # Arguments
    /// * `span` - Source location span for the variable declaration
    /// * `location` - Type location or other identifier for the variable's type
    /// 
    /// # Returns
    /// A new VariableInformation with nullable=false, reference=false, readonly=false
    /// 
    /// # Usage
    /// This is the most commonly used constructor for typical variable declarations
    /// where no special type modifiers are specified.
    /// 
    /// # Examples
    /// Used for simple variable declarations like:
    /// - `let x: string`
    /// - `var count: i32`
    /// - Class field declarations without modifiers
    pub fn basic(span: Span<'base>, location: L) -> Self {
        Self::new(span, location, false, false, false)
    }

    /// Creates a readonly VariableInformation instance
    /// 
    /// This convenience constructor creates a variable that cannot be modified
    /// after initialization, equivalent to const or final semantics in other languages.
    /// 
    /// # Arguments
    /// * `span` - Source location span for the variable declaration
    /// * `location` - Type location or other identifier for the variable's type
    /// 
    /// # Returns
    /// A new VariableInformation with readonly=true, nullable=false, reference=false
    /// 
    /// # Usage
    /// Used for constant declarations and immutable variables where the value
    /// cannot change after initialization.
    /// 
    /// # Examples
    /// Used for declarations like:
    /// - `const PI: f64 = 3.14159`
    /// - `final result: string`
    /// - Function parameters that should not be modified
    pub fn readonly(span: Span<'base>, location: L) -> Self {
        Self::new(span, location, false, false, true)
    }

    pub fn reference(span: Span<'base>, location: L) -> Self {
        Self::new(span, location, false, true, false)
    }

    pub fn nullable(span: Span<'base>, location: L) -> Self {
        Self::new(span, location, true, false, false)
    }
}

pub type TypeVariableInformation<'base> = VariableInformation<'base, TypeLocation>;
pub type AstVariableInformation<'base> = VariableInformation<'base, AstSignatureLocation>;

#[derive(Debug, Clone)]
pub struct Scope<'base> {
    pub module_ref: ModuleRef<'base>,
    variables: TimuHashMap<'base, Cow<'base, str>, TypeVariableInformation<'base>>,
    pub parent_scope: Option<ScopeLocation>,
    pub parent_type: Option<TypeLocation>,
    pub current_type: TypeLocation,
    pub location: ScopeLocation,
}

impl<'base> Scope<'base> {
    pub fn new(module_ref: ModuleRef<'base>, parent_scope: Option<ScopeLocation>,  parent_type: Option<TypeLocation>, location: ScopeLocation) -> Self {
        Self {
            module_ref,
            variables: TimuHashMap::new(),
            parent_scope,
            parent_type,
            location,
            current_type: TypeLocation::UNDEFINED
        }
    }

    pub fn get_variable(&self, context: &TirContext<'base>, name_span: &Span<'base>) -> Option<TypeVariableInformation<'base>> {
        let name = name_span.text;
        debug!("get_variable: name: {}, scope: {}", name, self.location.0);

        /* Search in current scope */
        if let Some(variable) = self.variables.get(name) {
            return Some(variable.clone());
        }

        /* Search in parent scope */
        if let Some(type_location) = self.parent_scope.and_then(|parent_location| context.get_scope(parent_location)).and_then(|parent_scope| parent_scope.get_variable(context, name_span)) {
            return Some(type_location);
        }

        /* Search in module level */
        let module = self.module_ref.upgrade(context).unwrap();
        let module_scope = context.get_scope(module.scope_location).unwrap_or_else(|| panic!("Module scope not found for module: {}", module.path));

        if module_scope.location != self.location {
            if let Some(variable_info) = module_scope.get_variable(context, name_span) {
                return Some(variable_info);
            }
        }

        /* Handle type and module resolution */
        if let Some(type_location) = module.types.get(name) {
            return Some(TypeVariableInformation::basic(name_span.clone(), *type_location));
        }

        /* Search as a module name  */
        if let Some(module_ref) = context.modules.get(name) {
            if let Some(type_location) = module.types.get(module_ref.path.as_ref()) {
                return Some(TypeVariableInformation::basic(name_span.clone(), *type_location));
            }
        }

        if let Some(type_location) = context.types.location(name) {
            return Some(TypeVariableInformation::basic(name_span.clone(), type_location));
        }

        if let Some(ast_location) = module.ast_imported_modules.get(name) {
            if let Some(signature) = context.ast_signatures.get_from_location(*ast_location) {
                let full_name = signature.value.build_full_name(context, BuildFullNameLocater::Module(signature.extra.as_ref().unwrap()), None);

                if let Some(type_location) = context.types.location(full_name.as_str()) {
                    return Some(TypeVariableInformation::basic(name_span.clone(), type_location));
                }
            }
        }

        if let Some(module_ref) = context.modules.get(name) {
            if let Some(type_location) = module.types.get(module_ref.path.as_ref()) {
                return Some(TypeVariableInformation::basic(name_span.clone(), *type_location));
            }
        }

        None
    }

    pub fn add_variable(&mut self, variable: TypeVariableInformation<'base>) -> Result<(), TirError> {
        simplelog::debug!("Adding variable: <u><b><on-green>{}</></b></u>, location <u><b>{:?}</b></u>, scope: {}", variable.span.text, variable.location, self.location.0);
        self.variables.validate_insert((*variable.span.text).into(), variable)?;
        Ok(())
    }

    pub fn set_current_type(&mut self, type_location: TypeLocation) {
        self.current_type = type_location;
    }
}

#[derive(Clone, Debug, TimuError, thiserror::Error, EnumDiscriminants, EnumProperty)]
pub enum ScopeError {
    #[error("Variable already defined")]
    VariableAlreadyDefined(SpanInfo),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{file::SourceFile, nom_tools::State, tir::resolver::TypeLocation};

    #[test]
    fn scope_add_variable_success() {
        let state = State::new(SourceFile::new(vec!["test".into()], "test".to_string()));
        let span = crate::nom_tools::Span {
            text: "test_var",
            position: 0..8,
            state: state.clone(),
        };
        
        let mut scope = Scope::new(
            ModuleRef::default(),
            None,
            None,
            ScopeLocation(0)
        );
        
        let variable = TypeVariableInformation::basic(span, TypeLocation(1));
        let result = scope.add_variable(variable);
        assert!(result.is_ok());
    }

    #[test]
    fn scope_add_duplicate_variable_error() {
        let state = State::new(SourceFile::new(vec!["test".into()], "test".to_string()));
        let span = crate::nom_tools::Span {
            text: "test_var",
            position: 0..8,
            state: state.clone(),
        };
        
        let mut scope = Scope::new(
            ModuleRef::default(),
            None,
            None,
            ScopeLocation(0)
        );
        
        let variable1 = TypeVariableInformation::basic(span.clone(), TypeLocation(1));
        let variable2 = TypeVariableInformation::basic(span, TypeLocation(2));
        
        scope.add_variable(variable1).unwrap();
        let result = scope.add_variable(variable2);
        assert!(result.is_err());
    }

    #[test]
    fn scope_set_current_type() {
        let mut scope = Scope::new(
            ModuleRef::default(),
            None,
            None,
            ScopeLocation(0)
        );
        
        scope.set_current_type(TypeLocation(5));
        assert_eq!(scope.current_type, TypeLocation(5));
    }

    #[test]
    fn scope_location_constants() {
        assert_eq!(ScopeLocation::UNDEFINED.0, usize::MAX);
    }
}

impl From<ScopeError> for TirError {
    fn from(value: ScopeError) -> Self {
        ResolverError::Scope(Box::new(value)).into()
    }
}
