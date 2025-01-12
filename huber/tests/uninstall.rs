use common::{install_pkg, uninstall_pkg};
use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{reset_huber, INVALID_PKG};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_uninstall() {
    defer! {
        reset_huber();
    }

    install_pkg("k3s");

    let assert = uninstall_pkg("k3s");
    assert_eq_last_line!(assert.get_output().stderr, "[INFO ] Uninstalled k3s");
}

#[test]
#[sequential]
fn test_uninstall_fail() {
    defer! {
        reset_huber();
    }

    huber_cmd!(arg("uninstall")
        .arg(INVALID_PKG)
        .assert()
        .failure()
        .stderr(format!("[ERROR] Package not found: \"{}\"\n", INVALID_PKG)));
}
