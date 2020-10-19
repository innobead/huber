use std::fs;
use std::path::PathBuf;

use log::Level;

use crate::base::log::Logger;
use crate::base::result::Result;

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

    pub(crate) fn init(&self) -> Result<()> {
        Logger::init(&self)
    }

    pub(crate) fn bin_dir(&self) -> Result<PathBuf> {
        let bin_dir = self.home_dir.join("bin");

        if !bin_dir.exists() || !bin_dir.is_dir() {
            fs::remove_dir_all(bin_dir.as_path())?;
            fs::create_dir_all(bin_dir.as_path())?;
        }

        Ok(bin_dir)
    }
}
