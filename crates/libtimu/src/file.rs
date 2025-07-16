//! Source file representation and handling for the Timu language compiler.
//!
//! This module provides the `SourceFile` struct which represents a source file
//! with its path and contents, along with conversions to error reporting types.

use std::{error::Error, path::PathBuf, sync::Arc};

use libtimu_macros_core::SourceCode;

/// Represents a source file with its path and source code content
/// 
/// This structure is used throughout the compiler to track source files
/// and provide error reporting with proper file context.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    /// The file path as a vector of path components
    pub path: Arc<Vec<String>>,
    /// The source code content of the file
    pub code: Arc<String>,
}

impl From<SourceFile> for SourceCode {
    fn from(file: SourceFile) -> Self {
        let pathbuffer = PathBuf::from_iter(file.path.iter());
        let path = pathbuffer.to_string_lossy();
        
        SourceCode {
            source: file.code.to_string(),
            name: path.to_string()   
        }
    }
}

impl From<&SourceFile> for SourceCode {
    fn from(file: &SourceFile) -> Self {
        let pathbuffer = PathBuf::from_iter(file.path.iter());
        let path = pathbuffer.to_string_lossy();
        
        SourceCode {
            source: file.code.to_string(),
            name: path.to_string()   
        }
    }
}

impl Error for SourceFile {}

impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SourceFile: {:?}", self.path)
    }
}

impl SourceFile {
    /// Creates a new source file with the given path and source code
    /// 
    /// # Arguments
    /// * `path` - Vector of path components representing the file path
    /// * `code` - The source code content as a string
    pub fn new(path: Vec<String>, code: String) -> Self {
        Self {
            path: path.into(),
            code: code.into(),
        }
    }

    /// Returns a reference to the file path components
    pub fn path(&self) -> &Vec<String> {
        &self.path
    }

    /// Returns a reference to the source code content
    pub fn code(&self) -> &String {
        self.code.as_ref()
    }
}
