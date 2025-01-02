use std::env;
use std::fs::File;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use anyhow::anyhow;
use async_trait::async_trait;
use huber_common::model::config::{
    Config, ConfigFieldConvertTrait, ConfigPath, HUBER_PKG_ROOT_DIR,
};
use huber_common::model::package::{Package, PackageIndex};
use huber_common::model::repo::Repository;
use lazy_static::lazy_static;
use log::debug;
use rayon::prelude::*;
use regex::Regex;
use simpledi_rs::di::{DIContainer, DIContainerExtTrait, DependencyInjectTrait};

use crate::error::HuberError::PackageNotFound;
use crate::github::{GithubClient, GithubClientTrait};
use crate::service::repo::{RepoAsyncTrait, RepoService, RepoTrait};
use crate::service::{ItemOperationTrait, ServiceTrait};

lazy_static! {
    static ref managed_repo_modified_time: RwLock<Option<SystemTime>> = Default::default();
    static ref managed_pkg_indexes: RwLock<Vec<PackageIndex>> = Default::default();
}

pub trait CacheTrait {
    fn get_package(&self, name: &str) -> anyhow::Result<Package>;
    fn get_external_package(&self, name: &str) -> anyhow::Result<Package>;
    fn list_packages(&self, pattern: &str, owner: &str) -> anyhow::Result<Vec<Package>>;
    fn list_external_packages(&self) -> anyhow::Result<Vec<Package>>;
    fn has_package(&self, name: &str) -> anyhow::Result<bool>;
    fn has_external_package(&self, name: &str) -> anyhow::Result<bool>;
    fn get_package_indexes(&self) -> anyhow::Result<Vec<PackageIndex>>;

    fn refresh_package_indexes(&self) -> anyhow::Result<()>;
}

#[async_trait]
pub trait CacheAsyncTrait {
    async fn update_repositories(&self) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct CacheService {
    pub container: Option<Arc<DIContainer>>,
}

unsafe impl Send for CacheService {}

unsafe impl Sync for CacheService {}

impl ServiceTrait for CacheService {}

impl DependencyInjectTrait for CacheService {
    fn inject(&mut self, container: Arc<DIContainer>) {
        self.container = Some(container);
    }
}

impl Default for CacheService {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheService {
    pub fn new() -> Self {
        Self { container: None }
    }
}

impl CacheTrait for CacheService {
    fn get_package(&self, name: &str) -> anyhow::Result<Package> {
        if !self.has_package(name)? {
            return Err(anyhow!(PackageNotFound(name.into())));
        }

        let config = self.container.get::<Config>().unwrap();
        let pkg_file = config.pkg_manifest_file(name)?;

        if pkg_file.exists() {
            Ok(serde_yaml::from_reader::<File, Package>(File::open(
                pkg_file,
            )?)?)
        } else {
            self.get_external_package(name)
        }
    }

    fn get_external_package(&self, name: &str) -> anyhow::Result<Package> {
        match self
            .list_external_packages()?
            .into_iter()
            .find(|it| it.name == name)
        {
            None => Err(anyhow!(PackageNotFound(name.into()))),
            Some(pkg) => Ok(pkg),
        }
    }

    fn list_packages(&self, pattern: &str, owner: &str) -> anyhow::Result<Vec<Package>> {
        // managed packages
        let mut pkgs: Vec<Package> = match pattern {
            "" => {
                let indexes: Vec<_> = self.get_package_indexes()?.into_par_iter().collect();
                indexes
                    .into_iter()
                    .filter_map(|it| {
                        if owner.is_empty() || it.owner == owner {
                            self.get_package(&it.name)
                                .map_err(|err| {
                                    debug!("{}", err);
                                    err
                                })
                                .ok()
                        } else {
                            None
                        }
                    })
                    .collect()
            }

            _ => {
                let regex = Regex::new(pattern)?;
                let indexes: Vec<_> = self.get_package_indexes()?.into_par_iter().collect();
                indexes
                    .into_iter()
                    .filter_map(|it| {
                        if regex.is_match(&it.name) {
                            self.get_package(&it.name)
                                .map_err(|err| {
                                    debug!("{}", err);
                                    err
                                })
                                .ok()
                        } else {
                            None
                        }
                    })
                    .collect()
            }
        };

        // external packages
        pkgs.append(&mut self.list_external_packages()?);
        pkgs.sort_by(|p1, p2| p1.name.cmp(&p2.name));

        Ok(pkgs)
    }

