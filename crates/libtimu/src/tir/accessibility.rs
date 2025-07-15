//! Accessibility control for the Timu language compiler.
//! 
//! This module provides utilities for checking and enforcing accessibility rules
//! when importing items across module boundaries. It ensures that private items
//! cannot be accessed from outside their defining module, while allowing public
//! items to be imported freely.
//!
//! # Overview
//!
//! The Timu language uses explicit accessibility modifiers to control which items
//! can be imported from other modules:
//!
//! - **Private items** (default): Only accessible within the same module
//! - **Public items** (`pub` keyword): Can be imported from other modules
//! - **Interfaces**: Always public (they define contracts)
//! - **Modules**: Always accessible for import
//! - **Extensions**: Cannot be imported directly
//!
//! # Usage
//!
//! ```ignore
//! use crate::tir::accessibility::check_import_accessibility;
//! 
//! // Check if an AST signature can be imported
//! let result = check_import_accessibility(
//!     &ast_signature,
//!     &import_span,
//!     import_source_file
//! );
//! 
//! match result {
//!     Ok(()) => {
//!         // Import is allowed
//!     },
//!     Err(accessibility_error) => {
//!         // Import violates accessibility rules
//!     }
//! }
//! ```

use std::ops::Range;

use crate::{
    file::SourceFile, 
    nom_tools::{Span, ToRange}, 
    tir::{
        ast_signature::AstSignatureValue, 
        signature::Signature, 
        TirError
    }
};

use super::module::ModuleRef;

/// Checks if an AST signature item is accessible for import from another module.
///
/// This function enforces Timu's accessibility rules by checking whether an item
/// marked with specific visibility modifiers can be imported from outside its
/// defining module. The rules are:
///
/// - **Classes**: Must have `pub` modifier to be importable
/// - **Functions**: Must have `pub` modifier to be importable  
/// - **Interfaces**: Always importable (public contracts)
/// - **Modules**: Always importable
/// - **Extensions**: Never directly importable
///
/// # Arguments
///
/// * `ast_signature` - The signature of the item being imported
/// * `import_span` - The source span of the import statement
/// * `import_source` - The source file containing the import statement
///
/// # Returns
///
/// * `Ok(())` - The import is allowed
/// * `Err(TirError::AccessibilityViolation)` - The import violates accessibility rules
///
/// # Examples
///
/// ```ignore
/// // Check if a class can be imported
/// let result = check_import_accessibility(
///     &class_signature,
///     &import_span, 
///     import_file.clone()
/// );
/// ```
pub fn check_import_accessibility<'base>(
    ast_signature: &Signature<AstSignatureValue<'base>, ModuleRef<'base>>,
    import_span: &Span<'base>,
    import_source: SourceFile
) -> Result<(), TirError> {
    // Determine if the imported item is accessible from another module
    let is_accessible = match ast_signature.value.as_ref() {
        AstSignatureValue::Module(_) => {
            // Modules are always accessible for import
            true
        },
        AstSignatureValue::Class(class_definition_ast) => {
            // Classes are accessible if they are marked as public
            class_definition_ast.is_public.is_some()
        },
        AstSignatureValue::Function(function_definition_ast) => {
            // Functions are accessible if they are marked as public
            function_definition_ast.is_public.is_some()
        },
        AstSignatureValue::Interface(_) => {
            // Interfaces are always accessible for import (they define public contracts)
            true
        },
        AstSignatureValue::Extend(_) => {
            // Extensions cannot be imported directly
            false
        },
    };
    
    // If the item is not accessible, return an accessibility violation error
    if !is_accessible {
        // Get the location of the actual private item for better error reporting
        let (item_position, item_source) = get_item_location_info(ast_signature.value.as_ref());
        
        return Err(TirError::accessibility_violation(
            import_span.text.to_string(),
            import_span.to_range(),
            import_source,
            item_position,
            item_source
        ));
    }

    Ok(())
}

