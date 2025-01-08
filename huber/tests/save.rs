use std::fs;
use std::path::Path;

use scopeguard::defer;

use crate::common::{install_pkg, reset_huber, save_pkg_list};

#[macro_use]
mod common;

#[test]
fn test_save() {
    defer! {
        reset_huber();
    }

    install_pkg("k9s");

    let file_name = "huber-packages.txt";
    let file_path = Path::new(file_name);
    defer! {
        let  _ = fs::remove_file(file_path);
    };

    let result = save_pkg_list(file_name);
    assert_eq_last_line!(
        result.get_output().stderr,
        format!(
            "[INFO ] Saved the package list to {}",
            file_path.canonicalize().unwrap().to_string_lossy()
        )
    );
    assert!(fs::exists(file_path).unwrap());
}
