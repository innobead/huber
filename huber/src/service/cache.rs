use std::env;
use std::fs::File;
use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use huber_common::model::config::{
    Config, ConfigFieldConvertTrait, ConfigPath, MANAGED_PKG_ROOT_DIR,
};
use huber_common::model::package::{Package, PackageIndex};
use huber_common::model::repo::Repository;
use log::debug;
use rayon::prelude::*;
use regex::Regex;
use simpledi_rs::di::{DIContainer, DIContainerExtTrait, DependencyInjectTrait};

use crate::error::HuberError::PackageNotFound;
use crate::github::{GithubClient, GithubClientTrait};
use crate::service::repo::{RepoAsyncTrait, RepoService, RepoTrait};
use crate::service::{ItemOperationTrait, ServiceTrait};

pub trait CacheTrait {
    fn get_package(&self, name: &str) -> anyhow::Result<Package>;
    fn get_unmanaged_package(&self, name: &str) -> anyhow::Result<Package>;
    fn list_packages(&self, pattern: &str, owner: &str) -> anyhow::Result<Vec<Package>>;
    fn list_unmanaged_packages(&self) -> anyhow::Result<Vec<Package>>;
    fn has_package(&self, name: &str) -> anyhow::Result<bool>;
    fn has_unmanaged_package(&self, name: &str) -> anyhow::Result<bool>;
    fn get_package_indexes(&self) -> anyhow::Result<Vec<PackageIndex>>;
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
        let pkg_file = config.managed_pkg_manifest_file(name)?;

        if pkg_file.exists() {
            Ok(serde_yaml::from_reader::<File, Package>(File::open(
                pkg_file,
            )?)?)
        } else {
            self.get_unmanaged_package(name)
        }
    }

    fn get_unmanaged_package(&self, name: &str) -> anyhow::Result<Package> {
        match self
            .list_unmanaged_packages()?
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
            "" => self
                .get_package_indexes()?
                .par_iter()
                .filter_map(|it: &PackageIndex| {
                    if owner.is_empty() || it.owner == owner {
                        if let Ok(p) = self.get_package(&it.name) {
                            Some(p)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect(),

            _ => {
                let regex = Regex::new(pattern)?;

                self.get_package_indexes()?
                    .par_iter()
                    .filter_map(|it: &PackageIndex| {
                        if regex.is_match(&it.name) {
                            if let Ok(p) = self.get_package(&it.name) {
                                Some(p)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect()
            }
        };

        // unmanaged packages
        pkgs.append(&mut self.list_unmanaged_packages()?);
        pkgs.sort_by(|p1, p2| p1.name.cmp(&p2.name));

        Ok(pkgs)
    }

    fn list_unmanaged_packages(&self) -> anyhow::Result<Vec<Package>> {
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

        // unmanaged
        self.has_unmanaged_package(name)
    }

    fn has_unmanaged_package(&self, name: &str) -> anyhow::Result<bool> {
        Ok(self
            .list_unmanaged_packages()?
            .iter()
            .any(|it| it.name == name))
    }

    fn get_package_indexes(&self) -> anyhow::Result<Vec<PackageIndex>> {
        let config = self.container.get::<Config>().unwrap();
        let index_file = config.managed_pkg_index_file()?;
        let pkg_indexes =
            serde_yaml::from_reader::<File, Vec<PackageIndex>>(File::open(index_file)?)?;

        Ok(pkg_indexes)
    }
}

#[async_trait]
impl CacheAsyncTrait for CacheService {
    // FIXME enhance performance
    async fn update_repositories(&self) -> anyhow::Result<()> {
        if let Ok(path) = env::var(MANAGED_PKG_ROOT_DIR) {
            debug!(
                "Bypassed updating repositories, because MANAGED_PKG_ROOT_DIR set: {}",
                path
            );
            return Ok(());
        }

        debug!("Updating repos");
        let config = self.container.get::<Config>().unwrap();

        let dir = config.huber_repo_dir()?;
        debug!("Updating {:?}", dir);

        let client = GithubClient::new(config.to_github_credentials(), config.to_github_key_path());

        debug!("Updating managed repos");
        client.clone("innobead", "huber", dir.clone()).await?;

        debug!("Updating unmanaged repos");
        let repo_service = self.container.get::<RepoService>().unwrap();
        for repo in repo_service.list()? {
            match repo.url {
                Some(ref url) => {
                    debug!("Updating {:?}", config.unmanaged_repo_dir(&repo.name)?);
                    repo_service
                        .download_save_pkgs_file_from_remote_github(&repo.name, url)
                        .await?;
                }
                _ => debug!(
                    "Failed to update unmanaged repo due to empty url: {:?}",
                    &repo
                ),
            }
        }

        Ok(())
    }
}
