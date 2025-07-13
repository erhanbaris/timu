//! Core types and traits for Timu language compiler macros.
//!
//! This crate provides the fundamental types and traits used by the Timu
//! procedural macros for error handling and source code representation.

use std::fmt::{Display, Formatter};

pub mod traits;

/// Represents source code with its content and associated name/path
/// 
/// This struct is used throughout the error reporting system to provide
/// context about where errors occurred, including the source file name
/// and the actual source code content.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceCode {
    /// The actual source code content
    pub source: String,
    /// The name or path of the source file
    pub name: String,
}

impl Display for SourceCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}