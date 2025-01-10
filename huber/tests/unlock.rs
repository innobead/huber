use scopeguard::defer;

use crate::common::{install_pkg, lock_pkg, reset_huber, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
fn test_unlock() {
    defer! {
        reset_huber();
    }

    install_pkg(PKG_VERSION_1);
    lock_pkg(PKG_VERSION_1);

    huber_cmd!(arg("unlock").arg("k9s").assert().success());
}
