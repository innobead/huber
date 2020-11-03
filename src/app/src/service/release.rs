use std::sync::Arc;

use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::model::package::{Package, Release};
use huber_common::result::Result;

use crate::service::{ItemOperationTrait, ItemSearchTrait};

pub(crate) trait ReleaseTrait {
    fn current(&self, pkg: &Package) -> Result<Release>;
    fn set_current(&self, release: &Release) -> Result<()>;
    fn list_current(&self) -> Result<Vec<Release>>;
    fn delete_release(&self, release: &Release) -> Result<()>;
}

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

impl ReleaseTrait for ReleaseService {
    fn current(&self, pkg: &Package) -> Result<Release> {
        unimplemented!()
    }

    fn set_current(&self, _release: &Release) -> Result<()> {
        unimplemented!()
    }

    fn list_current(&self) -> Result<Vec<Release>> {
        unimplemented!()
    }

    fn delete_release(&self, release: &Release) -> Result<()> {
        unimplemented!()
    }
}

impl ItemOperationTrait for ReleaseService {
    type Item = Package;
    type ItemInstance = Release;
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
        unimplemented!()
    }

    fn find(&self, _condition: &Self::Condition) -> Result<Vec<Self::ItemInstance>> {
        unimplemented!()
    }

    fn get(&self, _name: &str) -> Result<Self::ItemInstance> {
        unimplemented!()
    }

    fn has(&self, _name: &str) -> Result<bool> {
        unimplemented!()
    }
}

impl ItemSearchTrait for ReleaseService {
    type Item = Release;

    fn search(
        &self,
        _name: Option<&str>,
        _pattern: Option<&str>,
        _owner: Option<&str>,
    ) -> Result<Vec<Self::Item>> {
        unimplemented!()
    }

    fn info(&self, _name: &str) -> Result<Self::Item> {
        unimplemented!()
    }
}
