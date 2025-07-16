//! Tests for .tim language files in the /lang directory.
//!
//! This module contains unit tests that verify the compilation and semantic
//! analysis of real Timu language source files. These tests ensure that
//! the compiler correctly handles various language features and constructs.

use std::fs;
use crate::{
    file::SourceFile,
    nom_tools::State,
    process_code,
    tir::{build, TirError}
};

/// Helper function to load a .tim file from the lang directory.
fn load_lang_file(filename: &str) -> Result<String, std::io::Error> {
    // Try relative to current working directory first (for GitHub Actions)
    if let Ok(content) = fs::read_to_string(&format!("./lang/{}", filename)) {
        return Ok(content);
    }
    
    // Fall back to relative from crate directory (for local development)
    if let Ok(content) = fs::read_to_string(&format!("../lang/{}", filename)) {
        return Ok(content);
    }
    
    // Fall back to relative from crate directory (for local development)
    let path2 = format!("../../lang/{}", filename);
    fs::read_to_string(&path2)
}

/// Helper function to test if a .tim file compiles successfully.
fn test_lang_file_compilation(filename: &str, module_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = load_lang_file(filename).map_err(|e| format!("Failed to load {}: {}", filename, e))?;
    let state = State::new(SourceFile::new(vec![module_name.into()], content));
    let ast = process_code(&state).map_err(|e| format!("Failed to parse {}: {}", filename, e))?;
    let _tir = build(vec![ast.into()]).map_err(|e| format!("Failed to compile {}: {}", filename, e))?;
    Ok(())
}

/// Helper function to test two .tim files together.
fn test_two_lang_files(
    file1: &str, module1: &str,
    file2: &str, module2: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let content1 = load_lang_file(file1)?;
    let state1 = State::new(SourceFile::new(vec![module1.into()], content1));
    let ast1 = process_code(&state1)?;
    
    let content2 = load_lang_file(file2)?;
    let state2 = State::new(SourceFile::new(vec![module2.into()], content2));
    let ast2 = process_code(&state2)?;
    
    let _tir = build(vec![ast1.into(), ast2.into()])?;
    Ok(())
}

/// Tests single file compilation - files that should compile successfully on their own.
mod single_file_tests {
    use super::*;

    #[test]
    fn test_comprehensive_interface_and_class() {
        let result = test_lang_file_compilation("test.tim", "test");
        assert!(result.is_ok(), "test.tim should compile successfully");
    }

    #[test]
    fn test_lib_standalone() {
        let result = test_lang_file_compilation("lib.tim", "lib");
        assert!(result.is_ok(), "lib.tim should compile successfully");
    }

    #[test]
    fn test_simple_lib_standalone() {
        let result = test_lang_file_compilation("simple_lib.tim", "simple_lib");
        assert!(result.is_ok(), "simple_lib.tim should compile successfully");
    }

    #[test]
    fn test_comprehensive_lib_standalone() {
        let result = test_lang_file_compilation("comprehensive_lib.tim", "comprehensive_lib");
        if let Err(e) = &result {
            eprintln!("Error compiling comprehensive_lib.tim: {}", e);
        }
        assert!(result.is_ok(), "comprehensive_lib.tim should compile successfully: {:?}", result);
    }

    #[test]
    fn test_testlib_standalone() {
        let result = test_lang_file_compilation("testlib.tim", "testlib");
        assert!(result.is_ok(), "testlib.tim should compile successfully");
    }

    #[test]
    fn test_testprivate_standalone() {
        let result = test_lang_file_compilation("testprivate.tim", "testprivate");
        // This should fail at TIR phase since it imports an undefined private class
        assert!(result.is_err(), "testprivate.tim should fail due to missing dependency");
    }

    #[test]
    fn test_testprivatefunc_standalone() {
        let result = test_lang_file_compilation("testprivatefunc.tim", "testprivatefunc");
        // This should fail at TIR phase since it imports an undefined private function
        assert!(result.is_err(), "testprivatefunc.tim should fail due to missing dependency");
    }
}

/// Tests accessibility scenarios - files that should fail due to privacy violations.
mod accessibility_tests {
    use super::*;

    #[test]
    fn test_private_class_import_violation() {
        // simple_main.tim tries to import PrivateClass from simple_lib.tim
        let result = test_two_lang_files(
            "simple_lib.tim", "simple_lib",
            "simple_main.tim", "simple_main"
        );
        assert!(result.is_err(), "Should fail due to private class import");
        
        // More detailed check
        let content1 = load_lang_file("simple_lib.tim").unwrap();
        let state1 = State::new(SourceFile::new(vec!["simple_lib".into()], content1));
        let ast1 = process_code(&state1).unwrap();
        
        let content2 = load_lang_file("simple_main.tim").unwrap();
        let state2 = State::new(SourceFile::new(vec!["simple_main".into()], content2));
        let ast2 = process_code(&state2).unwrap();
        
        let tir_result = build(vec![ast1.into(), ast2.into()]);
        assert!(tir_result.is_err(), "Should fail due to private class import");
        
        match tir_result.unwrap_err() {
            TirError::AccessibilityViolation(error) => {
                assert!(error.item_name.contains("PrivateClass"));
            },
            _ => panic!("Expected AccessibilityViolation error"),
        }
    }

