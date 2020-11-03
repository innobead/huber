use huber_common::result::Result;

pub(crate) mod cache;
pub(crate) mod context;
pub(crate) mod package;
pub(crate) mod release;
pub(crate) mod update;

pub(crate) trait ItemOperationTrait {
    type Item;
    type ItemInstance;
    type Condition;

    fn create(&self, obj: &Self::Item) -> Result<Self::ItemInstance>;
    fn update(&self, obj: &Self::Item) -> Result<Self::ItemInstance>;
    fn delete(&self, name: &str) -> Result<()>;
    fn list(&self) -> Result<Vec<Self::ItemInstance>>;
    fn find(&self, condition: &Self::Condition) -> Result<Vec<Self::ItemInstance>>;
    fn get(&self, name: &str) -> Result<Self::ItemInstance>;
    fn has(&self, name: &str) -> Result<bool>;
}

pub(crate) trait ItemSearchTrait {
    type Item;

    fn search(
        &self,
        name: Option<&str>,
        pattern: Option<&str>,
        owner: Option<&str>,
    ) -> Result<Vec<Self::Item>>;

    fn info(&self, name: &str) -> Result<Self::Item>;
}
