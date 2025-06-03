use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Fields};
use darling::FromField;

mod error;

#[derive(Debug, FromField)]
#[darling(attributes(label), forward_attrs)]
struct LabelField {
    message: String,
}

#[proc_macro_derive(TimuError, attributes(source_code, label))]
pub fn derive_timu_error(input: TokenStream) -> TokenStream {
let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(ref data) = input.data {
        if let Fields::Named(ref fields) = data.fields {
            let mut field_values = Vec::new();
            for (i, field) in fields.named.iter().enumerate() {
                for attr in field.attrs.iter() {
                    if attr.path().is_ident("label") {
                        
                        let label = if let Some(ident) = field.ident.clone() {
                            syn::Member::Named(ident)
                        } else {
                            syn::Member::Unnamed(syn::Index {
                                index: i as u32,
                                span: field.span(),
                            })
                        };
                        
                        println!("Label: {:?}", label);

                        let label = LabelField::from_field(field).unwrap();
                        let name = &field.ident;
                        let message = label.message.to_string();



                        field_values.push(quote! {
                            libtimu_macros_core::traits::LabelField {
                                label: #message,
                                position: self.#name,
                            }
                        });
                    }
                    if attr.path().is_ident("source_code") {
                        if let Some(ident) = field.ident.clone() {
                            syn::Member::Named(ident)
                        } else {
                            syn::Member::Unnamed(syn::Index {
                                index: i as u32,
                                span: field.span(),
                            })
                        };
                    }
                }
            }


            let name = input.ident;
            return TokenStream::from(quote!(
                impl libtimu_macros_core::traits::TimuErrorTrait for #name {
                    fn labels(&self) -> Vec<libtimu_macros_core::traits::LabelField> {
                        vec![#(#field_values),*]
                    }
                }));
        }
    }

    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Only structs with named fields can derive `FromRow`",
        )
        .to_compile_error(),
    )
}
