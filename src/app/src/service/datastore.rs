use crate::service::ItemOperationTrait;
use huber_common::di::{DIContainer, MutableRc};
use huber_common::result::Result;

#[derive(Debug)]
pub(crate) struct DatastoreService;

impl DatastoreService {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl ItemOperationTrait for DatastoreService {
    type Item = ();
    type ItemInstance = ();

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
