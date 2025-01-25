use common::{install_pkg, uninstall_pkg};
use scopeguard::defer;
use sequential_test::sequential;

use crate::common::{reset_huber, INVALID_PKG, PKG_VERSION_1};

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_uninstall() {
    defer! {
        reset_huber();
    }

    let tokens: Vec<_> = PKG_VERSION_1.splitn(2, '@').collect();
    let pkg = tokens[0];
    install_pkg(pkg);

    let assert = uninstall_pkg(pkg);
    assert_contain_line_regex!(assert.get_output().stderr, &format!("Uninstalled {}", pkg));
}

#[test]
// #[sequential]
fn test_uninstall_fail() {
    defer! {
        reset_huber();
    }

    let assert = huber_cmd!(arg("uninstall").arg(INVALID_PKG).assert().success());
    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!(r#"Package {} not found"#, INVALID_PKG)
    );
}
