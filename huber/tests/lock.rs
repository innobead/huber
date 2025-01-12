use std::path::Path;

use assert_cmd::Command;
use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{install_pkg, reset_huber, update_pkg, HUBER_EXEC, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_lock_fail() {
    defer! {
        reset_huber();
    }

    let assert = Command::new(HUBER_EXEC)
        .arg("lock")
        .arg("k9s@0.32.7")
        .env(
            "huber_pkg_root_dir",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .failure();

    assert_eq_last_line!(
        assert.get_output().stderr,
        "[ERROR] Package unable to lock: Package not installed: \"k9s\""
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
