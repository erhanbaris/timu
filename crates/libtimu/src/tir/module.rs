//! Module system for the Timu language compiler.
//!
//! This module implements the module system that organizes code into separate
//! compilation units. Each module represents a single source file and contains
//! its own namespace for types, functions, and imported items.
//!
//! # Overview
//!
//! The module system provides:
//! - **Module isolation**: Each module has its own namespace
//! - **Import management**: Track what items are imported from other modules
//! - **Type organization**: Organize types and signatures within modules
//! - **Hierarchical structure**: Support for nested modules and submodules
//!
//! # Module Types
//!
//! - **Regular modules**: Modules with actual source code and AST
//! - **Phantom modules**: Placeholder modules for organizational purposes
//! - **Module references**: Lightweight references to modules for cross-referencing
//!
//! # Import System
//!
//! Modules can import items from other modules using `use` statements:
//! ```timu
//! use other_module.SomeClass;
//! use utils.helper as my_helper;
//! ```

use std::{borrow::Cow, rc::Rc};

use indexmap::IndexMap;

use crate::{
    ast::FileAst, 
    file::SourceFile, 
    map::TimuHashMap, 
    tir::{
        object_signature::GetItem, 
        scope::{AstVariableInformation, ScopeLocation}
    }
};

use super::{
    resolver::{AstSignatureLocation, TypeLocation}, 
    signature::SignaturePath, 
    TirContext
};

/// Represents a module in the Timu language compiler.
/// 
/// A module corresponds to a single source file and contains all the declarations,
/// imports, and metadata needed for compilation. Modules form the top-level
/// organizational unit in the Timu language.
/// 
/// # Lifetime Parameters
/// 
/// * `'base` - The lifetime of the source code being compiled
/// 
/// # Fields
/// 
/// * `name` - The module's name (usually the filename)
/// * `path` - The full module path (e.g., "utils.math")
/// * `file` - The source file containing the module's code
/// * `ast_signatures` - Signatures for all declarations in this module
/// * `ast_imported_modules` - Items imported from other modules via `use` statements
/// * `types` - Type definitions and their locations
/// * `ast` - The parsed AST for this module (None for phantom modules)
/// * `modules` - Submodules contained within this module
/// * `scope_location` - The root scope for this module
/// 
/// # Module Types
/// 
/// - **Regular Module**: Has an AST and represents actual source code
/// - **Phantom Module**: No AST, used for organizational purposes
#[derive(Debug)]
pub struct Module<'base> {
    /// The module's name (usually derived from filename)
    #[allow(dead_code)]
    pub name: Cow<'base, str>,
    /// The full module path (e.g., "utils.math.trigonometry")
    pub path: Cow<'base, str>,
    /// The source file containing this module's code
    pub file: SourceFile,
    /// Signatures for all declarations in this module (classes, functions, etc.)
    pub ast_signatures: TimuHashMap<'base, SignaturePath<'base>, AstVariableInformation<'base>>,
    /// Items imported from other modules via `use` statements
    pub ast_imported_modules: IndexMap<Cow<'base, str>, AstSignatureLocation>,
    /// Type definitions and their resolved locations
    pub types: IndexMap<SignaturePath<'base>, TypeLocation>,
    /// The parsed AST for this module (None for phantom modules)
    pub ast: Option<Rc<FileAst<'base>>>,
    /// Submodules contained within this module
    pub modules: IndexMap<Cow<'base, str>, ModuleRef<'base>>,
    /// The root scope location for this module
    pub scope_location: ScopeLocation
}

impl<'base> Module<'base> {
    /// Creates a new regular module with actual source code and AST.
    /// 
    /// Regular modules contain parsed source code and are used for normal
    /// compilation units like user-written files.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The module's name (usually filename)
    /// * `path` - The full module path
    /// * `file` - The source file
    /// * `ast` - The parsed AST
    /// * `scope_location` - The root scope for this module
    pub fn new(name: Cow<'base, str>, path: Cow<'base, str>, file: SourceFile, ast: Rc<FileAst<'base>>, scope_location: ScopeLocation) -> Self {
        Self {
            name,
            path,
            file,
            ast_signatures: TimuHashMap::new(),
            ast_imported_modules: IndexMap::new(),
            types: IndexMap::new(),
            ast: Some(ast),
            modules: IndexMap::new(),
            scope_location,
        }
    }

    /// Creates a phantom module without source code.
    /// 
    /// Phantom modules are used for organizational purposes when we need
    /// to represent module hierarchies but don't have actual source files.
    /// For example, when creating intermediate modules in a path like
    /// `utils.math.trigonometry`, we might need phantom modules for
    /// `utils` and `utils.math`.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The module's name
    /// * `path` - The full module path
    /// * `file` - A placeholder source file
    /// * `scope_location` - The root scope for this module
    pub fn phantom(name: Cow<'base, str>, path: Cow<'base, str>, file: SourceFile, scope_location: ScopeLocation) -> Self {
        Self {
            name,
            path,
            file,
            ast_imported_modules: IndexMap::new(),
            ast_signatures: TimuHashMap::new(),
            types: IndexMap::new(),
            ast: None,
            modules: IndexMap::new(),
            scope_location,
        }
    }

