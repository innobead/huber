use scopeguard::defer;
use sequential_test::sequential;
use tempfile::tempdir;

#[macro_use]
mod common;

use crate::common::reset_huber;

#[test]
fn test_config_not_found() {
    defer! {
        reset_huber();
    }

    huber_cmd!(arg("config").arg("show").assert().failure());
}

#[test]
#[sequential]
fn test_config_save_and_found() {
    defer! {
        reset_huber();
    }

    let github_token = "token";
    let github_base_uri = "uri";
    let github_key = "key";
    let log_level = "trace";
    let huber_dir = tempdir().unwrap();

    huber_cmd!(arg("config")
        .arg("save")
        .arg("--github-token")
        .arg(github_token)
        .arg("--github-key")
        .arg(github_key)
        .arg("--github-base-uri")
        .arg(github_base_uri)
        .arg("--log-level")
        .arg(log_level)
        .arg("--huber-dir")
        .arg(huber_dir.path())
        .arg("--output-format")
        .arg("yaml")
        .assert()
        .success());

    huber_cmd!(arg("config")
        .arg("show")
        .arg("--huber-dir")
        .arg(huber_dir.path())
        .assert()
        .success());
}
