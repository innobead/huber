use std::path::Path;

use assert_cmd::assert::Assert;
use assert_cmd::Command;

pub const HUBER_EXEC: &str = env!("CARGO_BIN_EXE_huber");
pub fn install_pkg(name: &str) -> Assert {
    Command::new(HUBER_EXEC)
        .arg("install")
        .arg(name)
        .env(
            "MANAGED_PKG_ROOT_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .success()
}

pub fn uninstall_pkg(name: &str) -> Assert {
    Command::new(HUBER_EXEC)
        .arg("uninstall")
        .arg(name)
        .env(
            "MANAGED_PKG_ROOT_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .success()
}

pub fn save_pkg_list(file: &str) -> Assert {
    Command::new(HUBER_EXEC)
        .arg("save")
        .arg("--file")
        .arg(file)
        .env(
            "MANAGED_PKG_ROOT_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .success()
}

pub fn reset_huber() -> Assert {
    Command::new(HUBER_EXEC)
        .arg("reset")
        .env(
            "MANAGED_PKG_ROOT_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .success()
}

macro_rules! assert_eq_last_line {
    ($arr:expr, $str:expr) => {
        let line = String::from_utf8($arr.clone())
            .unwrap()
            .lines()
            .last()
            .unwrap()
            .to_string();

        assert_eq!(line, $str)
    };
}
