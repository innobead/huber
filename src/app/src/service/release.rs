use crate::service::{ItemOperationTrait, ItemSearchTrait};
use huber_common::config::Config;
use huber_common::result::Result;
use std::sync::Arc;
use tokio::runtime::Runtime;
use huber_common::model::package::{Package, Release};

#[derive(Debug)]
pub(crate) struct ReleaseService {
    pub(crate) config: Option<Arc<Config>>,
    pub(crate) runtime: Option<Arc<Runtime>>,
}

impl ReleaseService {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            runtime: None,
        }
    }
}

impl ItemOperationTrait for ReleaseService {
    type Item = Package;
    type ItemInstance = Release;

    fn create(&self, _obj: &Self::Item) -> Result<Self::ItemInstance> {


        unimplemented!()
    }

    fn update(&self, obj: &Self::Item) -> Result<Self::ItemInstance> {
        unimplemented!()
    }

    fn delete(&self, _name: &str) -> Result<()> {
        unimplemented!()
    }

    fn list(&self) -> Result<Vec<Self::ItemInstance>> {
        unimplemented!()
    }

    fn get(&self, _name: &str) -> Result<Self::ItemInstance> {
        unimplemented!()
    }

    fn has(&self, name: &str) -> Result<bool> {
        unimplemented!()
    }
}

impl ItemSearchTrait for ReleaseService {
    type Item = Release;

    fn search(&self, name: Option<&str>, pattern: Option<&str>, owner: Option<&str>) -> Result<Vec<Self::Item>> {
        unimplemented!()
    }

    fn info(&self, name: &str) -> Result<Self::Item> {
        unimplemented!()
    }
}
