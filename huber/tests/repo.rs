use std::env;
use std::path::Path;

use scopeguard::defer;
use sequential_test::sequential;
use crate::common::reset_huber;

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_repo_add_show_remove() {
    defer! {
        reset_huber();
    }

    let huber_config = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples")
        .join("huber.yaml");
    let repo = "external";

    let assert = huber_cmd!(arg("repo")
        .arg("add")
        .arg(repo)
        .arg("--file")
        .arg(huber_config.to_string_lossy().to_string())
        .assert()
        .success());
    assert_eq_last_line!(assert.get_output().stderr, "[INFO ] Repo added");

    huber_cmd!(arg("repo").arg("show").assert().success());

    let assert = huber_cmd!(arg("repo").arg("remove").arg(repo).assert().success());
    assert_eq_last_line!(assert.get_output().stderr, "[INFO ] Repo removed");
}
