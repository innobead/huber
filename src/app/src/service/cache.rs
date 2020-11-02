use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use huber_common::di::{DIContainer, MutableRc};
use huber_common::result::Result;

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
}

impl CacheService {
    pub(crate) fn new() -> Self {
        Self {
            dir: Default::default(),
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
