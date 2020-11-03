use crate::service::ItemOperationTrait;
use huber_common::config::Config;
use huber_common::model::context::Context;
use huber_common::result::Result;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Debug)]
pub(crate) struct ContextService {
    pub(crate) config: Option<Arc<Config>>,
    pub(crate) runtime: Option<Arc<Runtime>>,
}

impl ContextService {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            runtime: None,
        }
    }
}

impl ItemOperationTrait for ContextService {
    type Item = Context;
    type ItemInstance = Context;

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