    /// Returns a lightweight reference to this module.
    /// 
    /// Module references are used for cross-referencing modules without
    /// holding onto the full module data.
    pub fn get_ref(&self) -> ModuleRef<'base> {
        ModuleRef::new(self.path.clone(), self.file.clone())
    }

    /// Looks up an AST signature by name within this module.
    /// 
    /// This searches for declarations (classes, functions, etc.) that are
    /// defined directly in this module.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The name of the declaration to look up
    /// 
    /// # Returns
    /// 
    /// The signature location if found, None otherwise.
    pub fn get_ast_signature<T: AsRef<str>>(&self, key: T) -> Option<AstSignatureLocation> {
        self.ast_signatures.get(key.as_ref()).map(|item| item.location)
    }
}
    

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleRef<'base>(pub Cow<'base, str>, SourceFile);

impl GetItem for ModuleRef<'_> {
    fn get_item_location(&self, context: &TirContext<'_>, path: &str) -> Option<TypeLocation> {
        self
            .upgrade(context)
            .unwrap()
            .types
            .get(path).copied()
    }
}

impl<'base> ModuleRef<'base> {
    pub fn new(path: Cow<'base, str>, file: SourceFile) -> Self {
        ModuleRef(path, file)
    }

    pub fn upgrade<'ctx>(&self, context: &'ctx TirContext<'base>) -> Option<&'ctx Module<'base>> {
        context.modules.get(self.0.as_ref())
    }

    pub fn file(&self) -> SourceFile {
        self.1.clone()
    }

    pub fn as_cow(&self) -> Cow<'base, str> {
        self.0.clone()
    }
}

impl core::convert::AsRef<str> for ModuleRef<'_> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Default for ModuleRef<'_> {
    fn default() -> Self {
        use crate::file::SourceFile;
        Self::new("default".into(), SourceFile::new(vec!["default".into()], "default".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{file::SourceFile, nom_tools::State, process_code, tir::scope::ScopeLocation};

    #[test]
    fn module_creation() {
        let source_file = SourceFile::new(vec!["test".into()], "class Test {}".to_string());
        let state = State::new(source_file.clone());
        let ast = process_code(&state).unwrap();
        let ast_rc = std::rc::Rc::new(ast);
        
        let module = Module::new(
            "test_module".into(),
            "test.path".into(),
            source_file.clone(),
            ast_rc.clone(),
            ScopeLocation(0)
        );
        
        assert_eq!(module.name, "test_module");
        assert_eq!(module.path, "test.path");
        assert_eq!(module.scope_location.0, 0);
        assert!(module.ast.is_some());
        assert_eq!(module.ast_signatures.len(), 0);
        assert_eq!(module.types.len(), 0);
    }

    #[test]
    fn module_phantom_creation() {
        let source_file = SourceFile::new(vec!["phantom".into()], "".to_string());
        
        let phantom_module = Module::phantom(
            "phantom_module".into(),
            "phantom.path".into(),
            source_file.clone(),
            ScopeLocation(1)
        );
        
        assert_eq!(phantom_module.name, "phantom_module");
        assert_eq!(phantom_module.path, "phantom.path");
        assert_eq!(phantom_module.scope_location.0, 1);
        assert!(phantom_module.ast.is_none());
    }

    #[test]
    fn module_ref_creation_and_methods() {
        let source_file = SourceFile::new(vec!["ref_test".into()], "".to_string());
        let module_ref = ModuleRef::new("test_ref".into(), source_file.clone());
        
        assert_eq!(module_ref.as_ref(), "test_ref");
        assert_eq!(module_ref.as_cow(), "test_ref");
        assert_eq!(module_ref.file(), source_file);
    }

    #[test]
    fn module_ref_default() {
        let default_ref = ModuleRef::default();
        assert_eq!(default_ref.as_ref(), "default");
    }

    #[test]
    fn module_get_ref() {
        let source_file = SourceFile::new(vec!["get_ref_test".into()], "class Test {}".to_string());
        let state = State::new(source_file.clone());
        let ast = process_code(&state).unwrap();
        let ast_rc = std::rc::Rc::new(ast);
        
        let module = Module::new(
            "test_module".into(),
            "test.path".into(),
            source_file.clone(),
            ast_rc,
            ScopeLocation(0)
        );
        
        let module_ref = module.get_ref();
        assert_eq!(module_ref.as_ref(), "test.path");
        assert_eq!(module_ref.file(), source_file);
    }

    #[test]
    fn module_get_ast_signature_none() {
        let source_file = SourceFile::new(vec!["signature_test".into()], "".to_string());
        let phantom_module = Module::phantom(
            "signature_module".into(),
            "signature.path".into(),
            source_file,
            ScopeLocation(0)
        );
        
        let result = phantom_module.get_ast_signature("non_existent");
        assert!(result.is_none());
    }

    #[test]
    fn module_ref_upgrade_none() {
        use crate::tir::context::TirContext;
        
        let source_file = SourceFile::new(vec!["upgrade_test".into()], "".to_string());
        let module_ref = ModuleRef::new("non_existent".into(), source_file);
        let context = TirContext::default();
        
        let result = module_ref.upgrade(&context);
        assert!(result.is_none());
    }
}
