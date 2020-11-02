use tokio::runtime::Runtime;

use huber_common::result::Result;

pub(crate) mod cache;
pub(crate) mod context;
pub(crate) mod datastore;
pub(crate) mod package;

pub(crate) trait ItemOperationTrait {
    type Item;
    type ItemInstance;

    fn create(&self, obj: &Self::Item) -> Result<Self::ItemInstance>;
    fn delete(&self, name: &str) -> Result<()>;
    fn list(&self) -> Result<Vec<Self::ItemInstance>>;
    fn get(&self, name: &str) -> Result<Self::ItemInstance>;
}

pub(crate) trait ItemSearchTrait {
    type Item;

    fn search(&self, name: Option<&str>, pattern: Option<&str>) -> Result<Vec<Self::Item>>;
    fn search_unmanaged(&self, obj: &Self::Item) -> Result<Self::Item>;
    fn info(&self, name: &str) -> Result<Self::Item>;
}
