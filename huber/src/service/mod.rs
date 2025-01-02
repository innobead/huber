use async_trait::async_trait;
use simpledi_rs::di::DependencyInjectTrait;

pub mod cache;
pub mod config;
pub mod package;
pub mod release;
pub mod repo;
pub mod update;

pub trait ServiceTrait: DependencyInjectTrait {}

pub trait ItemOperationTrait: ItemSearchTrait + ItemOperationAsyncTrait {
    type Item;
    type ItemInstance;
    type Condition;

    fn delete(&self, name: &str) -> anyhow::Result<()>;
    fn list(&self) -> anyhow::Result<Vec<Self::ItemInstance>>;
    fn get(&self, name: &str) -> anyhow::Result<Self::ItemInstance>;
    fn has(&self, name: &str) -> anyhow::Result<bool> {
        Ok(!self.search(Some(name), None, None)?.is_empty())
    }
}

#[async_trait]
pub trait ItemOperationAsyncTrait {
    type Item_;
    type ItemInstance_;
    type Condition_;

    async fn create(&self, obj: Self::Item_) -> anyhow::Result<Self::ItemInstance_>;
    async fn update(&self, obj: &Self::Item_) -> anyhow::Result<Self::ItemInstance_>;
    async fn find(&self, condition: &Self::Condition_) -> anyhow::Result<Vec<Self::ItemInstance_>>;
}

pub trait ItemSearchTrait {
    type SearchItem;

    fn search(
        &self,
        name: Option<&str>,
        pattern: Option<&str>,
        owner: Option<&str>,
    ) -> anyhow::Result<Vec<Self::SearchItem>>;
}
