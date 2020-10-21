use std::fs;
use std::path::PathBuf;

use log::Level;

use crate::log::Logger;
use crate::result::Result;

pub struct Config {
    pub log_level: Level,
    pub home_dir: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Self {
            log_level: Level::Error,
            home_dir: dirs::home_dir().unwrap().join(".huber"),
        }
    }

    pub fn init(&self) -> Result<()> {
        Logger::init(&self)
    }

    pub fn bin_dir(&self) -> Result<PathBuf> {
        let bin_dir = self.home_dir.join("bin");

        if !bin_dir.exists() || !bin_dir.is_dir() {
            fs::remove_dir_all(bin_dir.as_path())?;
            fs::create_dir_all(bin_dir.as_path())?;
        }

        Ok(bin_dir)
    }
}
