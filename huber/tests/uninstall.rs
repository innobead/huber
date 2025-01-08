use std::path::Path;

use assert_cmd::Command;
use common::{install_pkg, uninstall_pkg, HUBER_EXEC};
use scopeguard::defer;

use crate::common::reset_huber;

#[macro_use]
mod common;

#[test]
fn test_uninstall() {
    defer! {
        reset_huber();
    }

    install_pkg("k3s");

    let assert = uninstall_pkg("k3s");
    assert_eq_last_line!(assert.get_output().stderr, "[INFO ] Uninstalled k3s");
}

#[test]
fn test_uninstall_fail() {
    defer! {
        reset_huber();
    }

    Command::new(HUBER_EXEC)
        .arg("uninstall")
        .arg("pkg_notfound")
        .env(
            "MANAGED_PKG_ROOT_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .failure()
        .stderr("[WARN ] package not found: \"pkg_notfound\"\n");
}
