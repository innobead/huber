#![cfg(test)]

use std::path::Path;

use assert_cmd::Command;
use scopeguard::defer;

use crate::common::{install_pkg, reset_huber, HUBER_EXEC};

mod common;

#[test]
fn test_info() {
    defer! {
        reset_huber();
    }

    install_pkg("k9s@v0.32.7");

    Command::new(HUBER_EXEC)
        .arg("info")
        .arg("k9s")
        .env(
            "MANAGED_PKG_ROOT_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .success();
}

#[test]
fn test_info_fail() {
    defer! {
        reset_huber();
    }

    Command::new(HUBER_EXEC)
        .arg("info")
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
