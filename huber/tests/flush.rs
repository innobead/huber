use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{install_pkg, reset_huber, PKG_VERSION_1, PKG_VERSION_2};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_flush_nothing() {
    defer! {
        reset_huber();
    }

    huber_cmd!(arg("flush")
        .assert()
        .success()
        .stderr("[INFO ] Nothing to flush\n"));
}

#[test]
#[sequential]
fn test_flush() {
    defer! {
        reset_huber();
    }

    install_pkg(PKG_VERSION_1);
    install_pkg(PKG_VERSION_2);

    let assert = huber_cmd!(arg("flush").assert().success());
    let tokens: Vec<_> = PKG_VERSION_1.splitn(2, '@').collect();

    assert_eq_last_line!(
        assert.get_output().stderr,
        format!(
            "[INFO ] {} (version: v{}, source: github) removed",
            tokens[0], tokens[1]
        )
    );
}
