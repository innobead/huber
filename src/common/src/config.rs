use std::fs;
use std::path::PathBuf;

use log::Level;

use crate::log::Logger;
use crate::output::OutputFormat;
use crate::result::Result;

pub struct Config {
    pub log_level: Level,
    pub output_format: OutputFormat,
    pub home_dir: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Self {
            log_level: Level::Error,
            output_format: OutputFormat::Console,
            home_dir: dirs::home_dir().unwrap().join(".huber"),
        }
    }

    pub fn init(&self) -> Result<()> {
        Logger::init(&self)
    }

    pub fn bin_dir(&self) -> Result<PathBuf> {
        self.dir("bin")
    }

    pub fn github_dir(&self) -> Result<PathBuf> {
        self.dir("github")
    }

    fn dir(&self, path: &str) -> Result<PathBuf> {
        let dir = self.home_dir.join(path);

        if !dir.exists() || !dir.is_dir() {
            fs::remove_dir_all(dir.as_path())?;
            fs::create_dir_all(dir.as_path())?;
        }

        Ok(dir)
    }
}
