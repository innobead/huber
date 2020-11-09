use regex::Regex;
use std::cmp::Ordering;
use std::path::Path;

// https://github.com/golang/go/blob/master/src/go/build/syslist.go
const GO_OS_LIST: &str = "aix android darwin dragonfly freebsd hurd illumos ios js linux nacl netbsd openbsd plan9 solaris windows zos macos";
const GO_ARCH_LIST: &str = "386 amd64 amd64p32 arm armbe arm64 arm64be ppc64 ppc64le mips mipsle mips64 mips64le mips64p32 mips64p32le ppc riscv riscv64 s390 s390x sparc sparc64 wasm x86_64 aarch64 64bit";

pub fn is_empty_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .read_dir()
        .map(|mut it| it.next().is_none())
        .unwrap_or(false)
}

pub fn trim_os_arch(str: &str) -> String {
    let revert_sort = |x: &&str, y: &&str| -> Ordering { y.len().cmp(&x.len()) };

    let mut os_pattern = GO_OS_LIST.split(" ").collect::<Vec<&str>>();
    os_pattern.sort_by(revert_sort);
    let os_pattern = os_pattern.join("|");

    let mut arch_pattern = GO_ARCH_LIST.split(" ").collect::<Vec<&str>>();
    arch_pattern.sort_by(revert_sort);
    let arch_pattern = arch_pattern.join("|");

    let re = Regex::new(&format!(
        r"(?i)[-_]({})[-_]({})[-_]*",
        os_pattern, arch_pattern
    ))
    .unwrap();
    re.replace_all(str, "").to_string()
}

#[cfg(test)]
mod test {
    use crate::file::trim_os_arch;

    #[test]
    fn test_trim_arch_os() {
        let data = vec![
            ("name-linux-amd64", "name"),
            ("name-Linux-aarch64", "name"),
            ("name-linux-Arm64", "name"),
            ("name_Linux-64bit", "name"),
            ("name_macOS-64bit", "name"),
        ];

        for x in data {
            assert_eq!(trim_os_arch(x.0), x.1);
        }
    }
}
