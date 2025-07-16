//! Qualified path handling for module and type references in Timu.
//!
//! This module provides utilities for parsing and managing qualified paths such as
//! `module.submodule.Type` or `std.collections.HashMap`. It handles the splitting
//! of dot-separated identifiers while preserving source location information for
//! each component.
//!
//! # Path Representation
//!
//! Qualified paths in Timu are represented in multiple forms:
//! - **Raw path**: The original text with potential whitespace
//! - **Split components**: Individual identifiers as separate spans
//! - **Normalized text**: Clean dot-separated representation
//!
//! # Usage
//!
//! This is primarily used for:
//! - Module import paths: `use std.collections.HashMap`
//! - Type references: `std.Option<T>`
//! - Qualified function calls: `math.sqrt(16.0)`

use std::{borrow::Cow, ops::Range};

use crate::nom_tools::{NomSpan, Span};

/// Enumeration for different path representations during parsing
/// 
/// This enum is used internally during parsing to distinguish between
/// simple single-component paths and complex multi-component paths.
#[allow(dead_code)]
pub enum SplitedPathEnum<'base> {
    /// A simple path with a single component
    Path(NomSpan<'base>),
    /// A qualified path with multiple dot-separated components
    Splited(Vec<NomSpan<'base>>),
}

/// A qualified path split into individual components with source location tracking
/// 
/// This struct represents a dot-separated path (like `module.Type`) where each
/// component maintains its original source location for error reporting while
/// also providing a clean textual representation.
#[derive(Debug)]
pub struct SplitedPath<'base> {
    /// Individual path components with their source locations
    pub paths: Vec<Span<'base>>,
    /// Clean textual representation of the full path
    pub text: Cow<'base, str>,
}

impl<'base> SplitedPath<'base> {
    /// Creates a new split path from a full path span and its components
    /// 
    /// This constructor handles the creation of a clean textual representation
    /// by checking if the original path contains whitespace. If it does, it
    /// reconstructs a clean version by joining the individual components with dots.
    /// 
    /// # Arguments
    /// * `full_path` - The original full path span as parsed from source
    /// * `paths` - Vector of individual path components
    /// 
    /// # Examples
    /// - Input: `"module . Type"` → Output: `"module.Type"`
    /// - Input: `"simple"` → Output: `"simple"` (borrowed)
    pub fn new(full_path: Span<'base>, paths: Vec<Span<'base>>) -> Self {
        let text = match full_path.text.contains(char::is_whitespace) {
            true => {
                let path = paths.iter().map(|path| path.text)
                .collect::<Vec<&str>>()
                .join(".");
                Cow::Owned(path)
            }
            false => Cow::Borrowed(full_path.text)
        };

        Self { paths, text }
    }

    /// Returns the source range that encompasses the entire qualified path
    /// 
    /// Calculates the range from the start of the first component to the end
    /// of the last component, providing the full span for error reporting.
    pub fn to_range(&self) -> Range<usize> {
        let start = self.paths.first().map_or(0, |path| path.position.start);
        let end = self.paths.last().map_or(0, |path| path.position.end);
        start..end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{file::SourceFile, nom_tools::State};

    /// Helper function to create spans for testing
    fn create_test_spans<'a>(full_text: &'a str, components: &[(usize, usize)]) -> (Span<'a>, Vec<Span<'a>>) {
        let source_file = SourceFile::new(vec!["test".into()], full_text.to_string());
        let state = State::new(source_file);
        
        let full_span = Span {
            text: full_text,
            position: 0..full_text.len(),
            state,
        };
        
        // Create component spans by cloning the state
        let component_spans: Vec<Span> = components.iter().map(|(start, end)| {
            let component_source = SourceFile::new(vec!["test".into()], full_text.to_string());
            let component_state = State::new(component_source);
            Span {
                text: &full_text[*start..*end],
                position: *start..*end,
                state: component_state,
            }
        }).collect();
        
        (full_span, component_spans)
    }

    #[test]
    fn test_splited_path_new_simple_path() {
        let (full_span, components) = create_test_spans("SimpleClass", &[(0, 11)]);
        let splited_path = SplitedPath::new(full_span, components);
        
        // Simple path should be borrowed, not owned
        assert!(matches!(splited_path.text, Cow::Borrowed(_)));
        assert_eq!(splited_path.text, "SimpleClass");
        assert_eq!(splited_path.paths.len(), 1);
        assert_eq!(splited_path.paths[0].text, "SimpleClass");
    }

    #[test]
    fn test_splited_path_new_qualified_path_no_whitespace() {
        let (full_span, components) = create_test_spans("module.Type", &[(0, 6), (7, 11)]);
        let splited_path = SplitedPath::new(full_span, components);
        
        // No whitespace, should be borrowed
        assert!(matches!(splited_path.text, Cow::Borrowed(_)));
        assert_eq!(splited_path.text, "module.Type");
        assert_eq!(splited_path.paths.len(), 2);
        assert_eq!(splited_path.paths[0].text, "module");
        assert_eq!(splited_path.paths[1].text, "Type");
    }

