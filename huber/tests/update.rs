use scopeguard::defer;

use crate::common::{install_pkg, reset_huber, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
fn test_update() {
    defer! {
        reset_huber();
    }

    install_pkg(PKG_VERSION_1);
    let tokens: Vec<_> = PKG_VERSION_1.splitn(2, "@").collect();

    huber_cmd!(arg("update").arg(tokens[0]).assert().success());
}