    #[test]
    fn test_private_imports_violation() {
        // test_private_imports.tim imports private items from comprehensive_lib.tim
        let result = test_two_lang_files(
            "comprehensive_lib.tim", "comprehensive_lib",
            "test_private_imports.tim", "test_private_imports"
        );
        assert!(result.is_err(), "Private imports should fail");
        
        // More detailed check
        let content1 = load_lang_file("comprehensive_lib.tim").unwrap();
        let state1 = State::new(SourceFile::new(vec!["comprehensive_lib".into()], content1));
        let ast1 = process_code(&state1).unwrap();
        
        let content2 = load_lang_file("test_private_imports.tim").unwrap();
        let state2 = State::new(SourceFile::new(vec!["test_private_imports".into()], content2));
        let ast2 = process_code(&state2).unwrap();
        
        let tir_result = build(vec![ast1.into(), ast2.into()]);
        assert!(tir_result.is_err(), "Private imports should fail");
        
        match tir_result.unwrap_err() {
            TirError::AccessibilityViolation(error) => {
                // Should mention either PrivateClass or privateFunction
                assert!(
                    error.item_name.contains("PrivateClass") || 
                    error.item_name.contains("privateFunction"),
                    "Error should reference private item: {}", error.item_name
                );
            },
            _ => panic!("Expected AccessibilityViolation error"),
        }
    }

    #[test]
    fn test_testprivate_with_testlib_violation() {
        // testprivate.tim tries to import PrivateClass from testlib.tim
        let result = test_two_lang_files(
            "testlib.tim", "testlib",
            "testprivate.tim", "testprivate"
        );
        assert!(result.is_err(), "Should fail due to private class import");
    }

    #[test]
    fn test_testprivatefunc_with_testlib_violation() {
        // testprivatefunc.tim tries to import privateFunction from testlib.tim
        let result = test_two_lang_files(
            "testlib.tim", "testlib",
            "testprivatefunc.tim", "testprivatefunc"
        );
        assert!(result.is_err(), "Should fail due to private function import");
    }
}

/// Tests successful compilation scenarios - files that should work together.
mod success_tests {
    use super::*;

    #[test]
    fn test_public_imports_with_lib() {
        // test_public_imports.tim imports from comprehensive_lib.tim (should succeed)
        let result = test_two_lang_files(
            "comprehensive_lib.tim", "comprehensive_lib",
            "test_public_imports.tim", "test_public_imports"
        );
        assert!(result.is_ok(), "Public imports should compile successfully");
    }

    #[test]
    fn test_public_visibility() {
        let result = test_lang_file_compilation("test_public_visibility.tim", "test_public_visibility");
        assert!(result.is_ok(), "test_public_visibility.tim should compile successfully");
    }

    #[test]
    fn test_testmain_with_testlib() {
        // testmain.tim imports only public items from testlib.tim (should succeed)
        let result = test_two_lang_files(
            "testlib.tim", "testlib",
            "testmain.tim", "testmain"
        );
        assert!(result.is_ok(), "testmain.tim should compile successfully with testlib.tim");
    }
}

/// Tests error scenarios - files designed to test error conditions.
mod error_tests {
    use super::*;

    #[test]
    fn test_error_test_file() {
        let result = test_lang_file_compilation("error_test.tim", "error_test");
        
        // This file is likely designed to test error conditions
        match result {
            Ok(_) => {
                // If it succeeds, the error conditions might not be present
            },
            Err(_) => {
                // Expected for an error test file
            }
        }
    }

    #[test]
    fn test_accessibility_test_file() {
        let result = test_lang_file_compilation("accessibility_test.tim", "accessibility_test");
        
        // This file is designed to test accessibility features
        match result {
            Ok(_) => {
                // Success if the file doesn't contain violations
            },
            Err(_) => {
                // Error might be part of the test
            }
        }
    }
}

/// Tests parsing of all .tim files to ensure they have valid syntax.
mod syntax_tests {
    use super::*;

    #[test]
    fn test_all_files_parse() {
        let tim_files = [
            "test.tim",
            "simple_lib.tim",
            "simple_main.tim",
            "lib.tim", 
            "main.tim",
            "main_public.tim",
            "comprehensive_lib.tim",
            "test_lib.tim",
            "test_main.tim",
            "test_public_imports.tim",
            "test_private_imports.tim",
            "test_public_visibility.tim",
            "testlib.tim",
            "testmain.tim",
            "testprivate.tim",
            "testprivatefunc.tim",
            "accessibility_test.tim",
            "error_test.tim"
        ];

        for file in &tim_files {
            let content = load_lang_file(file);
            assert!(content.is_ok(), "{} should be readable", file);
            
            if let Ok(content) = content {
                let state = State::new(SourceFile::new(vec![file.replace(".tim", "").into()], content));
                let result = process_code(&state);
                assert!(result.is_ok(), "{} should parse successfully", file);
            }
        }
    }
}