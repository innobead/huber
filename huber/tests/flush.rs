use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{install_pkgs, reset_huber, PKG_VERSION_1, PKG_VERSION_2};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_flush_nothing() {
    defer! {
        reset_huber();
    }

    let assert = huber_cmd!(arg("flush").assert().success());
    assert_contain_line_regex!(assert.get_output().stderr, "Nothing to flush");
}

#[test]
#[sequential]
fn test_flush() {
    defer! {
        reset_huber();
    }

    install_pkgs(&[PKG_VERSION_1]);
    install_pkgs(&[PKG_VERSION_2]);

    let assert = huber_cmd!(arg("flush").assert().success());
    let tokens: Vec<_> = PKG_VERSION_1.splitn(2, '@').collect();
    let pkg = tokens[0];
    let version = tokens[1];

    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(
            r#"{} \(version: {}, source: github\) removed"#,
            pkg, version
        )
    );
}
