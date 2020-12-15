use async_trait::async_trait;

use huber_common::result::Result;
use simpledi_rs::di::DependencyInjectTrait;

pub(crate) mod cache;
pub(crate) mod config;
pub(crate) mod package;
pub(crate) mod release;
pub(crate) mod repo;
pub(crate) mod update;

pub(crate) trait ServiceTrait: DependencyInjectTrait {}

pub(crate) trait ItemOperationTrait: ItemSearchTrait + ItemOperationAsyncTrait {
    type Item;
    type ItemInstance;
    type Condition;

    fn delete(&self, name: &str) -> Result<()>;
    fn list(&self) -> Result<Vec<Self::ItemInstance>>;
    fn get(&self, name: &str) -> Result<Self::ItemInstance>;
    fn has(&self, name: &str) -> Result<bool> {
        Ok(!self.search(Some(name), None, None)?.is_empty())
    }
}

#[async_trait]
pub(crate) trait ItemOperationAsyncTrait {
    type Item_;
    type ItemInstance_;
    type Condition_;

    async fn create(&self, obj: Self::Item_) -> Result<Self::ItemInstance_>;
    async fn update(&self, obj: &Self::Item_) -> Result<Self::ItemInstance_>;
    async fn find(&self, condition: &Self::Condition_) -> Result<Vec<Self::ItemInstance_>>;
}

pub(crate) trait ItemSearchTrait {
    type SearchItem;

    fn search(
        &self,
        name: Option<&str>,
        pattern: Option<&str>,
        owner: Option<&str>,
    ) -> Result<Vec<Self::SearchItem>>;
}
