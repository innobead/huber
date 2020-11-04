use std::fs;
use std::path::PathBuf;

use hubcaps::Credentials;
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
    pub github_credentials: Option<Credentials>,
    pub git_ssh_key: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            log_level: Level::Error,
            output_format: OutputFormat::Console,
            home_dir: dirs::home_dir().unwrap().join(".huber"),
            github_credentials: None,
            git_ssh_key: None,
        }
    }

    pub fn init(&self) -> Result<()> {
        Logger::init(&self)
    }

    pub fn huber_repo_dir(&self) -> Result<PathBuf> {
        self.dir("huber_repo")
    }

    pub fn bin_dir(&self) -> Result<PathBuf> {
        self.dir("bin")
    }

    pub fn managed_pkg_root_dir(&self) -> Result<PathBuf> {
        Ok(self.huber_repo_dir()?.join("generated"))
    }

    pub fn managed_pkg_manifest_file(&self, name: &str) -> Result<PathBuf> {
        Ok(self
            .managed_pkg_root_dir()?
            .join("packages")
            .join(name)
            .with_extension("yaml"))
    }

    pub fn managed_pkg_index_file(&self) -> Result<PathBuf> {
        Ok(self
            .managed_pkg_root_dir()?
            .join("index")
            .with_extension("yaml"))
    }

    pub fn installed_pkg_root_dir(&self) -> Result<PathBuf> {
        self.dir("installed_packages")
    }

    pub fn installed_pkg_dir(&self, pkg: &Package, version: &str) -> Result<PathBuf> {
        Ok(self
            .installed_pkg_root_dir()?
            .join(pkg.source.to_string())
            .join(format!("{}_{}", pkg.source.owner(), pkg.name))
            .join(version))
    }

    pub fn installed_pkg_bin_dir(&self, pkg: &Package, version: &str) -> Result<PathBuf> {
        Ok(self.installed_pkg_dir(pkg, version)?.join("bin"))
    }

    pub fn installed_pkg_manifest_file(&self, pkg: &Package, version: &str) -> Result<PathBuf> {
        Ok(self
            .installed_pkg_dir(pkg, version)?
            .join(&pkg.name)
            .with_extension("yaml"))
    }

    pub fn current_pkg_dir(&self, pkg: &Package) -> Result<PathBuf> {
        self.installed_pkg_dir(pkg, "current")
    }

    pub fn current_pkg_bin_dir(&self, pkg: &Package) -> Result<PathBuf> {
        Ok(self.current_pkg_dir(pkg)?.join("bin"))
    }

    pub fn current_pkg_manifest_file(&self, pkg: &Package) -> Result<PathBuf> {
        Ok(self
            .current_pkg_dir(pkg)?
            .join(&pkg.name)
            .with_extension("yaml"))
    }

    pub fn current_index_file(&self) -> Result<PathBuf> {
        Ok(self
            .installed_pkg_root_dir()?
            .join("index")
            .with_extension("yaml"))
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
