use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use huber_common::model::config::Config;
use simpledi_rs::di::{DIContainer, DIContainerTrait, DependencyInjectTrait};
use simpledi_rs::{create_dep, inject_dep};

use crate::service::cache::{CacheService, CacheTrait};
use crate::service::config::ConfigService;
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::repo::RepoService;
use crate::service::update::HuberUpdateService;

pub mod cache;
pub mod config;
pub mod package;
pub mod release;
pub mod repo;
pub mod update;

pub trait ServiceTrait: DependencyInjectTrait {}

pub trait ItemOperationTrait: ItemSearchTrait + ItemOperationAsyncTrait {
    type Item;
    type ItemInstance;
    type Condition;

    fn delete(&self, name: &str) -> anyhow::Result<()>;
    fn list(&self) -> anyhow::Result<Vec<Self::ItemInstance>>;
    fn get(&self, name: &str) -> anyhow::Result<Self::ItemInstance>;
    fn has(&self, name: &str) -> anyhow::Result<bool> {
        Ok(!self.search(Some(name), None, None)?.is_empty())
    }
}

#[async_trait]
pub trait ItemOperationAsyncTrait {
    type Item_;
    type ItemInstance_;
    type Condition_;

    async fn create(&self, obj: Self::Item_) -> anyhow::Result<Self::ItemInstance_>;
    async fn update(&self, obj: &Self::Item_) -> anyhow::Result<Self::ItemInstance_>;
    async fn find(&self, condition: &Self::Condition_) -> anyhow::Result<Vec<Self::ItemInstance_>>;
}

pub trait ItemSearchTrait {
    type SearchItem;

    fn search(
        &self,
        name: Option<&str>,
        pattern: Option<&str>,
        owner: Option<&str>,
    ) -> anyhow::Result<Vec<Self::SearchItem>>;
}

pub fn init_services(config: &Config) -> Arc<DIContainer> {
    let mut container = DIContainer::new();

    create_dep!(CacheService::new(), container);
    create_dep!(config.clone(), container);
    create_dep!(ConfigService::new(), container);
    create_dep!(PackageService::new(), container);
    create_dep!(ReleaseService::new(), container);
    create_dep!(RepoService::new(), container);
    create_dep!(HuberUpdateService::new(), container);

    let container = container.init().unwrap();

    inject_dep!(PackageService, container.clone());
    inject_dep!(ReleaseService, container.clone());
    inject_dep!(CacheService, container.clone());
    inject_dep!(HuberUpdateService, container.clone());
    inject_dep!(RepoService, container.clone());
    inject_dep!(ConfigService, container.clone());

    let cache_service = container
        .get::<CacheService>()
        .expect("Failed to get cache service");
    cache_service
        .refresh_package_indexes()
        .expect("Failed to refresh package indexes");

    container
}

pub fn check_pkg_installed(
    pkg_service: &PackageService,
    release_service: &ReleaseService,
    pkg: &String,
) -> anyhow::Result<()> {
    if !pkg_service.has(pkg)? {
        return Err(anyhow!("Package {} not found", pkg));
    }

    if !release_service.has(pkg)? {
        return Err(anyhow!("Package {} not installed", pkg));
    }

    Ok(())
}
