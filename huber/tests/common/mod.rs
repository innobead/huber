#![allow(dead_code)]
#![allow(unused_macros)]

use std::path::Path;

use assert_cmd::assert::Assert;
use assert_cmd::Command;

pub const HUBER_EXEC: &str = env!("CARGO_BIN_EXE_huber");
pub const INVALID_PKG: &str = "pkg_notfound";
pub const INVALID_PKG_VERSION: &str = "pkg_notfound@1.2.3";
pub const PKG_VERSION_1: &str = "k9s@0.32.5";
pub const PKG_VERSION_1_ASSERT: &str = "k9s@v0.32.5";
pub const PKG_VERSION_2: &str = "k9s@0.32.7";
pub const PKG_VERSION_2_ASSERT: &str = "k9s@v0.32.7";

pub fn install_pkg(name: &str) -> Assert {
    Command::new(HUBER_EXEC)
        .arg("install")
        .arg(name)
        .env(
            "huber_pkg_root_dir",
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
            "huber_pkg_root_dir",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .success()
}

pub fn lock_pkg(name_version: &str) -> Assert {
    Command::new(HUBER_EXEC)
        .arg("lock")
        .arg(name_version)
        .env(
            "huber_pkg_root_dir",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .success()
}

pub fn update_pkg(name: &str) -> Assert {
    Command::new(HUBER_EXEC)
        .arg("update")
        .arg(name)
        .env(
            "huber_pkg_root_dir",
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
            "huber_pkg_root_dir",
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
            "huber_pkg_root_dir",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("generated"),
        )
        .assert()
        .success()
}

macro_rules! assert_eq_last_line {
    ($value:expr, $expect:expr) => {
        let line = String::from_utf8($value.clone())
            .unwrap()
            .lines()
            .last()
            .unwrap()
            .to_string();

        assert_eq!(line, $expect)
    };
}

macro_rules! assert_eq_last_line_regex {
    ($value:expr, $expect:expr) => {
        let line = String::from_utf8($value.clone())
            .unwrap()
            .lines()
            .last()
            .unwrap()
            .to_string();

        assert!(regex::Regex::new($expect).unwrap().is_match(&line))
    };
}

macro_rules! huber_cmd {
    ($($body:tt)*) => {
        assert_cmd::Command::new(crate::common::HUBER_EXEC)
            .env(
                "huber_pkg_root_dir",
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .unwrap()
                    .join("generated"),
            )
            .$($body)*
    };
}
