use std::path::Path;

use assert_cmd::Command;
use scopeguard::defer;

use crate::common::{install_pkg, reset_huber, HUBER_EXEC};

#[macro_use]
mod common;

#[test]
fn test_flush_nothing() {
    defer! {
        reset_huber();
    }

    Command::new(HUBER_EXEC)
        .arg("flush")
        .env(
            "MANAGED_PKG_ROOT_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .success()
        .stderr("[INFO ] Nothing to flush\n");
}

#[test]
fn test_flush() {
    defer! {
        reset_huber();
    }

    install_pkg("k9s@v0.32.6");
    install_pkg("k9s@v0.32.7");

    let assert = Command::new(HUBER_EXEC)
        .arg("flush")
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
        "[INFO ] k9s (version: v0.32.6, source: github) removed"
    );
}
