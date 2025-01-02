#[macro_use]
mod common;

#[test]
fn test_completions() {
    huber_cmd!(arg("completions").arg("zsh").assert().success());
}
