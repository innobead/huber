use std::path::Path;

use assert_cmd::Command;
use scopeguard::defer;

use crate::common::{install_pkg, reset_huber, HUBER_EXEC};

#[macro_use]
mod common;

#[test]
fn test_lock_fail() {
    defer! {
        reset_huber();
    }

    Command::new(HUBER_EXEC)
        .arg("lock")
        .env(
            "MANAGED_PKG_ROOT_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .failure()
        .stderr("[WARN ] no packages locked\n");
}

#[test]
fn test_lock() {
    defer! {
        reset_huber();
    }

    install_pkg("k9s@v0.32.7");

    Command::new(HUBER_EXEC)
        .arg("lock")
        .env(
            "MANAGED_PKG_ROOT_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .failure()
        .stderr("[INFO ] Packages locked successfully\n");
}
