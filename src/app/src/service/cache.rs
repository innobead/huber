use std::fs::File;
use std::sync::Arc;

use async_trait::async_trait;
use log::info;
use rayon::prelude::*;
use regex::Regex;

use huber_common::di::DIContainer;
use huber_common::model::config::{Config, ConfigFieldConvertTrait, ConfigPath};
use huber_common::model::package::{Package, PackageIndex};
use huber_common::model::repo::Repository;
use huber_common::result::Result;

use crate::component::github::{GithubClient, GithubClientTrait};
use crate::service::repo::{RepoAsyncTrait, RepoService, RepoTrait};
use crate::service::{ItemOperationTrait, ServiceTrait};

pub(crate) trait CacheTrait {
    fn get_package(&self, name: &str) -> Result<Package>;
    fn get_unmanaged_package(&self, name: &str) -> Result<Package>;
    fn list_packages(&self, pattern: &str, owner: &str) -> Result<Vec<Package>>;
    fn list_unmanaged_packages(&self) -> Result<Vec<Package>>;
    fn has_package(&self, name: &str) -> Result<bool>;
    fn has_unmanaged_package(&self, name: &str) -> Result<bool>;
    fn get_package_indexes(&self) -> Result<Vec<PackageIndex>>;
}

#[async_trait]
pub(crate) trait CacheAsyncTrait {
    async fn update_repositories(&self) -> Result<()>;
}

#[derive(Debug)]
pub(crate) struct CacheInfo {
    location: String,
}

#[derive(Debug)]
pub(crate) struct CacheService {
    pub(crate) config: Option<Arc<Config>>,
    pub(crate) container: Option<Arc<DIContainer>>,
}

unsafe impl Send for CacheService {}

unsafe impl Sync for CacheService {}

impl ServiceTrait for CacheService {
    fn set_shared_properties(&mut self, config: Arc<Config>, container: Arc<DIContainer>) {
        self.config = Some(config);
        self.container = Some(container);
    }
}

impl CacheService {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            container: None,
        }
    }
}

impl CacheTrait for CacheService {
    fn get_package(&self, name: &str) -> Result<Package> {
        if !self.has_package(name)? {
            return Err(anyhow!("{} not found", name));
        }

        let config = self.config.as_ref().unwrap();
        let pkg_file = config.managed_pkg_manifest_file(name)?;

        if pkg_file.exists() {
            Ok(serde_yaml::from_reader::<File, Package>(File::open(
                pkg_file,
            )?)?)
        } else {
            self.get_unmanaged_package(name)
        }
    }

    fn get_unmanaged_package(&self, name: &str) -> Result<Package> {
        match self
            .list_unmanaged_packages()?
            .into_iter()
            .find(|it| it.name == name)
        {
            None => Err(anyhow!("{} not found", name)),
            Some(pkg) => Ok(pkg),
        }
    }

    fn list_packages(&self, pattern: &str, owner: &str) -> Result<Vec<Package>> {
        // managed packages
        let mut pkgs: Vec<Package> = match pattern {
            "" => self
                .get_package_indexes()?
                .par_iter()
                .filter_map(|it: &PackageIndex| {
                    if owner == "" || it.owner == owner {
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
                    .iter()
                    .filter_map(|it| {
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

    fn list_unmanaged_packages(&self) -> Result<Vec<Package>> {
        let container = self.container.as_ref().unwrap();
        let repo_service = container.get::<RepoService>().unwrap();

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

    fn has_package(&self, name: &str) -> Result<bool> {
        // managed
        if self.get_package_indexes()?.iter().any(|it| it.name == name) {
            return Ok(true);
        }

        // unmanaged
        self.has_unmanaged_package(name)
    }

    fn has_unmanaged_package(&self, name: &str) -> Result<bool> {
        Ok(self
            .list_unmanaged_packages()?
            .iter()
            .any(|it| it.name == name))
    }

    fn get_package_indexes(&self) -> Result<Vec<PackageIndex>> {
        let config = self.config.as_ref().unwrap();
        let index_file = config.managed_pkg_index_file()?;
        let pkg_indexes =
            serde_yaml::from_reader::<File, Vec<PackageIndex>>(File::open(index_file)?)?;

        Ok(pkg_indexes)
    }
}

#[async_trait]
impl CacheAsyncTrait for CacheService {
    // FIXME enhance performance
    async fn update_repositories(&self) -> Result<()> {
        info!("Updating repos");

        let container = self.container.as_ref().unwrap();
        let config = self.config.as_ref().unwrap();

        let dir = config.huber_repo_dir()?;
        info!("Updating {:?}", dir);

        let client = GithubClient::new(config.to_github_credentials(), config.to_github_key_path());

        info!("Updating managed repos");
        client.clone("innobead", "huber", dir.clone()).await?;

        info!("Updating unmanaged repos");
        let repo_service = container.get::<RepoService>().unwrap();
        for repo in repo_service.list()? {
            info!(
                "Updating {:?}",
                config.unmanaged_repo_dir(&repo.name).unwrap()
            );
            repo_service
                .download_save_pkgs_file(&repo.name, &repo.url)
                .await?;
        }

        Ok(())
    }
}
