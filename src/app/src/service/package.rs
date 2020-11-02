use std::borrow::Borrow;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow;
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::{container, DIContainer, MutableArc};
use huber_common::model::package::{Package, Release};
use huber_common::result::Result;

use crate::component::github::{GithubClient, GithubClientTrait};
use crate::service::cache::{CacheService, CacheTrait};
use crate::service::{ItemOperationTrait, ItemSearchTrait};

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
    type ItemInstance = Release;

    fn create(&self, obj: &Self::Item) -> Result<Self::ItemInstance> {
        unimplemented!()
    }

    fn delete(&self, name: &str) -> Result<()> {
        unimplemented!()
    }

    fn list(&self) -> Result<Vec<Self::ItemInstance>> {
        unimplemented!()
    }

    fn get(&self, name: &str) -> Result<Self::ItemInstance> {
        unimplemented!()
    }
}

impl ItemSearchTrait for PackageService {
    type Item = Package;

    fn search(
        &self,
        name: Option<&str>,
        pattern: Option<&str>,
    ) -> Result<Vec<Self::Item>> {
        let container = container();
        let cache_service = container.get::<CacheService>().unwrap();

        cache_service.update()?;

        let mut items: Vec<Self::Item> = vec![];

        if let Some(name) = name {
            items.push(cache_service.get_package(name)?);

            return Ok(items);
        }

        if let Some(pattern) = pattern {
            let mut found_pkgs = cache_service.list_packages(pattern)?;
            items.append(&mut found_pkgs);

            return Ok(items);
        }

        Err(anyhow!(""))
    }

    fn search_unmanaged(&self, obj: &Self::Item) -> Result<Self::Item> {
        unimplemented!()
    }

    fn info(&self, name: &str) -> Result<Self::Item> {
        unimplemented!()
    }
}
