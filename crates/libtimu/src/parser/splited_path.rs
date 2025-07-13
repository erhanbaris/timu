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
        let end = self.paths.last().map_or(0, |path| path.position.end + path.text.len());
        start..end
    }
}