    #[test]
    fn test_splited_path_new_qualified_path_with_whitespace() {
        let (full_span, components) = create_test_spans("module . Type", &[(0, 6), (9, 13)]);
        let splited_path = SplitedPath::new(full_span, components);
        
        // Contains whitespace, should be owned and normalized
        assert!(matches!(splited_path.text, Cow::Owned(_)));
        assert_eq!(splited_path.text, "module.Type");
        assert_eq!(splited_path.paths.len(), 2);
        assert_eq!(splited_path.paths[0].text, "module");
        assert_eq!(splited_path.paths[1].text, "Type");
    }

    #[test]
    fn test_splited_path_new_deep_qualified_path() {
        let (full_span, components) = create_test_spans("std.collections.HashMap", &[(0, 3), (4, 15), (16, 23)]);
        let splited_path = SplitedPath::new(full_span, components);
        
        assert!(matches!(splited_path.text, Cow::Borrowed(_)));
        assert_eq!(splited_path.text, "std.collections.HashMap");
        assert_eq!(splited_path.paths.len(), 3);
        assert_eq!(splited_path.paths[0].text, "std");
        assert_eq!(splited_path.paths[1].text, "collections");
        assert_eq!(splited_path.paths[2].text, "HashMap");
    }

    #[test]
    fn test_splited_path_new_deep_qualified_path_with_whitespace() {
        let (full_span, components) = create_test_spans("std . collections . HashMap", &[(0, 3), (6, 17), (20, 27)]);
        let splited_path = SplitedPath::new(full_span, components);
        
        // Contains whitespace, should be normalized
        assert!(matches!(splited_path.text, Cow::Owned(_)));
        assert_eq!(splited_path.text, "std.collections.HashMap");
        assert_eq!(splited_path.paths.len(), 3);
        assert_eq!(splited_path.paths[0].text, "std");
        assert_eq!(splited_path.paths[1].text, "collections");
        assert_eq!(splited_path.paths[2].text, "HashMap");
    }

    #[test]
    fn test_splited_path_to_range_single_component() {
        let (full_span, components) = create_test_spans("SimpleClass", &[(0, 11)]);
        let splited_path = SplitedPath::new(full_span, components);
        let range = splited_path.to_range();
        
        assert_eq!(range, 0..11);
    }

    #[test]
    fn test_splited_path_to_range_multiple_components() {
        let (full_span, components) = create_test_spans("module.Type.Inner", &[(0, 6), (7, 11), (12, 17)]);
        let splited_path = SplitedPath::new(full_span, components);
        let range = splited_path.to_range();
        
        // Should span from start of first component to end of last component
        assert_eq!(range, 0..17);
    }

    #[test]
    fn test_splited_path_to_range_empty_components() {
        let (full_span, components) = create_test_spans("", &[]);
        let splited_path = SplitedPath::new(full_span, components);
        let range = splited_path.to_range();
        
        // Empty components should return 0..0
        assert_eq!(range, 0..0);
    }

    #[test]
    fn test_splited_path_whitespace_detection() {
        // Test various whitespace scenarios
        let test_cases = &[
            ("module.Type", false),      // No whitespace
            ("module . Type", true),     // Space around dot
            ("module\t.Type", true),     // Tab before dot
            ("module.\tType", true),     // Tab after dot
            ("module\n.Type", true),     // Newline before dot
            ("module.\nType", true),     // Newline after dot
            ("module  .  Type", true),   // Multiple spaces
        ];

        for &(input, should_be_owned) in test_cases {
            let (full_span, components) = create_test_spans(input, &[(0, 6), (input.len() - 4, input.len())]);
            let splited_path = SplitedPath::new(full_span, components);
            
            match should_be_owned {
                true => {
                    assert!(matches!(splited_path.text, Cow::Owned(_)), 
                        "Expected owned for input: '{}'", input);
                    assert_eq!(splited_path.text, "module.Type",
                        "Normalization failed for input: '{}'", input);
                },
                false => {
                    assert!(matches!(splited_path.text, Cow::Borrowed(_)),
                        "Expected borrowed for input: '{}'", input);
                    assert_eq!(splited_path.text, input,
                        "Text should match input for: '{}'", input);
                }
            }
        }
    }

    #[test]
    fn test_splited_path_debug_implementation() {
        let (full_span, components) = create_test_spans("test.path", &[(0, 4), (5, 9)]);
        let splited_path = SplitedPath::new(full_span, components);
        let debug_str = format!("{:?}", splited_path);
        
        // Verify debug output contains the structure
        assert!(debug_str.contains("SplitedPath"));
        assert!(debug_str.contains("paths"));
        assert!(debug_str.contains("text"));
    }
}