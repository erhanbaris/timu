//! Procedural macro implementation for generating `TimuError` trait implementations.
//!
//! This module provides the core logic for the `#[derive(TimuError)]` macro, which
//! automatically generates implementations of the `TimuErrorTrait` for error types.
//! The macro supports rich error reporting with source code locations, labels, help text,
//! and error chaining.
//!
//! # Supported Attributes
//!
//! The macro recognizes several attributes that control error generation:
//!
//! ## Field Attributes
//! - `#[source_code]` - Marks a field as containing source code information
//! - `#[label("message")]` - Creates a labeled span with the given message
//! - `#[help]` - Marks a field as containing help text for the error
//! - `#[errors]` - Marks a field containing nested errors
//! - `#[reference]` - Marks a field containing error references
//!
//! ## Type Attributes  
//! - `#[diagnostic(code = "E001")]` - Sets an error code for the error type
//! - `#[diagnostic(help = "Try this...")]` - Sets default help text
//! - `#[diagnostic(transparent)]` - Makes the error transparent for chaining
//!
//! # Label Format Strings
//!
//! Label attributes support format string syntax for dynamic message generation:
//! ```ignore
//! #[label("Expected {expected}, found {actual}")]
//! struct TypeError {
//!     #[source_code] source: SourceCode,
//!     position: Range<usize>,
//!     expected: String,
//!     actual: String,
//! }
//! ```
//!
//! # Generated Implementation
//!
//! The macro generates a complete `TimuErrorTrait` implementation including:
//! - `labels()` - Returns labeled spans for error highlighting
//! - `source_code()` - Returns source code context
//! - `help()` - Returns help text if available  
//! - `errors()` - Returns nested errors for chaining
//! - `references()` - Returns error references
//! - `error_code()` - Returns error code if specified

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Variant};

/// Attribute parser for `#[label("message")]` annotations
/// 
/// Extracts label text that will be used to create error spans with
/// descriptive messages for highlighting relevant source code sections.
#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(label))]
struct Label(String);

/// Attribute parser for `#[diagnostic(...)]` annotations
/// 
/// Extracts diagnostic configuration including error codes, help text,
/// and transparency settings for error types.
#[derive(deluxe::ExtractAttributes, deluxe::ParseMetaItem)]
#[deluxe(attributes(diagnostic))]
#[derive(Debug)]
struct Diagnostic {
    /// Optional error code identifier (e.g., "E0001")
    #[deluxe(default)]
    code: Option<String>,

    /// Optional default help text for this error type
    #[deluxe(default)]
    help: Option<String>,

    /// Whether this error should be transparent in error chains
    #[deluxe(default)]
    transparent: bool,
}

/// Finds the field marked with `#[source_code]` attribute
/// 
/// Searches through the struct fields to find the one that contains source code
/// information for error reporting. Returns a `syn::Member` that can be used
/// to generate field access code.
/// 
/// # Arguments
/// * `fields` - The named fields of the struct being processed
/// 
/// # Returns
/// * `Some(Member)` - If a source_code field is found
/// * `None` - If no source_code field exists
fn get_source_code(fields: &mut FieldsNamed) -> Option<syn::Member> {
    for (i, field) in fields.named.iter_mut().enumerate() {
        if field.attrs.iter().any(|attr| attr.path().is_ident("source_code")) {
            let member = match field.ident.clone() {
                Some(ident) => syn::Member::Named(ident),
                None => syn::Member::Unnamed(syn::Index {
                    index: i as u32,
                    span: field.span()
                })
            };

            return Some(member);
        }
    }

    None
}

