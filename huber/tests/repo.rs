use std::env;
use std::path::Path;

use scopeguard::defer;

use crate::common::reset_huber;

#[macro_use]
mod common;

#[test]
// #[sequential]
fn test_repo_add_show_remove() {
    defer! {
        reset_huber();
    }

    let huber_config = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("docs")
        .join("src")
        .join("cmd")
        .join("repo")
        .join("huber.yaml");
    let repo = "external";

    let assert = huber_cmd!(arg("repo")
        .arg("add")
        .arg(repo)
        .arg("--file")
        .arg(huber_config.to_string_lossy().to_string())
        .assert()
        .success());
    assert_contain_line_regex!(assert.get_output().stderr, &format!("Repo {} added", repo));

    huber_cmd!(arg("repo").arg("show").assert().success());

    let assert = huber_cmd!(arg("repo").arg("remove").arg(repo).assert().success());
    assert_contain_line_regex!(
        assert.get_output().stderr,
        &format!("Repo {} removed", repo)
    );
}
