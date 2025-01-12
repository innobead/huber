use std::fs;

use filepath::FilePath;
use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{install_pkg, reset_huber, save_pkg_list, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_load() {
    defer! {
        reset_huber();
    }

    let file = tempfile::tempfile().unwrap();
    let path = file.path().unwrap().to_string_lossy().to_string();
    defer!(fs::remove_file(&path).unwrap());

    install_pkg(PKG_VERSION_1);
    save_pkg_list(&path);

    let assert = huber_cmd!(arg("load").arg("--file").arg(&path).assert().success());
    assert_eq_last_line!(
        assert.get_output().stderr,
        "[INFO ] Installed packages: total 1"
    );
}
