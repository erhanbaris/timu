use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Fields};
use darling::{ast, util, FromDeriveInput, FromField};

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

            for (i, field) in fields.named.iter().enumerate() {
                for attr in field.attrs.iter() {
                    if attr.path().is_ident("label") {
                        let label = LabelField::from_field(field).unwrap();
                        println!("{:?}", label);
                    }
                }
            }

            let field_vals = fields.named.iter().enumerate().map(|(_, field)| {
                let name = &field.ident;
                quote!(self.#name)
            });

            let name = input.ident;

            return TokenStream::from(quote!(
                impl libtimu_macros_core::traits::TimuErrorTrait for #name {
                    fn labels(&self) -> Vec<SourceSpan> {
                        vec![#(#field_vals),*]
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
