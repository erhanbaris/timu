//! TIR (Type Intermediate Representation) context management.
//!
//! This module contains the central `TirContext` that coordinates all aspects
//! of the type resolution and intermediate representation phase of compilation.
//! The context serves as the single source of truth for all modules, types,
//! scopes, and compilation state.
//!
//! # Overview
//!
//! The `TirContext` manages:
//! - **Modules**: All loaded modules and their relationships
//! - **Signatures**: Type and AST signatures for all declarations
//! - **Scopes**: Hierarchical scope tree for variable resolution
//! - **Types**: Mapping between AST nodes and their resolved types
//! - **Errors**: Collection of compilation errors encountered
//!
//! # Compilation Flow
//!
//! 1. **Module Registration**: Load and register all source modules
//! 2. **Signature Creation**: Create signatures for all top-level declarations
//! 3. **Scope Building**: Build the hierarchical scope tree
//! 4. **Type Resolution**: Resolve all type references and expressions
//! 5. **Error Reporting**: Collect and report any compilation errors
//!
//! # Thread Safety
//!
//! The context uses atomic operations for temporary type generation,
//! making it safe to use in multi-threaded compilation scenarios.

use std::{borrow::Cow, ops::Range, sync::atomic::{AtomicUsize, Ordering}};

use indexmap::IndexMap;
use simplelog::debug;

use crate::{ast::AstIndex, file::SourceFile, tir::object_signature::TypeValueDiscriminants};

use super::{
    module::ModuleRef, 
    resolver::{AstSignatureLocation, ResolveAst, TypeLocation}, 
    scope::{Scope, ScopeLocation}, 
    signature::SignaturePath, 
    AstSignature, 
    AstSignatureHolder, 
    Module, 
    TirError, 
    TypeSignature, 
    TypeSignatureHolder
};

/// The central context for Type Intermediate Representation (TIR) compilation.
/// 
/// This structure maintains all the state needed during the type resolution
/// and intermediate representation phase of compilation. It serves as the
/// coordination point for modules, types, scopes, and error reporting.
/// 
/// # Lifetime Parameters
/// 
/// * `'base` - The lifetime of the source code being compiled
/// 
/// # Fields
/// 
/// * `modules` - All loaded modules indexed by their path
/// * `ast_signatures` - Signatures for all AST declarations (classes, functions, etc.)
/// * `types` - Type signatures for resolved types
/// * `scopes` - Vector of all scopes in the compilation unit
/// * `types_scope` - Mapping from type names to their defining scopes
/// * `ast_type` - Mapping from AST nodes to their resolved types
/// * `tmp_type_indexer` - Atomic counter for generating unique temporary type names
/// * `errors` - Collection of all compilation errors encountered
/// 
/// # Examples
/// 
/// ```ignore
/// let mut context = TirContext::default();
/// 
/// // Generate temporary type name
/// let tmp_name = context.create_tmp_type(); // "$timu_type_0$"
/// 
/// // Access modules and errors
/// let module_count = context.modules.len();
/// let error_count = context.errors.len();
/// ```
#[derive(Debug, Default)]
pub struct TirContext<'base> {
    /// All modules in the compilation unit, indexed by module path
    pub modules: IndexMap<Cow<'base, str>, Module<'base>>,
    /// AST signatures for all top-level declarations
    pub ast_signatures: AstSignatureHolder<'base>,
    /// Type signatures for all resolved types
    #[allow(dead_code)]
    pub types: TypeSignatureHolder<'base>,
    /// All scopes in the compilation unit
    pub scopes: Vec<Scope<'base>>,
    /// Mapping from type names to their defining scopes
    pub types_scope: IndexMap<Cow<'base, str>, ScopeLocation>,
    /// Mapping from AST nodes to their resolved types
    pub ast_type: IndexMap<AstIndex, TypeLocation>,
    /// Atomic counter for generating unique temporary type names
    pub tmp_type_indexer: AtomicUsize,
    /// Collection of all compilation errors
    pub errors: Vec<TirError>,
}

impl<'base> TirContext<'base> {
    /// Creates a unique temporary type name for internal compiler use
    /// 
    /// This method generates a unique temporary type identifier that can be used
    /// for internal compiler operations like intermediate type representations
    /// during complex type resolution processes.
    /// 
    /// # Returns
    /// A unique string in the format `$timu_type_N$` where N is an incrementing counter
    /// 
    /// # Thread Safety
    /// This method is thread-safe due to the use of atomic operations for the counter.
    /// 
    /// # Usage
    /// Typically used during type inference and resolution when temporary type
    /// placeholders are needed before final types are determined.
    pub fn create_tmp_type(&self) -> String {
        format!("$timu_type_{}$", self.tmp_type_indexer.fetch_add(1, Ordering::SeqCst))
    }

