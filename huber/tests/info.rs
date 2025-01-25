use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{install_pkg, reset_huber, INVALID_PKG, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_info() {
    defer! {
        reset_huber();
    }

    install_pkg(PKG_VERSION_1);
    let pkg = PKG_VERSION_1.splitn(2, '@').collect::<Vec<_>>()[0];

    huber_cmd!(arg("info").arg(pkg).assert().success());
}

#[test]
#[sequential]
fn test_info_fail() {
    defer! {
        reset_huber();
    }

    let assert = huber_cmd!(arg("info").arg(INVALID_PKG).assert().failure());
    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(r#"Package not found: "{}""#, INVALID_PKG)
    );
}
