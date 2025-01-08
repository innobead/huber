use std::path::Path;

use assert_cmd::Command;
use scopeguard::defer;

use crate::common::{install_pkg, reset_huber, HUBER_EXEC};

#[macro_use]
mod common;

#[test]
fn test_install() {
    defer! {
        reset_huber();
    }

    let assert = install_pkg("k9s");
    assert_eq_last_line!(assert.get_output().stderr, "[INFO ] k3s@latest installed");
}

#[test]
fn test_install_fail() {
    defer! {
        reset_huber();
    }

    Command::new(HUBER_EXEC)
        .arg("install")
        .arg("pkg_notfound@0.1.0")
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
