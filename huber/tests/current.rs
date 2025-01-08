use std::path::Path;

use assert_cmd::Command;
use scopeguard::defer;

use crate::common::{install_pkg, reset_huber, HUBER_EXEC};

#[macro_use]
mod common;

#[test]
fn test_current() {
    defer! {
        reset_huber();
    }

    install_pkg("k9s@v0.32.7");

    let assert = Command::new(HUBER_EXEC)
        .arg("current")
        .arg("k9s@v0.32.7")
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
        "[INFO ] k9s@v0.32.7 is now the current version"
    );
}

#[test]
fn test_current_fail() {
    defer! {
        reset_huber();
    }

    Command::new(HUBER_EXEC)
        .arg("current")
        .arg("pkg_notfound@1.1.1")
        .env(
            "MANAGED_PKG_ROOT_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .failure()
        .stderr("[WARN ] package not installed: \"pkg_notfound\"\n");
}
