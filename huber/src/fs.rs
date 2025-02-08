use std::fs;
use std::path::{Path, PathBuf};

use log::debug;
use regex::Regex;

pub fn dir(dir: PathBuf) -> anyhow::Result<PathBuf> {
    if !dir.exists() {
        let _ = fs::remove_dir_all(dir.as_path());
        fs::create_dir_all(dir.as_path())?;
    }

    Ok(dir)
}

#[cfg(not(target_os = "windows"))]
pub fn set_executable_permission(path: &Path) -> anyhow::Result<()> {
    debug!("Making {:?} as executable", path);

    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn set_executable_permission(path: &Path) -> anyhow::Result<()> {
    debug!("Unsupported making {:?} as executable on Windows", path);
    Ok(())
}

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
    use crate::fs::{has_suffix, is_empty_dir};

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
