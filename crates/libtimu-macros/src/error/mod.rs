use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Variant};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(label))]
struct Label(String);

#[derive(deluxe::ExtractAttributes, deluxe::ParseMetaItem)]
#[deluxe(attributes(diagnostic))]
#[derive(Debug)]
struct Diagnostic {
    #[deluxe(default)]
    code: Option<String>,

    #[deluxe(default)]
    help: Option<String>,

    #[deluxe(default)]
    transparent: bool,
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
            return Some(quote! { Some(Box::new(self.#name.clone())) });
        }
    }

    None
}

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

fn generate_enum_source_code(enum_name: &Ident, enum_field_ident: &Ident, fields: &mut FieldsNamed) -> proc_macro2::TokenStream {
    let inner_match = match get_source_code(fields) {
        Some(member) => quote!( #member ),
        None => quote!( None )
    };

    quote!( #enum_name::#enum_field_ident { .. } => #inner_match )
}

fn generate_enum_errors(enum_name: &Ident, enum_field_ident: &Ident, fields: &mut FieldsNamed) -> proc_macro2::TokenStream {
    let inner_match = match get_errors(fields) {
        Some(member) => quote!( #member ),
        None => quote!( None )
    };

    quote!( #enum_name::#enum_field_ident { .. } => #inner_match )
}

fn generate_enum_error_code(enum_name: &Ident, enum_field_ident: &Ident, diagnostic: &Diagnostic) -> proc_macro2::TokenStream {

    //println!("diagnostic {:?}", diagnostic);
    let inner_match = match diagnostic.code.as_ref() {
        Some(code) => quote!( Some(Box::new(#code.to_string())) ),
        None => {
            //println!("No code in diag. {}", enum_field_ident);
            quote!( None )
        },
    };

    quote!( #enum_name::#enum_field_ident { .. } => #inner_match )
}

fn generate_enum_help(enum_name: &Ident, enum_field_ident: &Ident, diagnostic: &Diagnostic) -> proc_macro2::TokenStream {
    let inner_match = match diagnostic.help.as_ref() {
        Some(help) => quote!( Some(Box::new(#help.to_string())) ),
        None => quote!( None ),
    };

    quote!( #enum_name::#enum_field_ident { .. } => #inner_match )
}

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

fn generate_enum_references(enum_name: &Ident, enum_field_ident: &Ident, fields: &mut FieldsNamed) -> proc_macro2::TokenStream {
    let labels = get_references(fields);
    match labels.is_empty() {
        true => quote!( #enum_name::#enum_field_ident { .. } => None ),
        false => {
            let fields = labels.iter().map(|(field, _)| field).collect::<Vec<_>>();
            let tokens = labels.iter().map(|(_, token)| token).collect::<Vec<_>>();

            quote!( #enum_name::#enum_field_ident { #(#fields),*, .. } => Some(vec![#(#tokens),*]) )
        },
    }
}

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
                            _ => panic!("Unknown field ({})", function_name)
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
