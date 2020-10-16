use log::Level;
use std::path::PathBuf;
use crate::base::Logger;
use std::fs;

pub(crate) struct Config {
    pub(crate) log_level: Level,
    pub(crate) home_dir: PathBuf,
}

impl Config {
    pub(crate) fn new() -> Self {
        Self {
            log_level: Level::Error,
            home_dir: dirs::home_dir().unwrap().join(".huber"),
        }
    }

    pub(crate) fn init(&self) -> anyhow::Result<()> {
        Logger::init(&self)
    }

    pub(crate) fn bin_dir(&self) -> anyhow::Result<PathBuf> {
        let bin_dir = self.home_dir.join("bin");

        if !bin_dir.exists() || !bin_dir.is_dir() {
            fs::remove_dir_all(bin_dir.as_path())?;
            fs::create_dir_all(bin_dir.as_path())?;
        }

        Ok(bin_dir)
    }
}
