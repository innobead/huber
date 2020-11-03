use std::fs;
use std::path::PathBuf;

use log::Level;

use crate::log::Logger;
use crate::model::package::Package;
use crate::output::OutputFormat;
use crate::result::Result;

pub const HUBER_REPO: &str = "https://github.com/innobead/huber";

#[derive(Debug)]
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

    pub fn huber_repo_dir(&self) -> Result<PathBuf> {
        self.dir("huber_repo")
    }

    pub fn installed_pkg_dir(&self, pkg: &Package, version: &str) -> Result<PathBuf> {
        Ok(self
            .dir("installed_packages")?
            .join(pkg.source.to_string())
            .join(format!("{}_{}", pkg.source.owner(), pkg.name))
            .join(version))
    }

    pub fn installed_pkg_bin_dir(&self, pkg: &Package, version: &str) -> Result<PathBuf> {
        Ok(self.installed_pkg_dir(pkg, version)?.join("bin"))
    }

    pub fn current_pkg_dir(&self, pkg: &Package, _version: &str) -> Result<PathBuf> {
        self.installed_pkg_dir(pkg, "current")
    }

    fn dir(&self, path: &str) -> Result<PathBuf> {
        let dir = self.home_dir.join(path);

        if !dir.exists() || !dir.is_dir() {
            let _ = fs::remove_dir_all(dir.as_path());
            fs::create_dir_all(dir.as_path())?;
        }

        Ok(dir)
    }
}
