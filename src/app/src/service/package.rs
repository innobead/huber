use std::sync::Arc;

use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::model::package::Package;
use huber_common::result::Result;

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
        self.search(None, None, None)
    }

    fn find(&self, _condition: &Self::Condition) -> Result<Vec<Self::ItemInstance>> {
        unimplemented!()
    }

    fn get(&self, name: &str) -> Result<Self::ItemInstance> {
        self.info(name)
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

        cache_service.update()?;

        let owner = owner.unwrap_or("");
        let mut items: Vec<Self::SearchItem> = vec![];

        if let Some(name) = name {
            items.push(cache_service.get_package(name)?);

            return Ok(items);
        }

        if let Some(pattern) = pattern {
            let mut found_pkgs = cache_service.list_packages(pattern, owner)?;
            items.append(&mut found_pkgs);

            return Ok(items);
        }

        let mut all_pkgs = cache_service.list_packages("", owner)?;
        items.append(&mut all_pkgs);

        Ok(items)
    }

    fn info(&self, name: &str) -> Result<Self::SearchItem> {
        self.search(Some(name), None, None).map(|it| it[0].clone())
    }
}
