use std::fs;
use std::path::Path;

use assert_cmd::Command;
use scopeguard::defer;

use crate::common::{install_pkg, reset_huber, save_pkg_list, HUBER_EXEC};

#[macro_use]
mod common;

#[test]
fn test_load() {
    defer! {
        reset_huber();
    }

    let file_name = "huber-packages.txt";
    defer!(fs::remove_file(file_name).unwrap(););

    install_pkg("k9s");
    save_pkg_list(file_name);

    let assert = Command::new(HUBER_EXEC)
        .arg("load")
        .arg("--file")
        .arg(file_name)
        .env(
            "MANAGED_PKG_ROOT_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .success();
    assert_eq_last_line!(
        assert.get_output().stderr,
        "[INFO ] Installed packages: total 1"
    );
}
