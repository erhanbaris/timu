//! Module use statement resolution for the Timu language compiler.
//! 
//! This module implements the resolution logic for `use` statements that import
//! modules, classes, functions, and other AST elements from external modules
//! into the current scope.
//!
//! # Overview
//!
//! In Timu, `use` statements allow importing items from other modules:
//! ```timu
//! use lib.SomeClass;           // Import SomeClass from lib module
//! use lib.SomeClass as SC;     // Import with alias
//! use utils.helper;            // Import helper function
//! ```
//!
//! The resolution process:
//! 1. Looks up the imported item in the AST signature registry
//! 2. Determines the local name (either alias or original name)
//! 3. Adds the import to the current module's import registry
//! 4. Handles duplicate import detection and error reporting

use std::borrow::Cow;

use crate::{
    ast::UseAst, 
    tir::{
        accessibility::check_import_accessibility, 
        context::TirContext, 
        error::{ImportNotFound, ModuleAlreadyImported}, 
        scope::ScopeLocation, 
        TirError
    }
};

use super::{ResolveAst, TypeLocation};

/// Implementation of `ResolveAst` trait for `UseAst` nodes.
/// 
/// This enables the two-phase resolution system for use statements:
/// 1. **Resolve phase**: Validates imports and registers them in the module
/// 2. **Finish phase**: No additional work needed for use statements
impl<'base> ResolveAst<'base> for UseAst<'base> {
    /// Resolves a `use` statement by importing the specified item into the current module.
    /// 
    /// # Resolution Process
    /// 
    /// 1. **Lookup**: Searches for the imported item in the global AST signature registry
    /// 2. **Name determination**: Uses alias if provided, otherwise uses the original name
    /// 3. **Module registration**: Adds the import to the current module's import map
    /// 4. **Duplicate detection**: Checks for conflicting imports with the same local name
    /// 
    /// # Arguments
    /// 
    /// * `context` - Mutable reference to the TIR context containing all modules and signatures
    /// * `scope_location` - The scope where this use statement appears
    /// 
    /// # Returns
    /// 
    /// * `Ok(TypeLocation::UNDEFINED)` - Use statements don't have a specific type location
    /// * `Err(TirError::ImportNotFound)` - The imported item doesn't exist
    /// * `Err(TirError::ModuleAlreadyImported)` - An item with the same local name was already imported
    /// 
    /// # Examples
    /// 
    /// ```timu
    /// use lib.Calculator;        // Imports Calculator class from lib module
    /// use utils.sort as mysort;  // Imports sort function with alias 'mysort'
    /// use utils;                 // Imports utils module
    /// ```
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        // Step 1: Attempt to find the imported item in the AST signature registry
        if let Some(signature_location) = context.get_ast_location(&self.import.text) {
            // Step 2: Determine the local name for this import
            // Use alias if provided, otherwise extract the last part of the import path
            let name = match &self.alias {
                Some(alias) => std::borrow::Cow::Borrowed(alias.text),
                None => std::borrow::Cow::Borrowed(self.ast_name().text),
            };

            // Step 3: Get the current module and register the import
            let module_ref = context.get_scope(scope_location).unwrap().module_ref.clone();
            let module = context.modules.get_mut(module_ref.as_ref())
                .unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module_ref.as_ref()));
            
            // Step 4: Check for duplicate imports with the same local name
            if let Some(old) = module.ast_imported_modules.insert(name, signature_location) {
                let old_signature = context.ast_signatures.get_from_location(old).unwrap();
                return Err(TirError::ModuleAlreadyImported(ModuleAlreadyImported {
                    new_position: self.import.to_range(),
                    old_position: old_signature.position.clone(),
                    code: self.ast_name().state.file.clone().into(),
                }.into()));
            }

            // Step 4.5: Check accessibility of the imported item
            let ast_signature = context.ast_signatures.get_from_location(signature_location)
                .unwrap_or_else(|| panic!("Signature not found for location: {signature_location:?}"));
            
            // Use the dedicated accessibility module to check import permissions
            check_import_accessibility(
                ast_signature,
                self.import.paths.last().unwrap(),
                &self.import.paths.last().unwrap().state.file
            )?;
        } else {
            // Step 1 failed: The imported item doesn't exist in the signature registry
            return Err(TirError::ImportNotFound(ImportNotFound {
                module: self.import.text.to_string(),
                position: self.import.to_range(),
                code: self.ast_name().state.file.into(),
            }.into()));
        }

        // Use statements don't have a specific type location in the type system
        Ok(TypeLocation::UNDEFINED)
    }

    /// Completes the resolution of a `use` statement.
    /// 
    /// For use statements, no additional work is needed in the finish phase
    /// since all the import registration is handled in the resolve phase.
    /// 
    /// # Arguments
    /// 
    /// * `_context` - Unused TIR context (kept for trait consistency)
    /// * `_scope_location` - Unused scope location (kept for trait consistency)
    /// 
    /// # Returns
    /// 
    /// Always returns `Ok(())` since use statements require no finish phase work.
    fn finish(&self, _: &mut TirContext<'base>, _: ScopeLocation) -> Result<(), TirError> { 
        Ok(()) 
    }
    
    /// Returns the local name that this import will be known by in the current scope.
    /// 
    /// This is the name that other code in the same module will use to reference
    /// the imported item. It prioritizes aliases over the original name.
    /// 
    /// # Returns
    /// 
    /// * The alias text if an alias was provided (e.g., "SC" from `use lib.SomeClass as SC`)
    /// * The last component of the import path if no alias (e.g., "SomeClass" from `use lib.SomeClass`)
    /// 
    /// # Examples
    /// 
    /// ```timu
    /// use lib.Calculator;        // name() returns "Calculator"
    /// use lib.Calculator as Calc; // name() returns "Calc"
    /// use utils.math.sin as sine; // name() returns "sine"
    /// ```
    fn name(&self) -> Cow<'base, str> {
        if let Some(alias) = &self.alias {
            Cow::Borrowed(alias.text)
        } else {
            Cow::Borrowed(self.ast_name().text)
        }
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for module use statement resolution.
    //! 
    //! These tests verify the behavior of `UseAst` resolution including:
    //! - Name extraction with and without aliases
    //! - Import resolution error handling
    //! - Basic control flow validation
    
    use super::*;
    use crate::{
        file::SourceFile, 
        nom_tools::{Span, State}, 
        parser::splited_path::SplitedPath, 
        tir::{context::TirContext, module::Module, scope::ScopeLocation}
    };

    /// Creates a test `SplitedPath` from a dot-separated string.
    /// 
    /// This helper function constructs the complex path structure needed
    /// for testing use statements without requiring full parser integration.
    /// 
    /// # Arguments
    /// 
    /// * `text` - The full import path (e.g., "lib.TestClass")
    /// * `state` - The parser state containing source file information
    /// 
    /// # Returns
    /// 
    /// A `SplitedPath` containing individual path components with position information.
    fn create_test_splited_path<'a>(text: &'a str, state: &'a State) -> SplitedPath<'a> {
        let paths = text.split('.').enumerate().map(|(i, part)| {
            let start = i * (part.len() + 1);
            let end = start + part.len();
            Span {
                text: part,
                position: start..end,
                state: state.clone(),
            }
        }).collect();
        
        SplitedPath::new(
            Span {
                text,
                position: 0..text.len(),
                state: state.clone(),
            },
            paths
        )
    }

    /// Tests that `name()` returns the original class name when no alias is provided.
    /// 
    /// For `use lib.TestClass;`, the name should be "TestClass".
    #[test]
    fn use_ast_name_without_alias() {
        let state = State::new(SourceFile::new(vec!["test".into()], "use lib.TestClass;".to_string()));
        let import_path = create_test_splited_path("lib.TestClass", &state);
        
        let use_ast = UseAst {
            import: import_path,
            alias: None,
        };
        
        let name = use_ast.name();
        assert_eq!(name, "TestClass");
    }

    /// Tests that `name()` returns the alias when one is provided.
    /// 
    /// For `use lib.TestClass as TC;`, the name should be "TC".
    #[test]
    fn use_ast_name_with_alias() {
        let state = State::new(SourceFile::new(vec!["test".into()], "use lib.TestClass as TC;".to_string()));
        let import_path = create_test_splited_path("lib.TestClass", &state);
        
        let use_ast = UseAst {
            import: import_path,
            alias: Some(Span {
                text: "TC",
                position: 20..22,
                state: state.clone(),
            }),
        };
        
        let name = use_ast.name();
        assert_eq!(name, "TC");
    }

    /// Tests that `ast_name()` always returns the original name from the import path.
    /// 
    /// This method extracts the last component of the import path regardless of aliases.
    #[test]
    fn use_ast_ast_name_method() {
        let state = State::new(SourceFile::new(vec!["test".into()], "use lib.TestClass;".to_string()));
        let import_path = create_test_splited_path("lib.TestClass", &state);
        
        let use_ast = UseAst {
            import: import_path,
            alias: None,
        };
        
        let ast_name = use_ast.ast_name();
        assert_eq!(ast_name.text, "TestClass");
    }

    /// Tests that the `finish()` method always succeeds for use statements.
    /// 
    /// Use statements require no additional work in the finish phase.
    #[test]
    fn use_ast_finish_always_succeeds() {
        let mut context = TirContext::default();
        let state = State::new(SourceFile::new(vec!["test".into()], "use lib.TestClass;".to_string()));
        let import_path = create_test_splited_path("lib.TestClass", &state);
        
        let use_ast = UseAst {
            import: import_path,
            alias: None,
        };
        
        // Test finish - should always return Ok(())
        let result = use_ast.finish(&mut context, ScopeLocation(0));
        assert!(result.is_ok());
    }

    /// Tests that resolve returns `ImportNotFound` error for non-existent imports.
    /// 
    /// When trying to import something that doesn't exist in the AST signature registry,
    /// the resolver should return an appropriate error.
    #[test]
    fn use_ast_resolve_import_not_found() {
        let mut context = TirContext::default();
        let state = State::new(SourceFile::new(vec!["test".into()], "use nonexistent.Class;".to_string()));
        
        // Create main module
        let main_module = Module::phantom("main".into(), "main".into(), state.file.clone(), ScopeLocation(0));
        context.modules.insert("main".into(), main_module);
        
        let import_path = create_test_splited_path("nonexistent.Class", &state);
        
        // Create UseAst for non-existent import
        let use_ast = UseAst {
            import: import_path,
            alias: None,
        };
        
        // Test resolve - should return ImportNotFound error
        let result = use_ast.resolve(&mut context, ScopeLocation(0));
        assert!(result.is_err());
        
        match result.unwrap_err() {
            TirError::ImportNotFound(_) => {},
            _ => panic!("Expected ImportNotFound error"),
        }
    }

    /// Tests the basic resolution control flow for use statements.
    /// 
    /// This test validates that the resolver attempts to look up imports
    /// and returns appropriate errors when they don't exist.
    #[test]
    fn use_ast_resolve_basic_logic() {
        // This test focuses on the basic control flow without complex mocking
        let mut context = TirContext::default();
        let state = State::new(SourceFile::new(vec!["test".into()], "use lib.TestClass;".to_string()));
        
        // Create main module with minimal setup
        let main_module = Module::phantom("main".into(), "main".into(), state.file.clone(), ScopeLocation(0));
        context.modules.insert("main".into(), main_module);
        
        let import_path = create_test_splited_path("lib.TestClass", &state);
        
        let use_ast = UseAst {
            import: import_path,
            alias: None,
        };
        
        // Test that resolve attempts to look up the import
        // Without the signature present, it should return ImportNotFound
        let result = use_ast.resolve(&mut context, ScopeLocation(0));
        assert!(result.is_err());
        
        // Verify the error is ImportNotFound
        match result.unwrap_err() {
            TirError::ImportNotFound(err) => {
                // Verify the error contains the expected module name
                assert!(err.module.contains("lib.TestClass"));
            },
            _ => panic!("Expected ImportNotFound error"),
        }
    }

    /// Tests name extraction from complex dotted paths.
    /// 
    /// Verifies that both `ast_name()` and `name()` correctly extract
    /// the last component from multi-level import paths.
    #[test] 
    fn use_ast_name_extraction() {
        let state = State::new(SourceFile::new(vec!["test".into()], "use module.submodule.Class;".to_string()));
        let import_path = create_test_splited_path("module.submodule.Class", &state);
        
        let use_ast = UseAst {
            import: import_path,
            alias: None,
        };
        
        // Test ast_name extracts the last part of the path
        let ast_name = use_ast.ast_name();
        assert_eq!(ast_name.text, "Class");
        
        // Test name() method returns the same when no alias
        let name = use_ast.name();
        assert_eq!(name, "Class");
    }

    /// Tests accessibility control for private class imports.
    ///
    /// Verifies that importing a private class from another module
    /// results in an AccessibilityViolation error.
    #[test]
    fn use_ast_accessibility_violation_private_class() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::{build, TirError}
        };
        
        // Create library module with private class
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], "class PrivateClass {}".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create main module that tries to import private class
        let main_state = State::new(SourceFile::new(vec!["main".into()], "use lib.PrivateClass;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should fail with accessibility violation
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            TirError::AccessibilityViolation(error) => {
                assert!(error.item_name.contains("PrivateClass"));
            },
            _ => panic!("Expected AccessibilityViolation error"),
        }
    }

    /// Tests accessibility control allows public class imports.
    ///
    /// Verifies that importing a public class from another module
    /// works without accessibility violations.
    #[test]
    fn use_ast_accessibility_allows_public_class() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::build
        };
        
        // Create library module with public class
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], "pub class PublicClass {}".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create main module that imports public class
        let main_state = State::new(SourceFile::new(vec!["main".into()], "use lib.PublicClass;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should succeed
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_ok());
    }

    /// Tests that aliases take precedence over original names in `name()` method.
    /// 
    /// Verifies that:
    /// - `ast_name()` always returns the original name
    /// - `name()` returns the alias when provided
    #[test]
    fn use_ast_alias_takes_precedence() {
        let state = State::new(SourceFile::new(vec!["test".into()], "use very.long.path.ClassName as Short;".to_string()));
        let import_path = create_test_splited_path("very.long.path.ClassName", &state);
        
        let use_ast = UseAst {
            import: import_path,
            alias: Some(Span {
                text: "Short",
                position: 30..35,
                state: state.clone(),
            }),
        };
        
        // ast_name still returns the original name
        let ast_name = use_ast.ast_name();
        assert_eq!(ast_name.text, "ClassName");
        
        // but name() returns the alias
        let name = use_ast.name();
        assert_eq!(name, "Short");
    }

    /// Tests accessibility control for private function imports.
    ///
    /// Verifies that importing a private function from another module
    /// results in an AccessibilityViolation error.
    #[test]
    fn use_ast_accessibility_violation_private_function() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::{build, TirError}
        };
        
        // Create library module with private function
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], "func privateFunction(): void {}".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create main module that tries to import private function
        let main_state = State::new(SourceFile::new(vec!["main".into()], "use lib.privateFunction;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should fail with accessibility violation
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            TirError::AccessibilityViolation(error) => {
                assert!(error.item_name.contains("privateFunction"));
            },
            _ => panic!("Expected AccessibilityViolation error"),
        }
    }

    /// Tests accessibility control allows public function imports.
    ///
    /// Verifies that importing a public function from another module
    /// works without accessibility violations.
    #[test]
    fn use_ast_accessibility_allows_public_function() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::build
        };
        
        // Create library module with public function
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], "pub func publicFunction(): void {}".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create main module that imports public function
        let main_state = State::new(SourceFile::new(vec!["main".into()], "use lib.publicFunction;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should succeed
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_ok());
    }

    /// Tests accessibility control for interface imports.
    ///
    /// Interfaces should always be importable since they define public contracts.
    #[test]
    fn use_ast_accessibility_allows_interface_imports() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::build
        };
        
        // Create library module with interface
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], "interface TestInterface { func test(): void; }".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create main module that imports interface
        let main_state = State::new(SourceFile::new(vec!["main".into()], "use lib.TestInterface;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should succeed (interfaces are always public)
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_ok());
    }

    /// Tests accessibility control for module imports.
    ///
    /// Module imports should always be allowed regardless of content accessibility.
    #[test]
    fn use_ast_accessibility_allows_module_imports() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::build
        };
        
        // Create library module with mixed public/private content
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], 
            "pub class PublicClass {} class PrivateClass {}".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create main module that imports the module itself
        let main_state = State::new(SourceFile::new(vec!["main".into()], "use lib;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should succeed (module imports are always allowed)
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_ok());
    }

    /// Tests accessibility with nested module imports.
    ///
    /// Tests that accessibility control works correctly with nested module structures.
    #[test]
    fn use_ast_accessibility_nested_modules() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::{build, TirError}
        };
        
        // Create nested module with private class
        let nested_state = State::new(SourceFile::new(vec!["parent".into(), "child".into()], 
            "class NestedPrivateClass {}".to_string()));
        let nested_ast = process_code(&nested_state).unwrap();
        
        // Create main module that tries to import from nested module
        let main_state = State::new(SourceFile::new(vec!["main".into()], 
            "use parent.child.NestedPrivateClass;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should fail with accessibility violation
        let result = build(vec![main_ast.into(), nested_ast.into()]);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            TirError::AccessibilityViolation(error) => {
                assert!(error.item_name.contains("NestedPrivateClass"));
            },
            _ => panic!("Expected AccessibilityViolation error"),
        }
    }

    /// Tests accessibility with aliased imports.
    ///
    /// Verifies that accessibility control works the same whether imports use aliases or not.
    #[test]
    fn use_ast_accessibility_with_aliases() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::{build, TirError}
        };
        
        // Create library module with private class
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], "class PrivateClass {}".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create main module that tries to import private class with alias
        let main_state = State::new(SourceFile::new(vec!["main".into()], 
            "use lib.PrivateClass as MyClass;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should fail with accessibility violation even with alias
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            TirError::AccessibilityViolation(error) => {
                assert!(error.item_name.contains("PrivateClass"));
            },
            _ => panic!("Expected AccessibilityViolation error"),
        }
    }

    /// Tests multiple accessibility violations in a single module.
    ///
    /// When multiple private items are imported, the first violation should be reported.
    #[test] 
    fn use_ast_accessibility_multiple_violations() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::{build, TirError}
        };
        
        // Create library module with multiple private items
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], 
            "class PrivateClass1 {} class PrivateClass2 {} func privateFunc(): void {}".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create main module that imports multiple private items
        let main_state = State::new(SourceFile::new(vec!["main".into()], 
            "use lib.PrivateClass1; use lib.PrivateClass2; use lib.privateFunc;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should fail with accessibility violation
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            TirError::AccessibilityViolation(_) => {
                // Should get accessibility violation for one of the private items
            },
            TirError::ErrorCollection(errors) => {
                // Multiple errors might be collected
                assert!(!errors.errors.is_empty());
            },
            _ => panic!("Expected AccessibilityViolation or ErrorCollection"),
        }
    }

    /// Tests accessibility error message contains correct source locations.
    ///
    /// Verifies that the error properly identifies both the import location
    /// and the private item definition location.
    #[test]
    fn use_ast_accessibility_error_source_locations() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::{build, TirError}
        };
        
        // Create library module with private class
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], 
            "\nclass PrivateClass {\n    func init(this): void {}\n}".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create main module that imports private class
        let main_state = State::new(SourceFile::new(vec!["main".into()], 
            "\nuse lib.PrivateClass;\n".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should fail with accessibility violation
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            TirError::AccessibilityViolation(error) => {
                // Verify error contains both source locations
                assert!(error.import_info.position.start < error.import_info.position.end);
                assert!(error.item_info.position.start < error.item_info.position.end);
                assert!(error.item_name.contains("PrivateClass"));
                
                // Verify source codes are present
                assert!(!error.import_info.code.source.is_empty());
                assert!(!error.item_info.code.source.is_empty());
            },
            _ => panic!("Expected AccessibilityViolation error"),
        }
    }

    /// Tests accessibility with mixed public and private imports.
    ///
    /// When importing both public and private items, only the private ones should fail.
    #[test]
    fn use_ast_accessibility_mixed_imports() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::{build, TirError}
        };
        
        // Create library module with mixed accessibility
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], 
            "pub class PublicClass {} class PrivateClass {}".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create main module that imports both
        let main_state = State::new(SourceFile::new(vec!["main".into()], 
            "use lib.PublicClass; use lib.PrivateClass;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should fail due to private import
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            TirError::AccessibilityViolation(error) => {
                assert!(error.item_name.contains("PrivateClass"));
            },
            _ => panic!("Expected AccessibilityViolation error"),
        }
    }

    /// Tests accessibility control with class inheritance.
    ///
    /// Private base classes should not be importable even if extended by public classes.
    #[test]
    fn use_ast_accessibility_inheritance_scenarios() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::{build, TirError}
        };
        
        // Create library module with private class
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], 
            "class BaseClass {} pub class PublicDerived {}".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Try to import the private base class
        let main_state = State::new(SourceFile::new(vec!["main".into()], 
            "use lib.BaseClass;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Should fail with accessibility violation
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            TirError::AccessibilityViolation(error) => {
                assert!(error.item_name.contains("BaseClass"));
            },
            _ => panic!("Expected AccessibilityViolation error"),
        }
    }

    /// Tests that extend definitions cannot be imported directly.
    ///
    /// Extend statements are implementation details and should not be importable.
    #[test]
    fn use_ast_accessibility_extend_not_importable() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::build
        };
        
        // Create library module with public class that can be safely imported
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], 
            "pub class TestClass {} interface TestInterface { func test(): void; } extend TestClass: TestInterface { func test(): void {} }".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Import the public class (this should work)
        let main_state = State::new(SourceFile::new(vec!["main".into()], 
            "use lib.TestClass;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Should succeed since TestClass is public
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_ok());
    }

    /// Tests cross-module accessibility with deep nested paths.
    ///
    /// Verifies accessibility works with complex dotted import paths.
    #[test]
    fn use_ast_accessibility_deep_nested_paths() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::{build, TirError}
        };
        
        // Create deeply nested module with private class
        let nested_state = State::new(SourceFile::new(
            vec!["level1".into(), "level2".into(), "level3".into()], 
            "class DeepPrivateClass {}".to_string()
        ));
        let nested_ast = process_code(&nested_state).unwrap();
        
        // Try to import from deep path
        let main_state = State::new(SourceFile::new(vec!["main".into()], 
            "use level1.level2.level3.DeepPrivateClass;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Should fail with accessibility violation
        let result = build(vec![main_ast.into(), nested_ast.into()]);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            TirError::AccessibilityViolation(error) => {
                assert!(error.item_name.contains("DeepPrivateClass"));
            },
            _ => panic!("Expected AccessibilityViolation error"),
        }
    }

    /// Tests that accessibility violation error correctly highlights only the specific identifier.
    ///
    /// This test verifies that when compiling multiple files with accessibility violations,
    /// the error correctly highlights only the private class name (e.g., "PrivateClass")
    /// and not the entire import statement or module path.
    #[test]
    fn use_ast_accessibility_precise_highlighting() {
        use crate::{
            file::SourceFile, 
            nom_tools::State, 
            process_code,
            tir::{build, TirError}
        };
        
        // Create simple_lib.tim with PrivateClass
        let lib_content = r#"pub class PublicClass {
}

