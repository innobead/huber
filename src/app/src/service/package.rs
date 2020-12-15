use std::sync::Arc;

use async_trait::async_trait;
use log::debug;

use huber_common::model::config::{Config, ConfigFieldConvertTrait};
use huber_common::model::package::{Package, PackageSource};
use huber_common::result::Result;
use simpledi_rs::di::{DIContainer, DIContainerExtTrait, DependencyInjectTrait};

use crate::component::github::{GithubClient, GithubClientTrait};
use crate::service::cache::{CacheService, CacheTrait};
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait, ItemSearchTrait, ServiceTrait};

#[derive(Debug)]
pub(crate) struct PackageService {
    pub(crate) container: Option<Arc<DIContainer>>,
}
unsafe impl Send for PackageService {}
unsafe impl Sync for PackageService {}

impl PackageService {
    pub(crate) fn new() -> Self {
        Self { container: None }
    }
}

impl ServiceTrait for PackageService {}

impl DependencyInjectTrait for PackageService {
    fn inject(&mut self, container: Arc<DIContainer>) {
        self.container = Some(container)
    }
}

impl ItemOperationTrait for PackageService {
    type Item = Package;
    type ItemInstance = Package;
    type Condition = String;

    fn delete(&self, _name: &str) -> Result<()> {
        unimplemented!()
    }

    fn list(&self) -> Result<Vec<Self::ItemInstance>> {
        debug!("Getting all packages");

        self.search(None, None, None)
    }

    fn get(&self, name: &str) -> Result<Self::ItemInstance> {
        debug!("Getting package: {}", name);
        self.search(Some(name), None, None).map(|it| it[0].clone())
    }
}

#[async_trait]
impl ItemOperationAsyncTrait for PackageService {
    type Item_ = Package;
    type ItemInstance_ = Package;
    type Condition_ = String;

    async fn create(&self, _obj: Self::Item_) -> Result<Self::ItemInstance_> {
        unimplemented!()
    }

    async fn update(&self, _obj: &Self::Item_) -> Result<Self::ItemInstance_> {
        unimplemented!()
    }

    async fn find(&self, pkg_name: &Self::Condition_) -> Result<Vec<Self::ItemInstance_>> {
        debug!("Finding packages: {}", pkg_name);

        let config = self.container.get::<Config>().unwrap();
        let client = GithubClient::new(config.to_github_credentials(), config.to_github_key_path());
        let pkg = self.get(pkg_name)?;

        match &pkg.source {
            PackageSource::Github { owner, repo } => {
                let releases = client.get_releases(&owner, &repo, &pkg).await?;
                Ok(releases
                    .into_iter()
                    .map(|it| {
                        let mut pkg = it.package;
                        pkg.version = Some(it.version);
                        pkg.release_kind = it.kind;

                        pkg
                    })
                    .collect())
            }
            _ => Err(anyhow!("{} unsupported package source", pkg.source)),
        }
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
        let cache_service = self.container.get::<CacheService>().unwrap();

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
