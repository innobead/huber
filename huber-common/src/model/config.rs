use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::PathBuf;

use libcli_rs::output::OutputFormat;
use log::LevelFilter;
use octocrab::auth::Auth;
use serde::{Deserialize, Serialize};

use crate::fs::dir;
use crate::model::package::Package;

pub const MANAGED_PKG_ROOT_DIR: &str = "MANAGED_PKG_ROOT_DIR"; // generated directory

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub log_level: String,
    pub output_format: OutputFormat,
    pub huber_dir: PathBuf,
    pub github_token: Option<String>,
    pub github_key: Option<String>,
    pub github_base_uri: Option<String>,
    pub lock_pkg_versions: HashMap<String, Vec<String>>,
}

impl Config {
    pub fn new(
        log_level: String,
        output_format: OutputFormat,
        huber_dir: PathBuf,
        github_token: Option<String>,
        github_key: Option<String>,
        github_base_uri: Option<String>,
        lock_pkg_versions: HashMap<String, Vec<String>>,
    ) -> Self {
        Self {
            log_level,
            output_format,
            huber_dir: dir(huber_dir).unwrap(),
            github_token,
            github_key,
            github_base_uri,
            lock_pkg_versions,
        }
    }
}

pub trait ConfigPath {
    fn lock_file(&self) -> anyhow::Result<PathBuf>;
    fn config_file(&self) -> anyhow::Result<PathBuf>;

    fn bin_dir(&self) -> anyhow::Result<PathBuf>;
    fn temp_dir(&self) -> anyhow::Result<PathBuf>;
    fn repo_root_dir(&self) -> anyhow::Result<PathBuf>;
    fn huber_repo_dir(&self) -> anyhow::Result<PathBuf>;

    fn unmanaged_repo_dir(&self, name: &str) -> anyhow::Result<PathBuf>;
    fn unmanaged_repo_file(&self, name: &str) -> anyhow::Result<PathBuf>;
    fn unmanaged_repo_pkgs_file(&self, name: &str) -> anyhow::Result<PathBuf>;

    fn managed_pkg_root_dir(&self) -> anyhow::Result<PathBuf>;
    fn managed_pkg_manifest_file(&self, name: &str) -> anyhow::Result<PathBuf>;
    fn managed_pkg_index_file(&self) -> anyhow::Result<PathBuf>;

    fn installed_pkg_root_dir(&self) -> anyhow::Result<PathBuf>;
    fn installed_pkg_base_dir(&self, pkg: &Package) -> anyhow::Result<PathBuf>;
    fn installed_pkg_dir(&self, pkg: &Package, version: &str) -> anyhow::Result<PathBuf>;
    fn installed_pkg_bin_dir(&self, pkg: &Package, version: &str) -> anyhow::Result<PathBuf>;
    fn installed_pkg_manifest_file(&self, pkg: &Package, version: &str) -> anyhow::Result<PathBuf>;

    fn current_pkg_dir(&self, pkg: &Package) -> anyhow::Result<PathBuf>;
    fn current_pkg_bin_dir(&self, pkg: &Package) -> anyhow::Result<PathBuf>;
    fn current_pkg_manifest_file(&self, pkg: &Package) -> anyhow::Result<PathBuf>;
    fn current_index_file(&self) -> anyhow::Result<PathBuf>;
}

impl Default for Config {
    fn default() -> Self {
        let default_config = Self {
            log_level: LevelFilter::Off.to_string(),
            output_format: OutputFormat::Console,
            huber_dir: dirs::home_dir().unwrap().join(".huber"),
            github_token: None,
            github_key: None,
            github_base_uri: Some("https://api.github.com".to_string()),
            lock_pkg_versions: Default::default(),
        };

        let config_path = default_config.config_file().unwrap();
        if config_path.exists() {
            Config::from(config_path)
        } else {
            default_config
        }
    }
}

impl From<PathBuf> for Config {
    fn from(p: PathBuf) -> Self {
        let f = File::open(&p).unwrap();
        serde_yaml::from_reader(f).unwrap()
    }
}

pub trait ConfigFieldConvertTrait {
    fn to_github_credentials(&self) -> Auth;
    fn to_github_key_path(&self) -> Option<PathBuf>;
}

impl ConfigFieldConvertTrait for Config {
    fn to_github_credentials(&self) -> Auth {
        if let Some(token) = self.github_token.clone() {
            Auth::PersonalToken(token.into())
        } else {
            Auth::None
        }
    }

