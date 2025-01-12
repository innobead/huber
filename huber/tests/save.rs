use std::fs;

use filepath::FilePath;
use scopeguard::defer;
use sequential_test::sequential;
use crate::common::{install_pkg, reset_huber, save_pkg_list};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_save() {
    defer! {
        reset_huber();
    }

    install_pkg("k9s");

    let file = tempfile::tempfile().unwrap();
    let path = file.path().unwrap();
    defer! {
        let  _ = fs::remove_file(&path);
    };

    let result = save_pkg_list(path.to_string_lossy().as_ref());
    assert_eq_last_line!(
        result.get_output().stderr,
        format!(
            "[INFO ] Saved the package list to {}",
            path.canonicalize().unwrap().to_string_lossy().to_string()
        )
    );
    assert!(fs::exists(&path).unwrap());
}
