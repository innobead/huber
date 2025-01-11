use scopeguard::defer;

use crate::common::{install_pkg, reset_huber, INVALID_PKG, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
fn test_info() {
    defer! {
        reset_huber();
    }

    install_pkg(PKG_VERSION_1);
    let pkg = PKG_VERSION_1.splitn(2, '@').collect::<Vec<_>>()[0];

    huber_cmd!(arg("info").arg(pkg).assert().success());
}

#[test]
fn test_info_fail() {
    defer! {
        reset_huber();
    }

    huber_cmd!(arg("info")
        .arg(INVALID_PKG)
        .assert()
        .failure()
        .stderr(format!("[ERROR] Package not found: \"{}\"\n", INVALID_PKG)));
}
