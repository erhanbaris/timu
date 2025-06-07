use proc_macro::TokenStream;
use crate::error::timu_error;
mod error;

#[proc_macro_derive(TimuError, attributes(source_code, label, help, diagnostic, errors, reference))]
pub fn derive_timu_error(input: TokenStream) -> TokenStream {
    timu_error(input)
}
