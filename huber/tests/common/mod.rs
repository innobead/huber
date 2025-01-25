#![allow(dead_code)]
#![allow(unused_macros)]

use assert_cmd::assert::Assert;

pub const HUBER_EXEC: &str = env!("CARGO_BIN_EXE_huber");
pub const INVALID_PKG: &str = "pkg_notfound";
pub const INVALID_PKG_VERSION: &str = "pkg_notfound@1.2.3";
pub const PKG_VERSION_1: &str = "k9s@v0.32.5";
pub const PKG_VERSION_2: &str = "k9s@v0.32.7";

macro_rules! huber_cmd {
    ($($body:tt)*) => {
        assert_cmd::Command::new(crate::common::HUBER_EXEC)
            .env(
                "HUBER_PKG_ROOT_DIR",
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .unwrap()
                    .join("generated-v1"),
            )
            .arg("-l")
            .arg("debug")
            .$($body)*
    };
}

macro_rules! assert_contain_line_regex {
    ($value:expr, $expect:expr) => {
        let line = String::from_utf8($value.clone()).unwrap();
        println!("Value: \n{}", line);
        println!("Expected: \n{}", $expect);
        assert!(regex::Regex::new($expect).unwrap().is_match(&line))
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
        println!("Value: \n{}", line);
        println!("Expected: \n{}", $expect);
        assert!(regex::Regex::new($expect).unwrap().is_match(&line))
    };
}

macro_rules! assert_eq_last_line {
    ($value:expr, $expect:expr) => {
        let line = String::from_utf8($value.clone())
            .unwrap()
            .lines()
            .last()
            .unwrap()
            .to_string();
        println!("Value: \n{}", line);
        println!("Expected: \n{}", $expect);
        assert_eq!(line, $expect)
    };
}

pub fn install_pkg(name: &str) -> Assert {
    huber_cmd!(arg("install").arg(name).assert().success())
}

pub fn uninstall_pkg(name: &str) -> Assert {
    huber_cmd!(arg("uninstall").arg(name).assert().success())
}

pub fn lock_pkg(name_version: &str) -> Assert {
    huber_cmd!(arg("lock").arg(name_version).assert().success())
}

pub fn update_pkg(name: &str) -> Assert {
    huber_cmd!(arg("update").arg(name).assert().success())
}

pub fn save_pkg_list(file: &str) -> Assert {
    huber_cmd!(arg("save").arg("--file").arg(file).assert().success())
}

pub fn reset_huber() -> Assert {
    huber_cmd!(arg("reset").assert().success())
}
