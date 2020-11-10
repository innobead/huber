use std::sync::Arc;

use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::model::package::{Package, PackageSource};
use huber_common::result::Result;

use crate::component::github::{GithubClient, GithubClientTrait};
use crate::service::cache::{CacheService, CacheTrait};
use crate::service::{ItemOperationTrait, ItemSearchTrait};
use log::debug;

#[derive(Debug)]
pub(crate) struct PackageService {
    pub(crate) config: Option<Arc<Config>>,
    pub(crate) runtime: Option<Arc<Runtime>>,
}

impl PackageService {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            runtime: None,
        }
    }
}

impl ItemOperationTrait for PackageService {
    type Item = Package;
    type ItemInstance = Package;
    type Condition = String;

    fn create(&self, _obj: &Self::Item) -> Result<Self::ItemInstance> {
        unimplemented!()
    }

    fn update(&self, _obj: &Self::Item) -> Result<Self::ItemInstance> {
        unimplemented!()
    }

    fn delete(&self, _name: &str) -> Result<()> {
        unimplemented!()
    }

    fn list(&self) -> Result<Vec<Self::ItemInstance>> {
        debug!("Getting all packages");

        self.search(None, None, None)
    }

    fn find(&self, pkg_name: &Self::Condition) -> Result<Vec<Self::ItemInstance>> {
        debug!("Finding packages: {}", pkg_name);

        let config = self.config.as_ref().unwrap();
        let client = GithubClient::new(
            config.github_credentials.clone(),
            config.git_ssh_key.clone(),
        );
        let pkg = self.get(pkg_name)?;

        let mut runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            match &pkg.source {
                PackageSource::Github { owner, repo } => {
                    let releases = client.get_releases(&owner, &repo, &pkg).await?;
                    Ok(releases
                        .into_iter()
                        .map(|it| {
                            let mut pkg = it.package;
                            pkg.version = Some(it.version);
                            pkg.release_kind = Some(it.kind);

                            pkg
                        })
                        .collect())
                }
                _ => Err(anyhow!("{} unsupported package source", pkg.source)),
            }
        })
    }

    fn get(&self, name: &str) -> Result<Self::ItemInstance> {
        debug!("Getting package: {}", name);
        self.search(Some(name), None, None).map(|it| it[0].clone())
    }
}

impl ItemSearchTrait for PackageService {
    type SearchItem = Package;

    fn search(
        &self,
        name: Option<&str>,
        pattern: Option<&str>,
        owner: Option<&str>,
    ) -> Result<Vec<Self::SearchItem>> {
        let container = di_container();
        let cache_service = container.get::<CacheService>().unwrap();

        let owner = owner.unwrap_or("");
        let mut found_items: Vec<Self::SearchItem> = vec![];

        if let Some(name) = name {
            debug!("Searching package by name: {}", name);
            found_items.push(cache_service.get_package(name)?);

            return Ok(found_items);
        }

        if let Some(pattern) = pattern {
            debug!("Searching package by pattern: {}", pattern);
            let mut found_pkgs = cache_service.list_packages(pattern, owner)?;
            found_items.append(&mut found_pkgs);

            return Ok(found_items);
        }

        debug!("Searching all packages");
        let mut all_pkgs = cache_service.list_packages("", owner)?;
        found_items.append(&mut all_pkgs);

        Ok(found_items)
    }
}
