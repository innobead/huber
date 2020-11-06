extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(DIAware)]
pub fn derive_di_aware(input: TokenStream) -> TokenStream {
    let _derive_input = parse_macro_input!(input as DeriveInput);
    TokenStream::new()
}