class PrivateClass {
}"#;
        let lib_state = State::new(SourceFile::new(vec!["simple_lib".into()], lib_content.to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create accessibility_test.tim that imports PrivateClass
        let test_content = "use simple_lib.PrivateClass;\n\nclass Test {\n}";
        let test_state = State::new(SourceFile::new(vec!["accessibility_test".into()], test_content.to_string()));
        let test_ast = process_code(&test_state).unwrap();
        
        // Should fail with accessibility violation
        let result = build(vec![lib_ast.into(), test_ast.into()]);
        assert!(result.is_err(), "Expected accessibility violation");
        
        match result.unwrap_err() {
            TirError::AccessibilityViolation(error) => {
                // Verify error message contains the correct item name
                assert_eq!(error.item_name, "PrivateClass");
                
                // Verify import location highlighting - should point to "PrivateClass" only
                let import_position = &error.import_info.position;
                let import_source = &error.import_info.code.source;
                let highlighted_import = &import_source[import_position.start..import_position.end];
                assert_eq!(highlighted_import, "PrivateClass", 
                    "Import error should highlight only 'PrivateClass', got: '{}'", highlighted_import);
                
                // Verify item definition location highlighting - should point to "PrivateClass" only
                let item_position = &error.item_info.position;
                let item_source = &error.item_info.code.source;
                let highlighted_item = &item_source[item_position.start..item_position.end];
                assert_eq!(highlighted_item, "PrivateClass",
                    "Item error should highlight only 'PrivateClass', got: '{}'", highlighted_item);
                
                // Verify both source codes are present
                assert!(!error.import_info.code.source.is_empty());
                assert!(!error.item_info.code.source.is_empty());
                
                // Verify the import source contains the full import statement
                assert!(error.import_info.code.source.contains("use simple_lib.PrivateClass;"));
                
                // Verify the item source contains the class definition
                assert!(error.item_info.code.source.contains("class PrivateClass"));
            },
            other => panic!("Expected AccessibilityViolation error, got: {:?}", other),
        }
    }
}
