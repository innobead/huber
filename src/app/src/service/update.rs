use std::sync::Arc;

use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::result::Result;

pub(crate) trait UpdateTrait {
    fn has_update(&self) -> Result<bool>;
    fn update(&self) -> Result<()>;
    fn reset(&self) -> Result<()>;
}

#[derive(Debug)]
pub(crate) struct UpdateService {
    pub(crate) config: Option<Arc<Config>>,
    pub(crate) runtime: Option<Arc<Runtime>>,
}

impl UpdateService {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            runtime: None,
        }
    }
}

impl UpdateTrait for UpdateService {
    fn has_update(&self) -> Result<bool> {
        unimplemented!()
    }

    fn update(&self) -> Result<()> {
        unimplemented!()
    }

    fn reset(&self) -> Result<()> {
        unimplemented!()
    }
}
