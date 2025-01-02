extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

#[proc_macro]
pub fn process_lock(input: TokenStream) -> TokenStream {
    let lock_file_pathbuf_expr = parse_macro_input!(input as Expr);

    let result = quote! {
        use huber_common::model::config::Config;
        use std::fs::File;
        use fs2::FileExt;
        use log::{error, info};
        use anyhow::anyhow;

        let lock_path = #lock_file_pathbuf_expr;
        let f = if !lock_path.exists() {
            File::create(&lock_path)
        } else {
            File::open(&lock_path)
        }.unwrap();

        let r = f.try_lock_exclusive();
        match r {
            Ok(_) => {
                info!("{}: {:?}", "Locking the operation", lock_path);
            },

            Err(e) => {
                error!("{:?}: {:?}", lock_path, e);
                return Err(anyhow!("huber is already running by another process for the exclusion operation. Please try after the operation finished. {:?}", e))
            }
        }
    };

    result.into()
}
