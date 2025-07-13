//! Field declaration parsing for the Timu language.
//!
//! This module handles parsing of field declarations that appear within classes,
//! interfaces, and extensions. Fields represent data members with typed declarations
//! and optional visibility modifiers. The parser handles the various contexts where
//! fields can appear and enforces context-specific rules.
//!
//! # Field Declaration Syntax
//!
//! Fields in Timu follow a consistent syntax pattern across different contexts:
//!
//! ## Basic Field Syntax
//! ```timu
//! fieldName: FieldType;
//! pub publicField: Type;
//! ```
//!
//! ## Class Fields
//! ```timu
//! class MyClass {
//!     privateField: string;
//!     pub publicField: i32;
//!     optionalField: ?string;
//! }
//! ```
//!
//! ## Interface Fields
//! ```timu
//! interface MyInterface {
//!     requiredField: string;
//!     value: i32;
//! }
//! ```
//!
//! ## Extension Fields
//! ```timu
//! extend ExistingType : Interface {
//!     additionalField: string;  // Implicitly public
//!     // pub additionalField: string;  // Error: redundant 'pub'
//! }
//! ```
//!
//! # Field Components
//!
//! ## Visibility Modifiers
//! - **Default (private)**: Fields accessible only within the declaring type
//! - **`pub` (public)**: Fields accessible from outside the declaring type
//! - **Extension fields**: Always implicitly public (explicit `pub` is an error)
//!
//! ## Type Annotations
//! - **Required**: All fields must have explicit type declarations
//! - **Type modifiers**: Supports nullable (`?Type`) and reference (`ref Type`) types
//! - **Qualified types**: Supports module-qualified type names (`module.Type`)
//!
//! ## Field Names
//! - **Identifier rules**: Must follow standard identifier syntax
//! - **Scope**: Field names must be unique within their declaring type
//! - **Access patterns**: Used for dot notation access (`object.field`)
//!
//! # Context-Specific Rules
//!
//! ## Class Context
//! - Fields can be public or private
//! - Support for all type modifiers
//! - Used for object instance data
//!
//! ## Interface Context
//! - Fields define requirements for implementing types
//! - Visibility modifiers are allowed but have different semantics
//! - Part of the interface contract
//!
//! ## Extension Context
//! - Fields are implicitly public (cannot use explicit `pub`)
//! - Add new data members to existing types
//! - Must be provided when implementing the extended interface
//!
//! # Integration with Type System
//!
//! Field declarations integrate with the TIR system for:
//! - **Type resolution**: Field types are resolved during semantic analysis
//! - **Access checking**: Visibility rules are enforced during compilation
//! - **Memory layout**: Field declarations inform object layout decisions
//! - **Interface compliance**: Field requirements are checked during implementation

use std::fmt::{Display, Formatter};

use nom::character::complete::char;
use nom::sequence::terminated;
use nom::{IResult, Parser};
use nom_language::error::{VerboseError, VerboseErrorKind};

use crate::ast::{ClassDefinitionFieldAst, ExtendDefinitionFieldAst, FieldAst, InterfaceDefinitionFieldAst, TypeNameAst};
use crate::nom_tools::{cleanup, NomSpan};

use super::{ident, is_public, TimuParserError};

impl FieldAst<'_> {
    pub fn parse_field(input: NomSpan<'_>) -> IResult<NomSpan<'_>, (Option<NomSpan<'_>>, FieldAst<'_>), TimuParserError<'_>> {
        let (input, (is_public, name, field_type, _)) =
            (is_public, cleanup(terminated(ident(), cleanup(char(':')))), cleanup(TypeNameAst::parse), cleanup(char(';'))).parse(input)?;

        let original_is_public = is_public.clone();
        Ok((
            input,
            (original_is_public, FieldAst {
                is_public: is_public.map(|item| item.into()),
                name: name.into(),
                field_type,
            },
        )))
    }

    pub fn parse_class_field(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ClassDefinitionFieldAst<'_>, TimuParserError<'_>> {
        let (input, (_, field)) = Self::parse_field(input)?;
        Ok((input, ClassDefinitionFieldAst::Field(field)))
    }

    pub fn parse_interface_field(input: NomSpan<'_>) -> IResult<NomSpan<'_>, InterfaceDefinitionFieldAst<'_>, TimuParserError<'_>> {
        let (input, (_, field)) = Self::parse_field(input)?;
        Ok((input, InterfaceDefinitionFieldAst::Field(field)))
    }

    pub fn parse_extend_field(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExtendDefinitionFieldAst<'_>, TimuParserError<'_>> {
        let (input, (is_public, field)) = Self::parse_field(input)?;
        if let Some(is_public) = is_public {
            let error = VerboseError {
                errors: vec![(is_public, VerboseErrorKind::Context("All extended fields already public"))],
            };
            return Err(nom::Err::Failure(error));
        }
        
        Ok((input, ExtendDefinitionFieldAst::Field(field)))
    }
}

impl Display for FieldAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}: {};",
            match self.is_public {
                Some(_) => "pub ",
                None => "",
            },
            self.name.text,
            self.field_type
        )
    }
}