    fn list_external_packages(&self) -> anyhow::Result<Vec<Package>> {
        let repo_service = self.container.get::<RepoService>().unwrap();

        let repos = repo_service.list()?;
        let pkgs: Vec<Package> = repos
            .par_iter()
            .filter_map(|it: &Repository| {
                if let Ok(p) = repo_service.get_packages_by_repo(&it.name) {
                    Some(p)
                } else {
                    None
                }
            })
            .flat_map(|it| it)
            .collect();

        Ok(pkgs)
    }

    fn has_package(&self, name: &str) -> anyhow::Result<bool> {
        // managed
        if self.get_package_indexes()?.iter().any(|it| it.name == name) {
            return Ok(true);
        }

        // external
        self.has_external_package(name)
    }

    fn has_external_package(&self, name: &str) -> anyhow::Result<bool> {
        Ok(self
            .list_external_packages()?
            .iter()
            .any(|it| it.name == name))
    }

    fn get_package_indexes(&self) -> anyhow::Result<Vec<PackageIndex>> {
        self.refresh_package_indexes()?;

        Ok(managed_pkg_indexes
            .read()
            .map_err(|e| anyhow!("{}", e))?
            .clone())
    }

    fn refresh_package_indexes(&self) -> anyhow::Result<()> {
        let config = self.container.get::<Config>().unwrap();
        let index_file = config.pkg_index_file()?;

        let time = File::open(&index_file)?.metadata()?.modified()?;
        let modified_time = *managed_repo_modified_time
            .read()
            .map_err(|e| anyhow!("{}", e))?;
        if modified_time.is_some() && modified_time.unwrap() == time {
            return Ok(());
        }

        managed_repo_modified_time
            .write()
            .map_err(|e| anyhow!("{}", e))?
            .replace(time);
        *managed_pkg_indexes.write().map_err(|e| anyhow!("{}", e))? =
            serde_yaml::from_reader::<File, Vec<PackageIndex>>(File::open(index_file)?)?;

        Ok(())
    }
}

#[async_trait]
impl CacheAsyncTrait for CacheService {
    // FIXME enhance performance
    async fn update_repositories(&self) -> anyhow::Result<()> {
        let config = self.container.get::<Config>().unwrap();

        if let Ok(path) = env::var(HUBER_PKG_ROOT_DIR) {
            debug!(
                "Bypassed updating repositories, because {} is set to {}",
                HUBER_PKG_ROOT_DIR, path
            );
        } else {
            debug!("Updating huber repo");
            let dir = config.huber_repo_dir()?;

            debug!("Updating {:?}", dir);
            let client =
                GithubClient::new(config.to_github_credentials(), config.to_github_key_path());
            client.clone("innobead", "huber", dir).await?;
        }

        debug!("Updating external repos");
        let repo_service = self.container.get::<RepoService>().unwrap();
        for repo in repo_service.list()? {
            if let Some(url) = repo.url {
                debug!("Updating {:?}", config.external_repo_dir(&repo.name)?);
                repo_service
                    .download_save_pkgs_file_from_remote_github(&repo.name, &url)
                    .await?;
            } else if let Some(file) = repo.file {
                debug!("Updating {:?}", config.external_repo_dir(&repo.name)?);
                repo_service
                    .download_save_pkgs_file_from_local(&repo.name, &file)
                    .await?;
            } else {
                debug!(
                    "Failed to update external repos due to empty url and file: {:?}",
                    &repo
                );
            }
        }

        Ok(())
    }
}
