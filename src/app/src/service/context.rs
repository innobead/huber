use crate::service::ItemOperationTrait;
use huber_common::di::{DIContainer, DIObjectTrait, MutableRc};
use huber_common::model::context::Context;
use huber_common::result::Result;

#[derive(Debug)]
pub(crate) struct ContextService {
    container: MutableRc<DIContainer>,
}

impl DIObjectTrait for ContextService {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self {
        Self { container }
    }
}

impl ItemOperationTrait for ContextService {
    type Item = Context;
    type ItemInstance = Context;

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
