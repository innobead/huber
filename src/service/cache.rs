use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::base::di::{DIContainer, DIObjectTrait, MutableRc};
use crate::base::result::Result;

pub(crate) trait CacheTrait<K, V> {
    fn save(&self, key: K, data: &V) -> Result<CacheInfo>;
    fn get(&self, key: K) -> Result<V>;
}

#[derive(Debug)]
pub(crate) struct CacheInfo {
    location: String,
}

#[derive(Debug)]
pub(crate) struct CacheService {
    pub(crate) dir: PathBuf,
    container: MutableRc<DIContainer>,
}

impl DIObjectTrait for CacheService {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self {
        Self {
            dir: Default::default(),
            container,
        }
    }
}

impl<'a, K, V> CacheTrait<K, V> for CacheService
where
    K: ToString,
    V: Serialize + Deserialize<'a>,
{
    fn save(&self, key: K, data: &V) -> Result<CacheInfo> {
        unimplemented!()
    }

    fn get(&self, key: K) -> Result<V> {
        unimplemented!()
    }
}
