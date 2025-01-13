use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{install_pkg, reset_huber, update_pkg, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_lock_fail() {
    defer! {
        reset_huber();
    }

    let tokens: Vec<_> = PKG_VERSION_1.splitn(2, '@').collect();
    let assert = huber_cmd!(arg("lock").arg(PKG_VERSION_1).assert().failure());

    assert_eq_last_line!(
        assert.get_output().stderr,
        format!(
            "[ERROR] Package unable to lock: Package not installed: \"{}\"",
            tokens[0]
        )
    );
}

#[test]
#[sequential]
fn test_lock() {
    defer! {
        reset_huber();
    }

    install_pkg(PKG_VERSION_1);
    let assert = huber_cmd!(arg("lock").arg(PKG_VERSION_1).assert().success());
    assert_eq_last_line!(
        assert.get_output().stderr,
        "[INFO ] Packages locked successfully"
    );

    huber_cmd!(arg("lock").arg("show").assert().success());

    let assert = update_pkg("k9s");
    let tokens: Vec<_> = PKG_VERSION_1.splitn(2, '@').collect();
    assert_eq_last_line_regex!(
        assert.get_output().stderr,
        &format!(
            r"\[WARN \] Package k9s is locked to version {}. Skipping update to \S+",
            tokens[1]
        )
    );
}

#[test]
#[sequential]
fn test_lock_semver_req() {
    defer! {
        reset_huber();
    }

    let tokens: Vec<_> = PKG_VERSION_1.splitn(2, '@').collect();
    install_pkg(PKG_VERSION_1);

    let assert = huber_cmd!(arg("lock")
        .arg(format!("{}@~{}", tokens[0], tokens[1]))
        .assert()
        .success());

    assert_eq_last_line!(
        assert.get_output().stderr,
        "[INFO ] Packages locked successfully"
    );

    let assert = update_pkg(tokens[0]);
    assert_eq_last_line_regex!(
        assert.get_output().stderr,
        &format!(
            r"\[INFO \] Package {} updated to \S+ successfully",
            tokens[0]
        )
    );
}
