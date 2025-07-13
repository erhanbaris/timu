//! Traits and types for rich error handling in the Timu compiler.
//!
//! This module provides the core traits and supporting types that enable
//! the Timu error system to produce detailed diagnostics with source locations,
//! labels, help text, and error chaining.

use std::{fmt::{Debug, Display}, ops::Range};

/// Represents a labeled field with a source position for error reporting
/// 
/// This struct associates a human-readable label with a specific position
/// in the source code, used to highlight relevant parts of code in error messages.
#[derive(Clone, Debug)]
pub struct LabelField {
    /// The source position range where this label applies
    pub position: Range<usize>,
    /// The descriptive label text
    pub label: String,
}

/// A labeled span that combines a descriptive label with a source range
/// 
/// Similar to `LabelField` but with a different field order, used in
/// various parts of the error reporting system.
#[derive(Clone, Debug)]
pub struct LabeledSpan {
    /// The descriptive label text
    pub label: String,
    /// The source position range where this label applies
    pub span: Range<usize>,
}

impl LabeledSpan {
    /// Creates a new labeled span with the given label and source range
    pub fn new(label: String, span: Range<usize>) -> Self {
        Self { label, span }
    }
}

/// Core trait for Timu error types that provides rich diagnostic information
/// 
/// This trait enables errors to provide detailed information including:
/// - Source code labels and positions
/// - Nested errors and references to other errors
/// - Help text and error codes
/// - Source code context
/// 
/// The trait is designed to be implemented via the `TimuError` derive macro.
pub trait TimuErrorTrait: Display {
    /// Returns labeled fields that highlight specific parts of the source code
    fn labels(&self) -> Option<Vec<LabelField>>;
    
    /// Returns an iterator over nested errors contained within this error
    fn errors<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn TimuErrorTrait> + 'a>>;
    
    /// Returns references to other related errors
    fn references(&self) -> Option<Vec<Box<&dyn TimuErrorTrait>>>;
    
    /// Returns the source code context for this error
    fn source_code(&self) -> Option<Box<crate::SourceCode>> { None }
    
    /// Returns an optional error code for this error type
    fn error_code(&self) -> Option<Box<dyn Display>> { None }
    
    /// Returns optional help text to assist in resolving this error
    fn help(&self) -> Option<Box<dyn Display>> { None }
}
