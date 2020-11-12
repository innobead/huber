use std::fs::File;

use std::sync::Arc;

use log::info;
use regex::Regex;
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::model::package::{Package, PackageIndex};
use huber_common::result::Result;

use crate::component::github::{GithubClient, GithubClientTrait};
use crate::service::ItemOperationTrait;
use crate::service::repo::{RepoService, RepoTrait};

pub(crate) trait CacheTrait {
    fn update_repositories(&self) -> Result<()>;
    fn get_package(&self, name: &str) -> Result<Package>;
    fn get_unmanaged_package(&self, name: &str) -> Result<Package>;
    fn list_packages(&self, pattern: &str, owner: &str) -> Result<Vec<Package>>;
    fn list_unmanaged_packages(&self) -> Result<Vec<Package>>;
    fn has_package(&self, name: &str) -> Result<bool>;
    fn has_unmanaged_package(&self, name: &str) -> Result<bool>;
    fn get_package_indexes(&self) -> Result<Vec<PackageIndex>>;
}

#[derive(Debug)]
pub(crate) struct CacheInfo {
    location: String,
}

#[derive(Debug)]
pub(crate) struct CacheService {
    pub(crate) config: Option<Arc<Config>>,
    pub(crate) runtime: Option<Arc<Runtime>>,
}

impl CacheService {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            runtime: None,
        }
    }
}

impl CacheTrait for CacheService {
    fn update_repositories(&self) -> Result<()> {
        info!("Updating repos");

        let container = di_container();
        let config = self.config.as_ref().unwrap();

        let mut runtime = Runtime::new().unwrap();
        info!("Updating managed repos");
        runtime.block_on(async {
            let dir = config.huber_repo_dir()?;
            info!("Updating {:?}", dir);

            let client = GithubClient::new(
                config.github_credentials.clone(),
                config.git_ssh_key.clone(),
            );
            client.clone("innobead", "huber", dir.clone()).await
        })?;

        info!("Updating unmanaged repos");
        let repo_service = container.get::<RepoService>().unwrap();
        for repo in repo_service.list()? {
            info!("Updating {:?}", config.unmanaged_repo_dir(&repo.name).unwrap());
            repo_service.download_save_pkgs_file(&repo.name, &repo.url)?;
        };

        Ok(())
    }

    fn get_package(&self, name: &str) -> Result<Package> {
        if !self.has_package(name)? {
            return Err(anyhow!("{} not found", name));
        }

        let config = self.config.as_ref().unwrap();
        let pkg_file = config.managed_pkg_manifest_file(name)?;

        if pkg_file.exists() {
            Ok(serde_yaml::from_reader::<File, Package>(File::open(pkg_file)?)?)
        } else {
            self.get_unmanaged_package(name)
        }
    }

    fn get_unmanaged_package(&self, name: &str) -> Result<Package> {
        match self.list_unmanaged_packages()?.into_iter().find(|it| it.name == name) {
            None => Err(anyhow!("{} not found", name)),
            Some(pkg) => Ok(pkg)
        }
    }

    fn list_packages(&self, pattern: &str, owner: &str) -> Result<Vec<Package>> {
        let mut pkgs: Vec<Package> = vec![];

        // managed packages
        match pattern {
            "" => {
                for p in self.get_package_indexes()? {
                    if owner == "" {
                        pkgs.push(self.get_package(&p.name)?);
                        continue;
                    }

                    if p.owner == owner {
                        pkgs.push(self.get_package(&p.name)?);
                    }
                }
            }

            _ => {
                let regex = Regex::new(pattern)?;

                for p in self.get_package_indexes()? {
                    if regex.is_match(&p.name) {
                        pkgs.push(self.get_package(&p.name)?);
                    }
                }
            }
        }

        // unmanaged packages
        pkgs.append(&mut self.list_unmanaged_packages()?);

        Ok(pkgs)
    }

    fn list_unmanaged_packages(&self) -> Result<Vec<Package>> {
        let container = di_container();
        let repo_service = container.get::<RepoService>().unwrap();
        let mut pkgs: Vec<Package> = vec![];

        let repos = repo_service.list()?;
        for repo in repos {
            pkgs.append(&mut repo_service.get_packages_by_repo(&repo.name)?);
        }

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
        Ok(self.list_unmanaged_packages()?.iter().any(|it| it.name == name))
    }

    fn get_package_indexes(&self) -> Result<Vec<PackageIndex>> {
        let config = self.config.as_ref().unwrap();
        let index_file = config.managed_pkg_index_file()?;
        let pkg_indexes =
            serde_yaml::from_reader::<File, Vec<PackageIndex>>(File::open(index_file)?)?;

        Ok(pkg_indexes)
    }
}
