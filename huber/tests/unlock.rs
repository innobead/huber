use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{install_pkgs, lock_pkg, reset_huber, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_unlock() {
    defer! {
        reset_huber();
    }

    install_pkgs(&[PKG_VERSION_1]);

    let tokens = PKG_VERSION_1.splitn(2, '@').collect::<Vec<_>>();
    let pkg = tokens[0];
    let version = tokens[1].trim_start_matches('v');
    let pkg_version = format!("{}@{}", pkg, version);

    lock_pkg(&pkg_version);

    huber_cmd!(arg("unlock").arg(pkg).assert().success());
}
