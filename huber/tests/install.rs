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
    assert_eq_last_line_regex!(
        assert.get_output().stderr,
        &format!(r"\[INFO \] {}@latest/\S+ installed", pkg)
    );
}

#[test]
#[sequential]
fn test_install_compression() {
    defer! {
        reset_huber();
    }

    let assert = install_pkg("just");
    assert_eq_last_line_regex!(
        assert.get_output().stderr,
        &format!(r"\[INFO \] {}@latest/\S+ installed", "just")
    );
}

#[test]
#[sequential]
fn test_install_fail() {
    defer! {
        reset_huber();
    }

    let tokens = INVALID_PKG_VERSION.splitn(2, '@').collect::<Vec<_>>();
    huber_cmd!(arg("install")
        .arg(INVALID_PKG_VERSION)
        .assert()
        .success()
        .stderr(format!("[WARN ] {} not found\n", tokens[0])));
}