    fn to_github_key_path(&self) -> Option<PathBuf> {
        self.github_key.clone().map(PathBuf::from)
    }
}

impl ConfigPath for Config {
    fn lock_file(&self) -> anyhow::Result<PathBuf> {
        let f = self.huber_dir.join("lock");
        if !f.exists() {
            File::create(f.as_path())?;
        }
        Ok(self.huber_dir.join("lock"))
    }

    fn config_file(&self) -> anyhow::Result<PathBuf> {
        Ok(self.huber_dir.join("config.yaml"))
    }

    fn bin_dir(&self) -> anyhow::Result<PathBuf> {
        dir(self.huber_dir.join("bin"))
    }

    fn temp_dir(&self) -> anyhow::Result<PathBuf> {
        dir(env::temp_dir().join("huber"))
    }

    fn repo_root_dir(&self) -> anyhow::Result<PathBuf> {
        dir(self.huber_dir.join("repos"))
    }

    fn huber_repo_dir(&self) -> anyhow::Result<PathBuf> {
        dir(self.repo_root_dir()?.join("huber"))
    }

    fn unmanaged_repo_dir(&self, name: &str) -> anyhow::Result<PathBuf> {
        dir(self.repo_root_dir()?.join(name))
    }

    fn unmanaged_repo_file(&self, name: &str) -> anyhow::Result<PathBuf> {
        Ok(self.unmanaged_repo_dir(name)?.join("repo.yaml"))
    }

    fn unmanaged_repo_pkgs_file(&self, name: &str) -> anyhow::Result<PathBuf> {
        Ok(self.unmanaged_repo_dir(name)?.join("huber.yaml"))
    }

    fn managed_pkg_root_dir(&self) -> anyhow::Result<PathBuf> {
        if let Ok(path) = env::var(MANAGED_PKG_ROOT_DIR) {
            dir(PathBuf::from(path))
        } else {
            dir(self.huber_repo_dir()?.join("generated"))
        }
    }

    fn managed_pkg_manifest_file(&self, name: &str) -> anyhow::Result<PathBuf> {
        Ok(self
            .managed_pkg_root_dir()?
            .join("packages")
            .join(name)
            .with_extension("yaml"))
    }

    fn managed_pkg_index_file(&self) -> anyhow::Result<PathBuf> {
        Ok(self
            .managed_pkg_root_dir()?
            .join("index")
            .with_extension("yaml"))
    }

    fn installed_pkg_root_dir(&self) -> anyhow::Result<PathBuf> {
        dir(self.huber_dir.join("packages"))
    }

    fn installed_pkg_base_dir(&self, pkg: &Package) -> anyhow::Result<PathBuf> {
        dir(self
            .installed_pkg_root_dir()?
            .join(pkg.source.to_string())
            .join(format!("{}_{}", pkg.source.owner(), pkg.name)))
    }

    fn installed_pkg_dir(&self, pkg: &Package, version: &str) -> anyhow::Result<PathBuf> {
        let version = pkg.parse_version_from_tag_name(&version.to_string())?;
        dir(self.installed_pkg_base_dir(pkg)?.join(version))
    }

    fn installed_pkg_bin_dir(&self, pkg: &Package, version: &str) -> anyhow::Result<PathBuf> {
        let version = pkg.parse_version_from_tag_name(&version.to_string())?;
        dir(self.installed_pkg_dir(pkg, &version)?.join("bin"))
    }

    fn installed_pkg_manifest_file(&self, pkg: &Package, version: &str) -> anyhow::Result<PathBuf> {
        let version = pkg.parse_version_from_tag_name(&version.to_string())?;

        Ok(self
            .installed_pkg_dir(pkg, &version)?
            .join(&pkg.name)
            .with_extension("yaml"))
    }

    fn current_pkg_dir(&self, pkg: &Package) -> anyhow::Result<PathBuf> {
        Ok(self.installed_pkg_base_dir(pkg)?.join("current"))
    }

    fn current_pkg_bin_dir(&self, pkg: &Package) -> anyhow::Result<PathBuf> {
        Ok(self.current_pkg_dir(pkg)?.join("bin"))
    }

    fn current_pkg_manifest_file(&self, pkg: &Package) -> anyhow::Result<PathBuf> {
        Ok(self
            .current_pkg_dir(pkg)?
            .join(&pkg.name)
            .with_extension("yaml"))
    }

    fn current_index_file(&self) -> anyhow::Result<PathBuf> {
        Ok(self
            .installed_pkg_root_dir()?
            .join("index")
            .with_extension("yaml"))
    }
}
