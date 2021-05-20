use std::fs::File;
use std::path::PathBuf;
use std::{env, fs};

use hubcaps_ex::Credentials;
use log::LevelFilter;
use serde::{Deserialize, Serialize};

use crate::log::Logger;
use crate::model::package::Package;
use crate::output::OutputFormat;
use crate::result::Result;

pub const MANAGED_PKG_ROOT_DIR: &str = "MANAGED_PKG_ROOT_DIR"; // generated directory

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub log_level: String,
    pub output_format: OutputFormat,
    pub home_dir: PathBuf,
    pub github_token: Option<String>,
    pub github_key: Option<String>,
}

pub trait ConfigPath {
    fn lock_file(&self) -> Result<PathBuf>;
    fn config_file(&self) -> Result<PathBuf>;

    fn bin_dir(&self) -> Result<PathBuf>;
    fn temp_dir(&self) -> Result<PathBuf>;
    fn repo_root_dir(&self) -> Result<PathBuf>;
    fn huber_repo_dir(&self) -> Result<PathBuf>;

    fn unmanaged_repo_dir(&self, name: &str) -> Result<PathBuf>;
    fn unmanaged_repo_file(&self, name: &str) -> Result<PathBuf>;
    fn unmanaged_repo_pkgs_file(&self, name: &str) -> Result<PathBuf>;

    fn managed_pkg_root_dir(&self) -> Result<PathBuf>;
    fn managed_pkg_manifest_file(&self, name: &str) -> Result<PathBuf>;
    fn managed_pkg_index_file(&self) -> Result<PathBuf>;

    fn installed_pkg_root_dir(&self) -> Result<PathBuf>;
    fn installed_pkg_base_dir(&self, pkg: &Package) -> Result<PathBuf>;
    fn installed_pkg_dir(&self, pkg: &Package, version: &str) -> Result<PathBuf>;
    fn installed_pkg_bin_dir(&self, pkg: &Package, version: &str) -> Result<PathBuf>;
    fn installed_pkg_manifest_file(&self, pkg: &Package, version: &str) -> Result<PathBuf>;

    fn current_pkg_dir(&self, pkg: &Package) -> Result<PathBuf>;
    fn current_pkg_bin_dir(&self, pkg: &Package) -> Result<PathBuf>;
    fn current_pkg_manifest_file(&self, pkg: &Package) -> Result<PathBuf>;
    fn current_index_file(&self) -> Result<PathBuf>;
}

impl Config {
    pub fn new() -> Self {
        let default_config = Self {
            log_level: LevelFilter::Off.to_string(),
            output_format: OutputFormat::Console,
            home_dir: dirs::home_dir().unwrap().join(".huber"),
            github_token: None,
            github_key: None,
        };

        let p = default_config.config_file().unwrap();
        if p.exists() {
            Config::from(p)
        } else {
            default_config
        }
    }

    pub fn init(&self) -> Result<()> {
        Logger::init(&self)
    }
}

impl From<PathBuf> for Config {
    fn from(p: PathBuf) -> Self {
        let f = File::open(&p).unwrap();
        serde_yaml::from_reader(f).unwrap()
    }
}

pub trait ConfigFieldConvertTrait {
    fn to_github_credentials(&self) -> Option<Credentials>;
    fn to_github_key_path(&self) -> Option<PathBuf>;
}

impl ConfigFieldConvertTrait for Config {
    fn to_github_credentials(&self) -> Option<Credentials> {
        if let Some(token) = self.github_token.clone() {
            Some(Credentials::Token(token))
        } else {
            None
        }
    }

    fn to_github_key_path(&self) -> Option<PathBuf> {
        if let Some(path) = self.github_key.clone() {
            Some(PathBuf::from(path))
        } else {
            None
        }
    }
}

impl ConfigPath for Config {
    fn lock_file(&self) -> Result<PathBuf> {
        Ok(self.home_dir.join("lock"))
    }

    fn config_file(&self) -> Result<PathBuf> {
        Ok(self.home_dir.join("config.yaml"))
    }

