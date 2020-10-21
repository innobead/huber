extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(DIAware)]
pub fn derive_di_aware(input: TokenStream) -> TokenStream {
    let _derive_input = parse_macro_input!(input as DeriveInput);

    TokenStream::new()
}

/*
Derive <--- create
impl DIObjectTrait for RootCmd {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self {
        Self { container }
    }
}
 */