/// Finds the field marked with `#[help]` attribute and generates help code
/// 
/// Searches for a field containing help text and generates the appropriate
/// token stream for accessing it in the trait implementation. The generated
/// code will clone the field value and box it for the trait return type.
/// 
/// # Arguments  
/// * `fields` - The named fields of the struct being processed
/// 
/// # Returns
/// * `Some(TokenStream)` - Code to access the help field as `Some(Box::new(field.clone()))`
/// * `None` - If no help field exists
fn get_help(fields: &mut FieldsNamed) -> Option<proc_macro2::TokenStream> {
    for field in fields.named.iter_mut() {
        if field.attrs.iter().any(|attr| attr.path().is_ident("help")) {
            let name = &field.ident;
            return Some(quote! { Some(Box::new(self.#name.clone())) });
        }
    }

    None
}

/// Processes fields with `#[label("...")]` attributes to generate label code
/// 
/// This function handles the complex logic for parsing label attributes, including
/// support for format string interpolation. It extracts field references from
/// format strings and generates the appropriate code for dynamic label generation.
/// 
/// # Format String Support
/// 
/// Label messages can include format string syntax like `"Expected {expected}, found {actual}"`
/// which will interpolate field values at runtime to create dynamic error messages.
/// 
/// # Arguments
/// * `fields` - The named fields of the struct being processed
/// 
/// # Returns
/// A vector of tuples containing the field definition and the generated token stream
/// for creating the labeled span.
fn get_labels(fields: &mut FieldsNamed) -> Vec<(Field, proc_macro2::TokenStream)> {
    let mut field_values = Vec::new();
    for field in fields.named.iter_mut() {
        if let Ok(Label(message)) = deluxe::extract_attributes(field) {
            
            match message.contains('{') {
                true => {
                    //println!("contains {{");
                    let mut fields = Vec::new();
                    let mut out = String::new();
                    let mut read = message.as_str();

                    while let Some(start_index) = read.find('{') {
                        //println!("Text : {read}");
                        //println!("start_index: {}", start_index);

                        if let Some(end_index) = read.find('}') {
                            out.push_str(&read[0..start_index]);

                            out.push('{');
                            out.push('}');

                            //println!("end_index: {}", end_index);
                            let field_name = &read[start_index+1..end_index];
                            //println!("Found: {field_name}");
                            let field_name = format_ident!("{}", field_name);
                            fields.push(quote! { self.#field_name.to_string() });

                            read = &read[end_index+1..];
                        } else {
                            panic!("Invalid fmt syntax");
                        }
                    }

                    out.push_str(read);

                    let name = &field.ident;
                    field_values.push((field.clone(), quote! {
                        libtimu_macros_core::traits::LabelField {
                            label: {
                                use std::fmt::Write;
                                let mut s = String::new();
                                write!(&mut s, #out, #(#fields),*).unwrap();
                                s
                            },
                            position: self.#name.clone(),
                        }
                    }));
                    
                },
                false => {
                    let name = &field.ident;
                    field_values.push((field.clone(), quote! {
                        libtimu_macros_core::traits::LabelField {
                            label: #message.to_string(),
                            position: self.#name.clone(),
                        }
                    }));
                }
            };
        }
    }

    field_values
}

/// Finds fields marked with `#[reference]` attribute and generates reference code
/// 
/// Searches through struct fields to find those marked as error references and
/// generates code to box them as trait objects for error chaining. References
/// allow linking related errors together in a hierarchical structure.
/// 
/// # Arguments
/// * `fields` - The named fields of the struct being processed
/// 
/// # Returns
/// A vector of tuples containing the field definition and the generated token stream
/// for creating boxed trait object references.
fn get_references(fields: &mut FieldsNamed) -> Vec<(Field, proc_macro2::TokenStream)> {
    let mut field_values = Vec::new();
    for field in fields.named.iter_mut() {
        if field.attrs.iter().any(|attr| attr.path().is_ident("reference")) {
            let name = &field.ident;
            field_values.push((field.clone(), quote! { Box::new(&self.#name as &dyn libtimu_macros_core::traits::TimuErrorTrait) }));
        }
    }

    field_values
}

/// Finds the field marked with `#[errors]` attribute and generates error iterator code
/// 
/// Searches for a field containing a collection of nested errors and generates
/// code to iterate over them as trait objects. This enables error chaining where
/// one error can contain multiple child errors for comprehensive error reporting.
/// 
/// # Arguments
/// * `fields` - The named fields of the struct being processed
/// 
/// # Returns
/// * `Some(TokenStream)` - Code to create a boxed iterator over the error collection
/// * `None` - If no errors field exists
fn get_errors(fields: &mut FieldsNamed) -> Option<proc_macro2::TokenStream> {
    for field in fields.named.iter_mut() {
        if field.attrs.iter().any(|attr| attr.path().is_ident("errors")) {
            let name = &field.ident;
            return Some(quote! {
                std::boxed::Box::new(self.#name.iter().map(|x| -> &(dyn libtimu_macros_core::traits::TimuErrorTrait) { &*x }))
            });
        }
    }

    None
}

/// Generates `TimuErrorTrait` implementation for struct types
/// 
/// This function processes a struct definition marked with `#[derive(TimuError)]`
/// and generates a complete trait implementation. It analyzes all fields and their
/// attributes to build the appropriate error reporting functionality.
/// 
/// # Generated Methods
/// - `labels()` - Creates labeled spans for error highlighting
/// - `source_code()` - Returns source code context if available
/// - `errors()` - Returns iterator over nested errors  
/// - `error_code()` - Returns error code from diagnostic or None
/// - `help()` - Returns help text from diagnostic or field
/// - `references()` - Returns related error references
/// 
/// # Arguments
/// * `name` - The struct identifier
/// * `diagnostic` - Parsed diagnostic attributes from the struct
/// * `data` - The struct definition data
/// 
/// # Returns
/// A `TokenStream` containing the generated trait implementation
fn build_struct(name: Ident, diagnostic: Diagnostic, mut data: DataStruct) -> TokenStream {
    if let Fields::Named(fields) = &mut data.fields {
        let source_code = match get_source_code(fields) {
            Some(member) => quote!( Some(Box::new(self.#member.clone())) ),
            None => quote!( None ),
        };

        let errors = match get_errors(fields) {
            Some(errors) => quote!( Some(#errors) ),
            None => quote!( None ),
        };

        let error_code = match diagnostic.code {
            Some(code) => quote!( Some(Box::new(#code.to_string())) ),
            None => quote!( None ),
        };

        let help = match diagnostic.help {
            Some(help) => quote!( Some(Box::new(#help.to_string())) ),
            None => match get_help(fields) {
                Some(help) => help,
                None => quote!( None ),
            },
        };

        let labels = get_labels(fields).into_iter().map(|(_, token)| token).collect::<Vec<_>>();
        let reerences = get_references(fields).into_iter().map(|(_, token)| token).collect::<Vec<_>>();

        return TokenStream::from(quote!{
            impl libtimu_macros_core::traits::TimuErrorTrait for #name {
                fn labels(&self) -> Option<Vec<libtimu_macros_core::traits::LabelField>> { Some(vec![#(#labels),*]) }
                fn references<'a>(&'a self) -> Option<Vec<Box<&'a dyn libtimu_macros_core::traits::TimuErrorTrait>>> { Some(vec![#(#reerences),*]) }
                fn errors<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn libtimu_macros_core::traits::TimuErrorTrait> + 'a>> { #errors }
                fn source_code(&self) -> Option<Box<libtimu_macros_core::SourceCode>> { #source_code }
                fn error_code(&self) -> Option<Box<dyn std::fmt::Display>> { #error_code }
                fn help(&self) -> Option<Box<dyn std::fmt::Display>> { #help }
            }
        });
    }

    TokenStream::from(syn::Error::new(name.span(), "Only structs and enums with named fields can derive `TimuError`").to_compile_error())
}

/// Generates source code extraction logic for enum variants
/// 
/// Creates match arm code for extracting source code from a specific enum variant.
/// This is used when building the `source_code()` method implementation for enums.
/// 
/// # Arguments
/// * `enum_name` - The enum type identifier
/// * `enum_field_ident` - The specific variant identifier
/// * `fields` - The named fields of the variant
/// 
/// # Returns
/// A token stream containing the match arm for this variant
fn generate_enum_source_code(enum_name: &Ident, enum_field_ident: &Ident, fields: &mut FieldsNamed) -> proc_macro2::TokenStream {
    let inner_match = match get_source_code(fields) {
        Some(member) => quote!( #member ),
        None => quote!( None )
    };

    quote!( #enum_name::#enum_field_ident { .. } => #inner_match )
}

/// Generates error collection logic for enum variants
/// 
/// Creates match arm code for extracting nested errors from a specific enum variant.
/// This is used when building the `errors()` method implementation for enums.
/// 
/// # Arguments
/// * `enum_name` - The enum type identifier
/// * `enum_field_ident` - The specific variant identifier
/// * `fields` - The named fields of the variant
/// 
/// # Returns
/// A token stream containing the match arm for this variant
fn generate_enum_errors(enum_name: &Ident, enum_field_ident: &Ident, fields: &mut FieldsNamed) -> proc_macro2::TokenStream {
    let inner_match = match get_errors(fields) {
        Some(member) => quote!( #member ),
        None => quote!( None )
    };

    quote!( #enum_name::#enum_field_ident { .. } => #inner_match )
}

/// Generates error code extraction logic for enum variants
/// 
/// Creates match arm code for extracting error codes from diagnostic attributes
/// on specific enum variants. This is used when building the `error_code()` 
/// method implementation for enums.
/// 
/// # Arguments
/// * `enum_name` - The enum type identifier
/// * `enum_field_ident` - The specific variant identifier
/// * `diagnostic` - The diagnostic attributes for this variant
/// 
/// # Returns
/// A token stream containing the match arm for this variant
fn generate_enum_error_code(enum_name: &Ident, enum_field_ident: &Ident, diagnostic: &Diagnostic) -> proc_macro2::TokenStream {
    let inner_match = match diagnostic.code.as_ref() {
        Some(code) => quote!( Some(Box::new(#code.to_string())) ),
        None => quote!( None ),
    };

    quote!( #enum_name::#enum_field_ident { .. } => #inner_match )
}

/// Generates help text extraction logic for enum variants
/// 
/// Creates match arm code for extracting help text from diagnostic attributes
/// on specific enum variants. This is used when building the `help()` 
/// method implementation for enums.
/// 
/// # Arguments
/// * `enum_name` - The enum type identifier
/// * `enum_field_ident` - The specific variant identifier
/// * `diagnostic` - The diagnostic attributes for this variant
/// 
/// # Returns
/// A token stream containing the match arm for this variant
fn generate_enum_help(enum_name: &Ident, enum_field_ident: &Ident, diagnostic: &Diagnostic) -> proc_macro2::TokenStream {
    let inner_match = match diagnostic.help.as_ref() {
        Some(help) => quote!( Some(Box::new(#help.to_string())) ),
        None => quote!( None ),
    };

    quote!( #enum_name::#enum_field_ident { .. } => #inner_match )
}

/// Generates label extraction logic for enum variants
/// 
/// Creates match arm code for extracting labeled spans from fields in specific
/// enum variants. This handles both simple and format string labels, generating
/// appropriate destructuring patterns when labels are present.
/// 
/// # Arguments
/// * `enum_name` - The enum type identifier
/// * `enum_field_ident` - The specific variant identifier
/// * `fields` - The named fields of the variant
/// 
/// # Returns
/// A token stream containing the match arm for this variant
fn generate_enum_labels(enum_name: &Ident, enum_field_ident: &Ident, fields: &mut FieldsNamed) -> proc_macro2::TokenStream {
    let labels = get_labels(fields);
    match labels.is_empty() {
        true => quote!( #enum_name::#enum_field_ident { .. } => None ),
        false => {
            let fields = labels.iter().map(|(field, _)| field).collect::<Vec<_>>();
            let tokens = labels.iter().map(|(_, token)| token).collect::<Vec<_>>();

            quote!( #enum_name::#enum_field_ident { #(#fields),*, .. } => Some(vec![#(#tokens),*]) )
        },
    }
}

/// Generates reference extraction logic for enum variants
/// 
/// Creates match arm code for extracting error references from fields in specific
/// enum variants. This generates appropriate destructuring patterns when reference
/// fields are present for error chaining.
/// 
/// # Arguments
/// * `enum_name` - The enum type identifier
/// * `enum_field_ident` - The specific variant identifier
/// * `fields` - The named fields of the variant
/// 
/// # Returns
/// A token stream containing the match arm for this variant
fn generate_enum_references(enum_name: &Ident, enum_field_ident: &Ident, fields: &mut FieldsNamed) -> proc_macro2::TokenStream {
    let references = get_references(fields);
    match references.is_empty() {
        true => quote!( #enum_name::#enum_field_ident { .. } => None ),
        false => {
            let fields = references.iter().map(|(field, _)| field).collect::<Vec<_>>();
            let tokens = references.iter().map(|(_, token)| token).collect::<Vec<_>>();

            quote!( #enum_name::#enum_field_ident { #(#fields),*, .. } => Some(vec![#(#tokens),*]) )
        },
    }
}

/// Generates trait method implementations for enum types
/// 
/// This function creates match expressions for enum trait methods by iterating
/// through all variants and generating appropriate match arms. It handles both
/// transparent variants (which delegate to inner types) and regular variants
/// (which use their own field attributes).
/// 
/// # Arguments
/// * `enum_name` - The enum type identifier
/// * `function_name` - The trait method being implemented (labels, source_code, etc.)
/// * `variants` - All variants of the enum
/// 
/// # Returns
/// A token stream containing the complete match expression for the method
fn enum_generator(enum_name: &Ident, function_name: Ident, variants: &mut [Variant]) -> proc_macro2::TokenStream {
    let mut lines = Vec::new();
    for enum_field in variants.iter_mut() {
        
        let enum_field_ident = enum_field.ident.clone();
        //println!(" --- {:?} {:?}", enum_field_ident, deluxe::extract_attributes::<_, Diagnostic>(enum_field));
        if let Ok(diagnostic) = deluxe::extract_attributes::<_, Diagnostic>(enum_field) {
            
            // The error details will be comes from sub struct or enum
            if diagnostic.transparent {

                lines.push(quote! { #enum_name::#enum_field_ident ( data ) =>  data.#function_name() });
            } else {
                match &mut enum_field.fields {
                    Fields::Named(fields) => {    
                        //println!("Fields::Named");
                        let tokens = match function_name.to_string().as_str() {
                            "labels" => generate_enum_labels(enum_name, &enum_field_ident, fields),
                            "references" => generate_enum_references(enum_name, &enum_field_ident, fields),
                            "source_code" => generate_enum_source_code(enum_name, &enum_field_ident, fields),
                            "error_code" => generate_enum_error_code(enum_name, &enum_field_ident, &diagnostic),
                            "help" => generate_enum_help(enum_name, &enum_field_ident, &diagnostic),
                            "errors" => generate_enum_errors(enum_name, &enum_field_ident, fields),
                            _ => panic!("Unknown field ({function_name})")
                        };

                        lines.push(tokens);
                    }
                    Fields::Unnamed(_) => {
                        //println!("Fields::Unnamed");
                        let tokens = match function_name.to_string().as_str() {
                            "error_code" => generate_enum_error_code(enum_name, &enum_field_ident, &diagnostic),
                            "help" => generate_enum_help(enum_name, &enum_field_ident, &diagnostic),
                            _ => {
                                //println!("Received(Unnamed) {} and return None", function_name);
                                quote!( #enum_name::#enum_field_ident { .. } => None )
                            }
                        };
                        lines.push(tokens);
                    },
                    Fields::Unit => {
                        //println!("Fields::Unit");

                        let tokens = match function_name.to_string().as_str() {
                            "error_code" => generate_enum_error_code(enum_name, &enum_field_ident, &diagnostic),
                            "help" => generate_enum_help(enum_name, &enum_field_ident, &diagnostic),
                            _ => {
                                //println!("Received(Unit) {} and return None", function_name);
                                quote!( #enum_name::#enum_field_ident { .. } => None )
                            }
                        };

                        lines.push(tokens);
                    }
                };
            }
        } else {
            panic!("#[diagnostic] expected");
        }
    }
    quote!(
        match self {
            #(#lines),*
        }
    )
}

/// Generates `TimuErrorTrait` implementation for enum types
/// 
/// This function processes an enum definition marked with `#[derive(TimuError)]`
/// and generates a complete trait implementation. It builds match expressions
/// for each trait method by analyzing all variants and their diagnostic attributes.
/// 
/// # Enum Support
/// - **Named variants** - Variants with field attributes for labels, source code, etc.
/// - **Transparent variants** - Variants that delegate to inner error types
/// - **Unit variants** - Simple variants with only diagnostic attributes
/// - **Unnamed variants** - Variants with tuple fields
/// 
/// # Arguments
/// * `name` - The enum identifier
/// * `data` - The enum definition data
/// 
/// # Returns
/// A `TokenStream` containing the generated trait implementation
fn build_enum(name: Ident, data: DataEnum) -> TokenStream {
    let mut variants = Vec::new();
    for variant in data.variants.into_iter() {
        variants.push(variant);
    }

    let error_code = enum_generator(&name, format_ident!("error_code"), &mut (variants.clone()));
    let labels = enum_generator(&name, format_ident!("labels"), &mut (variants.clone()));
    let source_code = enum_generator(&name, format_ident!("source_code"), &mut (variants.clone()));
    let help = enum_generator(&name, format_ident!("help"), &mut (variants.clone()));
    let errors = enum_generator(&name, format_ident!("errors"), &mut (variants.clone()));
    let references = enum_generator(&name, format_ident!("references"), &mut (variants.clone()));

    TokenStream::from(quote!{
        impl libtimu_macros_core::traits::TimuErrorTrait for #name {
            fn labels(&self) -> Option<Vec<libtimu_macros_core::traits::LabelField>> { #labels }
            fn references<'a>(&'a self) -> Option<Vec<Box<&'a dyn libtimu_macros_core::traits::TimuErrorTrait>>> { #references }
            fn errors<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn libtimu_macros_core::traits::TimuErrorTrait> + 'a>> { #errors }
            fn source_code(&self) -> Option<Box<libtimu_macros_core::SourceCode>> { #source_code }
            fn error_code(&self) -> Option<Box<dyn std::fmt::Display>> { #error_code }
            fn help(&self) -> Option<Box<dyn std::fmt::Display>> { #help }
        }
    })
}

/// Main entry point for the `#[derive(TimuError)]` macro
/// 
/// This function is the primary procedural macro implementation that processes
/// derive input and generates appropriate trait implementations. It handles both
/// struct and enum types, parsing their diagnostic attributes and generating
/// complete `TimuErrorTrait` implementations.
/// 
/// # Processing Steps
/// 1. Parse the derive input syntax
/// 2. Extract and validate diagnostic attributes
/// 3. Dispatch to appropriate builder (struct or enum)
/// 4. Generate and return the trait implementation
/// 
/// # Supported Types
/// - **Structs** with named fields
/// - **Enums** with any variant types
/// 
/// # Arguments
/// * `input` - The token stream from the derive macro
/// 
/// # Returns
/// A `TokenStream` containing the generated trait implementation or compile errors
/// 
/// # Errors
/// Returns compile errors for:
/// - Missing diagnostic attributes
/// - Unsupported type structures (unions, etc.)
/// - Invalid attribute syntax
pub fn timu_error(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let diagnostic: Diagnostic = match deluxe::extract_attributes(&mut input) {
        Ok(diagnostic) => diagnostic,
        _ => return TokenStream::from(syn::Error::new(input.ident.span(), "diagnostic is missing").to_compile_error())
    };
    
    match input.data {
        syn::Data::Struct(data) => build_struct(input.ident, diagnostic, data),
        syn::Data::Enum(data) => build_enum(input.ident, data),
        _ => TokenStream::from(syn::Error::new(input.ident.span(), "Only structs and enums with named fields can derive `TimuError`").to_compile_error())
    }
}