    fn bin_dir(&self) -> Result<PathBuf> {
        dir(self.home_dir.join("bin"))
    }

    fn temp_dir(&self) -> Result<PathBuf> {
        dir(env::temp_dir().join("huber"))
    }

    fn repo_root_dir(&self) -> Result<PathBuf> {
        dir(self.home_dir.join("repos"))
    }

    fn huber_repo_dir(&self) -> Result<PathBuf> {
        dir(self.repo_root_dir()?.join("huber"))
    }

    fn unmanaged_repo_dir(&self, name: &str) -> Result<PathBuf> {
        dir(self.repo_root_dir()?.join(name))
    }

    fn unmanaged_repo_file(&self, name: &str) -> Result<PathBuf> {
        Ok(self.unmanaged_repo_dir(name)?.join("repo.yaml"))
    }

    fn unmanaged_repo_pkgs_file(&self, name: &str) -> Result<PathBuf> {
        Ok(self.unmanaged_repo_dir(name)?.join("huber.yaml"))
    }

    fn managed_pkg_root_dir(&self) -> Result<PathBuf> {
        if let Ok(path) = env::var(MANAGED_PKG_ROOT_DIR) {
            dir(PathBuf::from(path))
        } else {
            dir(self.huber_repo_dir()?.join("generated"))
        }
    }

    fn managed_pkg_manifest_file(&self, name: &str) -> Result<PathBuf> {
        Ok(self
            .managed_pkg_root_dir()?
            .join("packages")
            .join(name)
            .with_extension("yaml"))
    }

    fn managed_pkg_index_file(&self) -> Result<PathBuf> {
        Ok(self
            .managed_pkg_root_dir()?
            .join("index")
            .with_extension("yaml"))
    }

    fn installed_pkg_root_dir(&self) -> Result<PathBuf> {
        dir(self.home_dir.join("packages"))
    }

    fn installed_pkg_base_dir(&self, pkg: &Package) -> Result<PathBuf> {
        dir(self
            .installed_pkg_root_dir()?
            .join(pkg.source.to_string())
            .join(format!("{}_{}", pkg.source.owner(), pkg.name)))
    }

    fn installed_pkg_dir(&self, pkg: &Package, version: &str) -> Result<PathBuf> {
        let version = pkg.parse_version_from_tag_name(&version.to_string())?;

        dir(self.installed_pkg_base_dir(&pkg)?.join(version))
    }

    fn installed_pkg_bin_dir(&self, pkg: &Package, version: &str) -> Result<PathBuf> {
        let version = pkg.parse_version_from_tag_name(&version.to_string())?;

        dir(self.installed_pkg_dir(pkg, &version)?.join("bin"))
    }

    fn installed_pkg_manifest_file(&self, pkg: &Package, version: &str) -> Result<PathBuf> {
        let version = pkg.parse_version_from_tag_name(&version.to_string())?;

        Ok(self
            .installed_pkg_dir(pkg, &version)?
            .join(&pkg.name)
            .with_extension("yaml"))
    }

    fn current_pkg_dir(&self, pkg: &Package) -> Result<PathBuf> {
        Ok(self.installed_pkg_base_dir(&pkg)?.join("current"))
    }

    fn current_pkg_bin_dir(&self, pkg: &Package) -> Result<PathBuf> {
        Ok(self.current_pkg_dir(pkg)?.join("bin"))
    }

    fn current_pkg_manifest_file(&self, pkg: &Package) -> Result<PathBuf> {
        Ok(self
            .current_pkg_dir(pkg)?
            .join(&pkg.name)
            .with_extension("yaml"))
    }

    fn current_index_file(&self) -> Result<PathBuf> {
        Ok(self
            .installed_pkg_root_dir()?
            .join("index")
            .with_extension("yaml"))
    }
}

fn dir(dir: PathBuf) -> Result<PathBuf> {
    if !dir.exists() {
        let _ = fs::remove_dir_all(dir.as_path());
        fs::create_dir_all(dir.as_path())?;
    }

    Ok(dir)
}