    /// Retrieves an AST signature by its string key
    /// 
    /// This method looks up an AST signature in the signature holder using a string key.
    /// AST signatures contain metadata about top-level declarations before they are
    /// fully resolved into the type system.
    /// 
    /// # Arguments
    /// * `key` - A string-like type that can be converted to a string reference
    /// 
    /// # Returns
    /// `Some(&AstSignature)` if found, `None` if no signature exists for the key
    /// 
    /// # Usage
    /// Used during resolution phases to look up previously registered AST signatures
    /// for functions, classes, interfaces, and other top-level declarations.
    pub fn get_ast_signature<T: AsRef<str>>(&self, key: T) -> Option<&AstSignature<'base>> {
        self.ast_signatures.get(key.as_ref())
    }

    /// Retrieves the location identifier for an AST signature by its string key
    /// 
    /// This method returns the location identifier associated with an AST signature,
    /// which can be used for efficient lookups and cross-references throughout
    /// the compilation process.
    /// 
    /// # Arguments
    /// * `key` - A string-like type that can be converted to a string reference
    /// 
    /// # Returns
    /// `Some(AstSignatureLocation)` if found, `None` if no signature exists for the key
    /// 
    /// # Usage
    /// Used when you need the location identifier rather than the full signature,
    /// typically for establishing references between different parts of the AST.
    pub fn get_ast_location<T: AsRef<str>>(&self, key: T) -> Option<AstSignatureLocation> {
        self.ast_signatures.location(key.as_ref())
    }

    pub fn add_ast_signature(&mut self, key: Cow<'base, str>, signature: AstSignature<'base>) -> Result<AstSignatureLocation, TirError> {
        self.ast_signatures.add_signature(SignaturePath::cow(key), signature)
    }

    pub fn reserve_object_location(&mut self, object_name: Cow<'base, str>, type_shadow: TypeValueDiscriminants, signature_path: SignaturePath<'base>, module_ref: &ModuleRef<'base>, position: Range<usize>, source: SourceFile) -> Result<(SignaturePath<'base>, TypeLocation), TirError> {
        let module = self.modules.get_mut(module_ref.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module_ref.as_ref()));

        debug!("Reserving object location: <u><b>{}</b></u> in module <u><b>{}</b></u>", object_name, module_ref.as_ref());
        //add the signature to the context with full path
        let signature_location = self.types.reserve(signature_path.clone(), object_name.clone(), type_shadow, source.clone(), position.clone())?;

        //add the signature to the module with only the name
        module.types.insert(SignaturePath::cow(object_name), signature_location);
        Ok((signature_path, signature_location))
    }

    pub fn publish_object_location(&mut self, name: SignaturePath<'base>, signature: TypeSignature<'base>) {
        self.types.update(name, signature);
    }

    pub fn resolve<T: ResolveAst<'base>>(&mut self, signature: &T, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        signature.resolve(self, scope_location)
    }

    pub fn resolve_from_location(&mut self, signature_location: AstSignatureLocation, module_ref: &ModuleRef<'base>, parent_scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        let signature = self.ast_signatures.get_from_location(signature_location).map(|signature| signature.value.clone()).unwrap();
        let type_name = format!("{}.{}", module_ref.as_ref(), signature.name()); // todo: maybe it will not work with class function
        let type_location = self.types.location(&type_name);
        let scope_location = self.create_child_scope(type_name.into(), parent_scope_location, type_location);
        self.resolve(&signature, scope_location)
    }

    fn inner_create_scope(&mut self, type_info: Cow<'base, str>, module_ref: ModuleRef<'base>, parent_scope: Option<ScopeLocation>, parent_type: Option<TypeLocation>, current_type: Option<TypeLocation>) -> ScopeLocation {
        let scope_location = ScopeLocation(self.scopes.len());
        let mut scope = Scope::new(module_ref, parent_scope, parent_type, scope_location);

        if let Some(current_type) = current_type {
            scope.current_type = current_type;
        }

        self.scopes.push(scope);
        self.types_scope.insert(type_info, scope_location);
        scope_location
    }

    pub fn get_next_scope_location(&self) -> ScopeLocation {
        ScopeLocation(self.scopes.len())
    }

    pub fn create_scope(&mut self, type_info: Cow<'base, str>, module_ref: ModuleRef<'base>) -> ScopeLocation {
        let scope_location = self.inner_create_scope(type_info.clone(), module_ref, None, None, None);
        debug!("<on-yellow>New scope</>: {}(Parent) [{}]", scope_location.0, type_info);
        scope_location
    }

    pub fn create_child_scope(&mut self, type_info: Cow<'base, str>, parent_scope: ScopeLocation, current_type: Option<TypeLocation>) -> ScopeLocation {
        let tmp_parent_scope: ScopeLocation = parent_scope;
        let parent_scope = self.get_scope(parent_scope).expect("Parent scope not found, it is a bug");
        let scope_location = self.inner_create_scope(type_info.clone(), parent_scope.module_ref.clone(), Some(parent_scope.location), Some(parent_scope.current_type), current_type);
        debug!("<on-yellow>New scope</>: {}(Parent) -> {}(Child) [{}]", tmp_parent_scope.0, scope_location.0, type_info);
        scope_location
    }

    pub fn get_scope(&self, key: ScopeLocation) -> Option<&Scope<'base>> {
        self.scopes.get(key.0)
    }

    pub fn get_mut_scope(&mut self, key: ScopeLocation) -> Option<&mut Scope<'base>> {
        self.scopes.get_mut(key.0)
    }

    pub fn add_error(&mut self, error: TirError) {
        self.errors.push(error);
    }

    pub fn add_errors(&mut self, mut errors: Vec<TirError>) {
        self.errors.append(&mut errors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{file::SourceFile, nom_tools::State};

    #[test]
    fn context_create_tmp_type() {
        let context = TirContext::default();
        let tmp_type1 = context.create_tmp_type();
        let tmp_type2 = context.create_tmp_type();
        
        assert!(tmp_type1.starts_with("$timu_type_"));
        assert!(tmp_type2.starts_with("$timu_type_"));
        assert_ne!(tmp_type1, tmp_type2);
    }

    #[test]
    fn context_scope_management() {
        let mut context = TirContext::default();
        let state = State::new(SourceFile::new(vec!["test".into()], "test".to_string()));
        let module_ref = ModuleRef::new("test".into(), state.file.clone());
        
        // Create root scope
        let scope_location = context.create_scope("test_type".into(), module_ref.clone());
        assert_eq!(scope_location.0, 0);
        
        // Get scope
        let scope = context.get_scope(scope_location);
        assert!(scope.is_some());
        
        // Create child scope
        let child_scope = context.create_child_scope("child_type".into(), scope_location, None);
        assert_eq!(child_scope.0, 1);
        
        // Get next scope location
        let next_location = context.get_next_scope_location();
        assert_eq!(next_location.0, 2);
    }

    #[test]
    fn context_error_management() {
        let mut context = TirContext::default();
        let state = State::new(SourceFile::new(vec!["test".into()], "test".to_string()));
        
        assert_eq!(context.errors.len(), 0);
        
        // Add single error - create a simple error variant that exists
        let error = TirError::ImportNotFound(
            crate::tir::error::ImportNotFound {
                module: "test".to_string(),
                position: 0..4,
                code: state.file.clone().into(),
            }.into()
        );
        context.add_error(error);
        assert_eq!(context.errors.len(), 1);
        
        // Add multiple errors
        let errors = vec![
            TirError::ImportNotFound(
                crate::tir::error::ImportNotFound {
                    module: "test1".to_string(),
                    position: 0..5,
                    code: state.file.clone().into(),
                }.into()
            ),
            TirError::ImportNotFound(
                crate::tir::error::ImportNotFound {
                    module: "test2".to_string(),
                    position: 0..5,
                    code: state.file.clone().into(),
                }.into()
            ),
        ];
        context.add_errors(errors);
        assert_eq!(context.errors.len(), 3);
    }

    #[test]
    fn context_ast_signature_management() {
        let context = TirContext::default();
        
        // Test get_ast_signature with non-existent key
        let signature = context.get_ast_signature("non_existent");
        assert!(signature.is_none());
        
        // Test get_ast_location with non-existent key
        let location = context.get_ast_location("non_existent");
        assert!(location.is_none());
    }

    #[test]
    fn context_scope_get_mut() {
        let mut context = TirContext::default();
        let state = State::new(SourceFile::new(vec!["test".into()], "test".to_string()));
        let module_ref = ModuleRef::new("test".into(), state.file.clone());
        
        let scope_location = context.create_scope("test_type".into(), module_ref);
        
        // Test mutable access
        let scope_mut = context.get_mut_scope(scope_location);
        assert!(scope_mut.is_some());
        
        // Test invalid location
        let invalid_scope = context.get_mut_scope(ScopeLocation(999));
        assert!(invalid_scope.is_none());
    }
}
