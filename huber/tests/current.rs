use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{install_pkgs, reset_huber, INVALID_PKG, INVALID_PKG_VERSION, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_current() {
    defer! {
        reset_huber();
    }

    install_pkgs(&[PKG_VERSION_1]);

    let assert = huber_cmd!(arg("current").arg(PKG_VERSION_1).assert().success());
    assert_eq_last_line_regex!(
        assert.get_output().stderr,
        &format!(r#"{} is now the current version"#, PKG_VERSION_1)
    );
}

#[test]
#[sequential]
fn test_current_fail() {
    defer! {
        reset_huber();
    }

    let assert = huber_cmd!(arg("current").arg(INVALID_PKG_VERSION).assert().failure());
    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(r#"Package not installed: "{}""#, INVALID_PKG)
    );
}
