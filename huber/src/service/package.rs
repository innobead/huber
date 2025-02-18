use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use log::debug;
use simpledi_rs::di::{DIContainer, DIContainerExtTrait, DependencyInjectTrait};

use crate::error::HuberError::PackageNotFound;
use crate::gh::{GithubClient, GithubClientTrait};
use crate::model::config::{Config, ConfigFieldConvertTrait};
use crate::model::package::{Package, PackageSource, PackageSummary};
use crate::model::release::{ReleaseKind, SortModelTrait};
use crate::service::cache::{CacheService, CacheTrait};
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait, ItemSearchTrait, ServiceTrait};

#[derive(Debug, Clone)]
pub struct PackageService {
    pub container: Option<Arc<DIContainer>>,
}

unsafe impl Send for PackageService {}

unsafe impl Sync for PackageService {}

impl Default for PackageService {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageService {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub async fn find_summary(
        &self,
        pkg_name: &str,
        release_build_only: bool,
    ) -> anyhow::Result<Vec<PackageSummary>> {
        let mut pkgs: Vec<PackageSummary> = self
            .find(&pkg_name.to_string())
            .await?
            .into_iter()
            .filter(|it| {
                if release_build_only {
                    return matches!(
                        it.release_kind.unwrap_or(ReleaseKind::PreRelease),
                        ReleaseKind::Release
                    );
                }
                true
            })
            .map(PackageSummary::from)
            .collect();

        pkgs.sort_by_version();
        Ok(pkgs)
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

    fn delete(&self, _name: &str) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn list(&self) -> anyhow::Result<Vec<Self::ItemInstance>> {
        debug!("Getting all packages");

        self.search(None, None, None, None)
    }

    fn get(&self, name: &str) -> anyhow::Result<Self::ItemInstance> {
        debug!("Getting package: {}", name);

        let results = self.search(Some(name), None, None, None)?;
        if !results.is_empty() {
            Ok(results.first().unwrap().to_owned())
        } else {
            Err(anyhow!(PackageNotFound(name.into())))
        }
    }
}

#[async_trait]
impl ItemOperationAsyncTrait for PackageService {
    type Item_ = Package;
    type ItemInstance_ = Package;
    type Condition_ = String;

    async fn create(&self, _obj: Self::Item_) -> anyhow::Result<Self::ItemInstance_> {
        unimplemented!()
    }

    async fn update(&self, _obj: &Self::Item_) -> anyhow::Result<Self::ItemInstance_> {
        unimplemented!()
    }

    async fn find(&self, pkg_name: &Self::Condition_) -> anyhow::Result<Vec<Self::ItemInstance_>> {
        debug!("Finding packages: {}", pkg_name);

        let config = self.container.get::<Config>().unwrap();
        let client = GithubClient::new(config.to_github_credentials(), config.to_github_key_path());
        let pkg = self.get(pkg_name)?;

        match &pkg.source {
            PackageSource::Github { owner, repo } => {
                let releases = client.get_releases(owner, repo, &pkg).await?;
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
        repo: Option<&str>,
    ) -> anyhow::Result<Vec<Self::SearchItem>> {
        let cache_service = self.container.get::<CacheService>().unwrap();

        let owner = owner.unwrap_or("");
        let mut found_items: Vec<Self::SearchItem> = vec![];

        if let Some(pattern) = pattern {
            debug!("Searching package by pattern: {}", pattern);

            let mut found_pkgs = cache_service.list_packages(pattern, owner, repo)?;
            found_items.append(&mut found_pkgs);

            return Ok(found_items);
        }

        if let Some(name) = name {
            debug!("Searching package by name: {}", name);

            match cache_service.get_package(name, repo) {
                Ok(pkg) => found_items.push(pkg),
                Err(err) => debug!("{}", err),
            }

            return Ok(found_items);
        }

        debug!("Searching all packages");
        let mut all_pkgs = cache_service.list_packages("", owner, repo)?;
        found_items.append(&mut all_pkgs);

        Ok(found_items)
    }
}
