use std::path::Path;

use regex::Regex;

pub fn is_empty_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .read_dir()
        .map(|mut it| it.next().is_none())
        .unwrap_or(false)
}

pub fn has_suffix(s: &str) -> bool {
    if cfg!(target_os = "windows") && s.ends_with(".exe") {
        return false;
    }

    Regex::new(r".*\.\S+$").unwrap().is_match(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        assert!(is_empty_dir(dir.path()));
        assert!(!is_empty_dir("/tmp"));
    }

    #[test]
    fn test_has_suffix() {
        assert!(has_suffix("file.txt"));
        if cfg!(target_os = "windows") {
            assert!(!has_suffix("file.exe"));
        } else {
            assert!(has_suffix("file.exe"));
        }
        assert!(!has_suffix("file"));
    }
}
