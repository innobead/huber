use std::cmp::Ordering;

use regex::Regex;

// https://github.com/golang/go/blob/master/src/go/build/syslist.go
const GO_OS_LIST: &str = "aix android darwin dragonfly freebsd hurd illumos ios js linux nacl \
netbsd openbsd plan9 solaris windows zos macos osx";

const GO_ARCH_LIST: &str = "386 amd64 amd64p32 arm armbe arm64 arm64be ppc64 ppc64le mips \
mipsle mips64 mips64le mips64p32 mips64p32le ppc riscv riscv64 s390 s390x sparc sparc64 \
wasm x86_64 x64 aarch64 64bit";

pub fn trim_os_arch_version(str: &str) -> String {
    let revert_sort = |x: &&str, y: &&str| -> Ordering { y.len().cmp(&x.len()) };

    let mut os_pattern: Vec<_> = GO_OS_LIST.split(" ").collect();
    os_pattern.sort_by(revert_sort);
    let os_pattern = os_pattern.join("|");

    let mut arch_pattern: Vec<_> = GO_ARCH_LIST.split(" ").collect();
    arch_pattern.sort_by(revert_sort);
    let arch_pattern = arch_pattern.join("|");

    let patterns = [
        Regex::new(r"(?i)[-_]*v?\d+.\d+.\d+[-_]*").unwrap(),
        Regex::new(&format!(r"(?i)[-_.]*({})[-_]*", os_pattern)).unwrap(),
        Regex::new(&format!(r"(?i)[-_.]*({})[-_]*", arch_pattern)).unwrap(),
    ];

    let mut str = str.to_string();
    for pat in &patterns {
        if pat.is_match(&str) {
            str = pat.replace_all(&str, "").to_string();
        }
    }

    if cfg!(target_os = "windows") && !str.ends_with(".exe") {
        str += ".exe";
    }

    str
}

pub fn is_os_arch_match(os: &str, arch: &str, asset_url: &str) -> bool {
    let os = os.to_lowercase();
    let arch = arch.to_lowercase();
    let asset_url = asset_url.to_lowercase();

    let os_pattern = if os == "macos" {
        r"([-_.]|\b)(macos|darwin|apple|osx)([-_.]?|\b)"
    } else {
        &format!(r"([-_.]?|\b){}([-_.]?|\b)", os)
    };
    if !Regex::new(os_pattern).unwrap().is_match(&asset_url) {
        return false;
    }

    let arch_pattern = match arch.as_str() {
        "x86_64" => r"([-_.]|\b)(x86_64|x64|amd64|64bit)([-_.]?|\b)",
        "arm" => r"\b(arm|arm32|armhf|armv7)\b",
        "aarch64" => r"([-_.]|\b)(aarch64|arm64)([-_.]?|\b)",
        _ => return false,
    };

    Regex::new(arch_pattern).unwrap().is_match(&asset_url)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trim_os_arch_version() {
        let data = vec![
            ("name-linux-amd64", "name"),
            ("name-Linux-aarch64", "name"),
            ("name-linux-Arm64", "name"),
            ("name_Linux-64bit", "name"),
            ("name.Linux-64bit", "name"),
            ("name_macOS-64bit", "name"),
            ("name-v1.0.0_macOS-64bit", "name"),
            ("name-1.0.0_macOS-64bit", "name"),
            ("name_v1.0.0_macOS-64bit", "name"),
            ("name_macOS-64bit-v1.0.0", "name"),
            ("name_v1.0.0_macOS-64bit", "name"),
            ("name-v1.0.0-x86_64", "name"),
            ("name-x86_64", "name"),
            ("name-x86_64.exe", "name.exe"),
            ("name-darwin-arm64-v2.10.0", "name"),
        ];

        for x in data {
            if cfg!(target_os = "windows") {
                assert_eq!(trim_os_arch_version(x.0), "name.exe");
            } else {
                assert_eq!(trim_os_arch_version(x.0), x.1);
            }
        }
    }

    #[test]
    fn test_is_os_arch_match() {
        let data = vec![
            ("linux", "x86_64", "name-linux-amd64", true),
            ("linux", "x86_64", "name-linux-x86_64", true),
            ("linux", "x86_64", "name-linux-arm64", false),
            (
                "linux",
                "x86_64",
                "conftest_0.56.0_linux_x86_64.tar.gz",
                true,
            ),
            ("linux", "aarch64", "name-linux-aarch64", true),
            ("linux", "aarch64", "name-linux-arm64", true),
            ("linux", "aarch64", "name-linux-amd64", false),
            ("linux", "arm", "name-linux-armv7", true),
            ("linux", "arm", "name-linux-arm", true),
            ("linux", "arm", "name-linux-arm64", false),
            ("linux", "arm", "name-Linux-arm64", false),
            ("linux", "arm", "name-Linux-arm64.tar.gz", false),
            ("linux", "arm", "name-Linux-arm.tar.gz", true),
            ("windows", "x86_64", "name-windows-x86_64", true),
            ("windows", "x86_64", "name-windows-x86_64.exe", true),
            ("windows", "x86_64", "name-windows-amd64", true),
            ("windows", "x86_64", "name-windows-arm64", false),
            ("macos", "x86_64", "name-macos-amd64", true),
            ("macos", "x86_64", "name-darwin-amd64", true),
            ("macos", "x86_64", "name-macos-x86_64", true),
            ("macos", "x86_64", "name-darwin-x86_64", true),
            ("macos", "x86_64", "name-macos-arm64", false),
            ("macos", "aarch64", "name-macos-aarch64", true),
            ("macos", "aarch64", "name-darwin-aarch64", true),
            ("macos", "aarch64", "name-macos-arm64", true),
            ("macos", "aarch64", "name-darwin-arm64", true),
            ("macos", "aarch64", "name-macos-amd64", false),
        ];

        for (os, arch, url, expected) in data {
            assert_eq!(is_os_arch_match(os, arch, url), expected);
        }
    }
}
