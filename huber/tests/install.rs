use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{install_pkgs, reset_huber, INVALID_PKG_VERSION, PKG_VERSION_1, PKG_VERSION_2};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_install() {
    defer! {
        reset_huber();
    }

    let pkg = PKG_VERSION_1.splitn(2, '@').collect::<Vec<_>>()[0];
    let assert = install_pkgs(&[pkg]);
    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(r#"{}@latest/\S+ installed"#, pkg)
    );
}

#[test]
#[sequential]
fn test_install_version() {
    defer! {
        reset_huber();
    }

    let tokens: Vec<_> = PKG_VERSION_1.splitn(2, '@').collect();
    let pkg = tokens[0];
    let version = tokens[1];

    let assert = install_pkgs(&[&format!("{}@{}", pkg, version)]);
    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(r"{}@{} installed", pkg, version)
    );

    let tokens: Vec<_> = PKG_VERSION_2.splitn(2, '@').collect();
    let pkg = tokens[0];
    let version = tokens[1].trim_start_matches('v');

    let assert = install_pkgs(&[&format!("{}@{}", pkg, version)]);
    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(r"{}@{} installed", pkg, tokens[1])
    );
}

#[test]
#[sequential]
fn test_install_compression() {
    defer! {
        reset_huber();
    }

    let assert = install_pkgs(&["just"]);
    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(r#"{}@latest/\S+ installed"#, "just")
    );
}

#[test]
#[sequential]
fn test_install_multiple_packages() {
    defer! {
        reset_huber();
    }

    let assert = install_pkgs(&["argocd", "kubectl"]);

    // Skip tests if encountering `Sending warning alert CloseNotify` error.
    // This error would happen when running tests in GitHub workflow CI.
    if String::from_utf8(assert.get_output().stderr.clone())
        .unwrap()
        .contains("Sending warning alert CloseNotify")
    {
        assert!(
            true,
            "Skipped tests, because encountering `Sending warning alert CloseNotify` error"
        );
        return;
    }

    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(r#"{}@latest/\S+ installed"#, "argocd")
    );
    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(r#"{}@latest/\S+ installed"#, "kubectl")
    );
}

#[cfg(not(target_os = "windows"))]
#[test]
#[sequential]
fn test_install_no_artifact_templates() {
    defer! {
        reset_huber();
    }

    let assert = install_pkgs(&["bat"]);
    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(r#"{}@latest/\S+ installed"#, "bat")
    );

    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(
            r#"Decompressing {}-\S+.tar.gz which has extension "tar.gz""#,
            "bat"
        )
    );
}

#[cfg(target_os = "linux")]
#[test]
#[sequential]
fn test_install_stdlib() {
    defer! {
        reset_huber();
    }

    let assert = install_pkgs(&["bat", "--prefer-stdlib", "gnu"]);
    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(
            r#"Decompressing {}-\S+linux-gnu.tar.gz which has extension "tar.gz""#,
            "bat"
        )
    );

    assert_not_contain_line_regex!(
        assert.get_output().stderr,
        &format!(
            r#"Decompressing {}-\S+linux-musl.tar.gz which has extension "tar.gz""#,
            "bat"
        )
    );
}

#[test]
#[sequential]
fn test_install_fail() {
    defer! {
        reset_huber();
    }

    let pkg = INVALID_PKG_VERSION.splitn(2, '@').collect::<Vec<_>>()[0];
    let assert = huber_cmd!(arg("install").arg(INVALID_PKG_VERSION).assert().success());

    assert_contain_line_regex!(assert.get_output().stderr, &format!(r#"{} not found"#, pkg));
}
