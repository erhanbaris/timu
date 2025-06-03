use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, Ident, Variant};
mod error;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(label))]
struct Label(String);


#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(diagnostic))]
struct Diagnostic {
    #[deluxe(default)]
    code: Option<String>,

    #[deluxe(default)]
    help: Option<String>,
}

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

fn get_help(fields: &mut FieldsNamed) -> Option<proc_macro2::TokenStream> {
    for field in fields.named.iter_mut() {
        if field.attrs.iter().any(|attr| attr.path().is_ident("source_code")) {
            let name = &field.ident;
            return Some(quote! { Some(self.#name.clone()) });
        }
    }

    None
}

fn get_labels(fields: &mut FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    let mut field_values = Vec::new();
    for field in fields.named.iter_mut() {
        if let Ok(Label(message)) = deluxe::extract_attributes(field) {
            let name = &field.ident;
            field_values.push(quote! {
                libtimu_macros_core::traits::LabelField {
                    label: #message.to_string(),
                    position: self.#name,
                }
            });
        }
    }

    field_values
}

fn build_struct(name: Ident, diagnostic: Diagnostic, mut data: DataStruct) -> TokenStream {
    if let Fields::Named(fields) = &mut data.fields {
        let source_code = match get_source_code(fields) {
            Some(member) => {
                quote!( Some(self.#member.clone()) )
            },
            None => quote!( None ),
        };

        let error_code = match diagnostic.code {
            Some(code) => quote!( Some(#code.to_string()) ),
            None => quote!( None ),
        };


        let help = match diagnostic.help {
            Some(help) => quote!( Some(#help.to_string()) ),
            None => match get_help(fields) {
                Some(help) => help,
                None => quote!( None ),
            },
        };

        let labels = get_labels(fields);

        return TokenStream::from(quote!{
            impl libtimu_macros_core::traits::TimuErrorTrait for #name {
                fn labels(&self) -> Vec<libtimu_macros_core::traits::LabelField> {
                    vec![#(#labels),*]
                }
                fn source_code(&self) -> Option<String> { #source_code }
                fn error_code(&self) -> Option<String> { #error_code }
                fn help(&self) -> Option<String> { #help }
            }
        });
    }

    TokenStream::from(syn::Error::new(name.span(), "Only structs and enums with named fields can derive `TimuError`").to_compile_error())
}

fn enum_generator(name: &Ident, function_name: Ident, variants: &Vec<Variant>) -> proc_macro2::TokenStream {
    let mut lines = Vec::new();
    for variant in variants.iter() {
        let variant_ident = &variant.ident;
        lines.push(quote! { #name::#variant_ident ( data ) =>  data.#function_name() });
    }

    proc_macro2::TokenStream::from(quote!(
        match self {
            #(#lines),*
        }
    ))
}

fn build_enum(name: Ident, data: DataEnum) -> TokenStream {
    let mut variants = Vec::new();
    for variant in data.variants.into_iter() {
        variants.push(variant);
    }

    let labels = enum_generator(&name, format_ident!("labels"), &variants);
    let source_code = enum_generator(&name, format_ident!("source_code"), &variants);
    let error_code = enum_generator(&name, format_ident!("error_code"), &variants);
    let help = enum_generator(&name, format_ident!("help"), &variants);

    return TokenStream::from(quote!{
        impl libtimu_macros_core::traits::TimuErrorTrait for #name {
            fn labels(&self) -> Vec<libtimu_macros_core::traits::LabelField> { #labels }
            fn source_code(&self) -> Option<String> { #source_code }
            fn error_code(&self) -> Option<String> { #error_code }
            fn help(&self) -> Option<String> { #help }
        }
    });
}

#[proc_macro_derive(TimuError, attributes(source_code, label, help, diagnostic))]
pub fn derive_timu_error(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let diagnostic: Diagnostic = match deluxe::extract_attributes(&mut input) {
        Ok(diagnostic) => diagnostic,
        _ => return TokenStream::from(syn::Error::new(input.ident.span(), "diagnostic is missing").to_compile_error())
    };

    match input.data {
        syn::Data::Struct(data) => return build_struct(input.ident, diagnostic, data),
        syn::Data::Enum(data) => return build_enum(input.ident, data),
        _ => {}
    };

    TokenStream::from(syn::Error::new(input.ident.span(), "Only structs and enums with named fields can derive `TimuError`").to_compile_error())
}
