use std::fs;
use std::path::{Path, PathBuf};

use log::debug;

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
    debug!("Making {:?} as executable", path);

    use std::os::windows::fs::PermissionsExt;
    let file = File::open(file_path)?;
    let mut perms = file.metadata()?.permissions();
    perms.set_readonly(false);
    fs::set_permissions(file_path, perms)?;
    Ok(())
}