/// Extracts location information from an AST signature value for error reporting.
///
/// This helper function determines the source location and file information for
/// different types of AST signature values. This information is used to provide
/// precise error messages that point to both the import location and the private
/// item definition.
///
/// # Arguments
///
/// * `ast_value` - The AST signature value to extract location information from
///
/// # Returns
///
/// A tuple containing:
/// * `Range<usize>` - The source position range of the item definition
/// * `SourceFile` - The source file containing the item definition
fn get_item_location_info<'base>(ast_value: &AstSignatureValue<'base>) -> (Range<usize>, SourceFile) {
    match ast_value {
        AstSignatureValue::Class(class_definition_ast) => {
            (class_definition_ast.name.to_range(), class_definition_ast.name.state.file.clone())
        },
        AstSignatureValue::Function(function_definition_ast) => {
            (function_definition_ast.name.to_range(), function_definition_ast.name.state.file.clone())
        },
        AstSignatureValue::Interface(interface_definition_ast) => {
            (interface_definition_ast.name.to_range(), interface_definition_ast.name.state.file.clone())
        },
        AstSignatureValue::Module(_) => {
            // This shouldn't happen as modules are always accessible, but provide fallback
            // Note: We don't have access to the original import span here, so we use 0..0
            (0..0, SourceFile::new(vec!["<unknown>".into()], "<module>".to_string()))
        },
        AstSignatureValue::Extend(extend_definition_ast) => {
            (extend_definition_ast.name.to_range(), extend_definition_ast.name.names.last().unwrap().state.file.clone())
        },
    }
}

/// Checks if an individual AST definition has public accessibility.
///
/// This is a convenience function for checking the accessibility of specific
/// AST definition types without needing to wrap them in a signature.
///
/// # Arguments
///
/// * `definition` - The AST definition to check
///
/// # Returns
///
/// `true` if the definition is public and can be imported, `false` otherwise
pub fn is_definition_public<'base>(definition: &AstSignatureValue<'base>) -> bool {
    match definition {
        AstSignatureValue::Module(_) => true,
        AstSignatureValue::Class(class) => class.is_public.is_some(),
        AstSignatureValue::Function(function) => function.is_public.is_some(), 
        AstSignatureValue::Interface(_) => true,
        AstSignatureValue::Extend(_) => false,
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for accessibility checking functionality.
    //!
    //! The detailed accessibility tests are performed in the integration tests
    //! within the module_use resolver, since they require full AST construction
    //! and compilation context. These tests focus on the core logic.

    use crate::{
        file::SourceFile,
        nom_tools::State,
        process_code,
        tir::build
    };

    /// Integration test: Verify that the accessibility module correctly identifies
    /// accessibility violations when integrated with the full compiler pipeline.
    #[test]
    fn test_accessibility_integration_private_class() {
        // Create library module with private class
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], 
            "class PrivateClass {}".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create main module that tries to import private class
        let main_state = State::new(SourceFile::new(vec!["main".into()], 
            "use lib.PrivateClass;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should fail with accessibility violation
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_err());
    }

    /// Integration test: Verify that public classes can be imported successfully.
    #[test]
    fn test_accessibility_integration_public_class() {
        // Create library module with public class
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], 
            "pub class PublicClass {}".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create main module that imports public class
        let main_state = State::new(SourceFile::new(vec!["main".into()], 
            "use lib.PublicClass;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should succeed
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_ok());
    }

    /// Integration test: Verify that interfaces are always accessible.
    #[test]
    fn test_accessibility_integration_interface() {
        // Create library module with interface
        let lib_state = State::new(SourceFile::new(vec!["lib".into()], 
            "interface TestInterface { func test(): void; }".to_string()));
        let lib_ast = process_code(&lib_state).unwrap();
        
        // Create main module that imports interface
        let main_state = State::new(SourceFile::new(vec!["main".into()], 
            "use lib.TestInterface;".to_string()));
        let main_ast = process_code(&main_state).unwrap();
        
        // Try to build - should succeed
        let result = build(vec![main_ast.into(), lib_ast.into()]);
        assert!(result.is_ok());
    }
}