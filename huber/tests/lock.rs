use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{install_pkg, reset_huber, update_pkg, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_lock() {
    defer! {
        reset_huber();
    }

    install_pkg(PKG_VERSION_1);
    let tokens: Vec<_> = PKG_VERSION_1.splitn(2, '@').collect();
    let pkg = tokens[0];
    let version = tokens[1].trim_start_matches('v');
    let pkg_version = format!("{}@{}", pkg, version);

    let assert = huber_cmd!(arg("lock").arg(pkg_version).assert().success());
    assert_contain_line_regex!(
        assert.get_output().stderr,
        r#"Packages locked successfully"#
    );

    huber_cmd!(arg("lock").arg("show").assert().success());

    let assert = update_pkg("k9s");
    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(
            r#"Package {} is locked to version {}. Skipping update to \S+"#,
            pkg, version
        )
    );
}

#[test]
#[sequential]
fn test_lock_fail() {
    defer! {
        reset_huber();
    }

    let tokens: Vec<_> = PKG_VERSION_1.splitn(2, '@').collect();
    let version = tokens[1].trim_start_matches('v');
    let pkg = tokens[0];
    let pkg_version = format!("{}@{}", pkg, version);

    let assert = huber_cmd!(arg("lock").arg(pkg_version).assert().success());

    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(r#"Skipped locking package {}@"#, pkg)
    );
}

#[test]
#[sequential]
fn test_lock_semver_req() {
    defer! {
        reset_huber();
    }

    install_pkg(PKG_VERSION_1);

    let tokens: Vec<_> = PKG_VERSION_1.splitn(2, '@').collect();
    let pkg = tokens[0];
    let version = tokens[1].trim_start_matches('v');

    let assert = huber_cmd!(arg("lock")
        .arg(format!("{}@~{}", pkg, version))
        .assert()
        .success());

    assert_contain_line_regex!(
        assert.get_output().stderr,
        r#"Packages locked successfully"#
    );

    let assert = update_pkg(pkg);
    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(r#"Package {} updated to \S+ successfully"#, pkg)
    );
}
