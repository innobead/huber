use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{install_pkgs, reset_huber, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_update() {
    defer! {
        reset_huber();
    }

    install_pkgs(&[PKG_VERSION_1]);
    let tokens: Vec<_> = PKG_VERSION_1.splitn(2, "@").collect();

    huber_cmd!(arg("update").arg(tokens[0]).assert().success());
}
