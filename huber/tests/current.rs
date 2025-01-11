use scopeguard::defer;

use crate::common::{
    install_pkg, reset_huber, INVALID_PKG, INVALID_PKG_VERSION, PKG_VERSION_1, PKG_VERSION_1_ASSERT,
};

#[macro_use]
mod common;

#[test]
fn test_current() {
    defer! {
        reset_huber();
    }

    install_pkg(PKG_VERSION_1);

    let assert = huber_cmd!(arg("current").arg(PKG_VERSION_1).assert().success());
    assert_eq_last_line!(
        assert.get_output().stderr,
        format!(
            "[INFO ] {} is now the current version",
            PKG_VERSION_1_ASSERT
        )
    );
}

#[test]
fn test_current_fail() {
    defer! {
        reset_huber();
    }

    huber_cmd!(arg("current")
        .arg(INVALID_PKG_VERSION)
        .assert()
        .failure()
        .stderr(format!(
            "[ERROR] Package not installed: \"{}\"\n",
            INVALID_PKG
        )));
}
