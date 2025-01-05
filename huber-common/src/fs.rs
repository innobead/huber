use std::fs;
use std::path::PathBuf;

pub fn dir(dir: PathBuf) -> anyhow::Result<PathBuf> {
    if !dir.exists() {
        let _ = fs::remove_dir_all(dir.as_path());
        fs::create_dir_all(dir.as_path())?;
    }

    Ok(dir)
}
