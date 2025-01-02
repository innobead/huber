use std::fs;

use filepath::FilePath;
use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{install_pkgs, reset_huber, save_pkg_list, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_save() {
    defer! {
        reset_huber();
    }

    install_pkgs(&[PKG_VERSION_1]);

    let file = tempfile::tempfile().unwrap();
    let path = file.path().unwrap();
    defer! {
        let  _ = fs::remove_file(&path);
    };
    drop(file);

    let assert = save_pkg_list(path.to_string_lossy().to_string().as_ref());
    //FIXME: should check the file path
    assert_contain_line_regex!(assert.get_output().stderr, "Saved the package list to");
    assert!(fs::exists(&path).unwrap());
}
