use std::borrow::Borrow;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use tokio::runtime::Runtime;

use huber_common::di::{DIContainer, MutableRc};
use huber_common::model::package::{Package, Release};
use huber_common::result::Result;

use crate::component::github::{GithubClient, GithubClientTrait};
use crate::service::{ItemOperationTrait, ItemSearchTrait};

#[derive(Debug)]
pub(crate) struct ReleaseService;

impl ReleaseService {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl ItemOperationTrait for ReleaseService {
    type Item = Package;
    type ItemInstance = Release;

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

impl ItemSearchTrait for ReleaseService {
    type Item = Package;

    fn search(&self, runtime: &Runtime, pattern: &str) -> Result<Vec<Self::Item>> {
        let dir = Path::new("").join("");

        let releases = runtime.block_on(async {
            GithubClient::new(None).list_managed_releases().await
        });

        //
        // let release_names: Vec<PathBuf> =
        //     fs::read_dir(dir.join("generated/packages").as_path())
        //         ?.filter(|it| it.as_ref().clone().unwrap().file_type().unwrap().is_file())
        //         .map(|it| it.unwrap().path())
        //         .collect();
        //
        // let mut releases = vec![];
        // for x in release_names {
        //     releases.push(serde_yaml::from_str::<Release>(fs::read_to_string(x)?.as_str())?);
        // }

        releases
    }

    fn search_unmanaged(&self, obj: &Self::Item) -> Result<Self::Item> {
        unimplemented!()
    }

    fn info(&self, name: &str) -> Result<Self::Item> {
        unimplemented!()
    }
}
