//! Procedural macros for the Timu language compiler.
//!
//! This crate provides derive macros for error handling in the Timu compiler,
//! specifically the `TimuError` derive macro that automatically implements
//! the `TimuErrorTrait` for error types with rich diagnostic information.

use proc_macro::TokenStream;
use crate::error::timu_error;
mod error;

/// Derive macro for implementing `TimuErrorTrait` on error structs
/// 
/// This macro automatically generates implementations that provide rich error
/// diagnostics with source code locations, labels, help text, and error chaining.
/// 
/// # Attributes
/// - `source_code` - Marks fields containing source code information
/// - `label` - Marks fields that provide error labels for specific locations
/// - `help` - Marks fields containing help text for the error
/// - `diagnostic` - Marks fields with diagnostic messages
/// - `errors` - Marks fields containing nested errors
/// - `reference` - Marks fields containing error references
#[proc_macro_derive(TimuError, attributes(source_code, label, help, diagnostic, errors, reference))]
pub fn derive_timu_error(input: TokenStream) -> TokenStream {
    timu_error(input)
}
