use scopeguard::defer;
use sequential_test::sequential;
use crate::common::{install_pkg, reset_huber, INVALID_PKG_VERSION, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_install() {
    defer! {
        reset_huber();
    }

    let pkg = PKG_VERSION_1.splitn(2, '@').collect::<Vec<_>>()[0];
    let assert = install_pkg(pkg);
    assert_eq_last_line!(
        assert.get_output().stderr,
        format!("[INFO ] {}@latest installed", pkg)
    );
}

#[test]
#[sequential]
fn test_install_fail() {
    defer! {
        reset_huber();
    }

    huber_cmd!(arg("install")
        .arg(INVALID_PKG_VERSION)
        .assert()
        .failure()
        .stderr("[ERROR] Package not found: \"pkg_notfound\"\n"));
}
