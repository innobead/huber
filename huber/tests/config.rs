use assert_cmd::Command;
use tempfile::tempdir;

mod common;

use common::HUBER_EXEC;

#[test]
fn test_config_not_found() {
    Command::new(HUBER_EXEC)
        .arg("config")
        .arg("show")
        .assert()
        .failure()
        .stderr(
            "[WARN ] Config not found, please run `huber config save` to create a new one \
            if want to persist the configuration\n",
        );
}

#[test]
fn test_config_save_and_found() {
    let github_token = "token";
    let github_base_uri = "uri";
    let github_key = "key";
    let log_level = "trace";
    let huber_dir = tempdir().unwrap();

    Command::new(HUBER_EXEC)
        .arg("config")
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
        .success();

    Command::new(HUBER_EXEC)
        .arg("config")
        .arg("show")
        .arg("--huber-dir")
        .arg(huber_dir.path())
        .assert()
        .success();
}
