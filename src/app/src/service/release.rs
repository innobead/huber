use crate::service::{ItemOperationTrait, ItemSearchTrait};
use huber_common::di::{DIContainer, DIObjectTrait, MutableRc};
use huber_common::model::release::{Release, ReleaseInstance};
use huber_common::result::Result;
use huber_procmacro::DIAware;

#[derive(Debug, DIAware)]
pub(crate) struct ReleaseService {
    container: MutableRc<DIContainer>,
}

impl DIObjectTrait for ReleaseService {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self {
        Self { container }
    }
}

impl ItemOperationTrait for ReleaseService {
    type Item = Release;
    type ItemInstance = ReleaseInstance;

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

// cache,
impl ItemSearchTrait for ReleaseService {
    type Item = Release;

    fn search(&self, pattern: &str) -> Result<Vec<Self::Item>> {
        unimplemented!()
    }

    fn search_unmanaged(&self, obj: &Self::Item) -> Result<Self::Item> {
        unimplemented!()
    }

    fn get(&self, name: &str) -> Result<Self::Item> {
        unimplemented!()
    }
}
