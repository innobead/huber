use std::fs::{read_dir, remove_dir_all, remove_file, File};
use std::io::Write;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use hubcaps::Credentials;
use log::info;

use huber_common::config::Config;
use huber_common::di::DIContainer;
use huber_common::model::package::Package;
use huber_common::model::repo::Repository;
use huber_common::str::OsStrExt;

use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait, ItemSearchTrait, ServiceTrait};

pub(crate) trait RepoTrait {
    fn get_packages_by_repo(&self, name: &str) -> Result<Vec<Package>>;
}

#[async_trait]
pub(crate) trait RepoAsyncTrait {
    async fn download_save_pkgs_file(&self, name: &str, url: &str) -> Result<()>;
}

#[derive(Debug)]
pub(crate) struct RepoService {
    pub(crate) config: Option<Arc<Config>>,
    pub(crate) container: Option<Arc<DIContainer>>,
}
unsafe impl Send for RepoService {}
unsafe impl Sync for RepoService {}

impl ServiceTrait for RepoService {
    fn set_shared_properties(&mut self, config: Arc<Config>, container: Arc<DIContainer>) {
        self.config = Some(config);
        self.container = Some(container);
    }
}

impl RepoService {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            container: None,
        }
    }
}

impl ItemSearchTrait for RepoService {
    type SearchItem = Repository;

    fn search(
        &self,
        name: Option<&str>,
        _pattern: Option<&str>,
        _owner: Option<&str>,
    ) -> Result<Vec<Self::SearchItem>> {
        let repo = self.list()?.into_iter().find(|it| it.name == name.unwrap());
        if repo.is_some() {
            return Ok(vec![repo.unwrap()]);
        }

        Ok(vec![])
    }
}

impl ItemOperationTrait for RepoService {
    type Item = Repository;
    type ItemInstance = Repository;
    type Condition = String;

    fn delete(&self, name: &str) -> Result<()> {
        let config = self.config.as_ref().unwrap();

        let path = config.unmanaged_repo_dir(&name)?;
        if path.exists() {
            info!("{:?} removed", path);
            let _ = remove_dir_all(path);
        }

        Ok(())
    }

    // FIXME enhance performance
    fn list(&self) -> Result<Vec<Self::ItemInstance>> {
        let config = self.config.as_ref().unwrap();

        let mut repos: Vec<Repository> = vec![];
        let path = config.repo_root_dir()?;

        for entry in read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let dir_name = path.file_name().unwrap().to_str_direct();
                // not include managed repo
                if dir_name == "huber" {
                    continue;
                }

                let repo_f = config.unmanaged_repo_file(dir_name)?;
                if repo_f.exists() {
                    let f = File::open(&repo_f)?;
                    let result: Repository = serde_yaml::from_reader(f)?;
                    repos.push(result);
                }
            }
        }

        Ok(repos)
    }

    fn get(&self, _name: &str) -> Result<Self::ItemInstance> {
        unimplemented!()
    }
}

#[async_trait]
impl ItemOperationAsyncTrait for RepoService {
    type Item_ = Repository;
    type ItemInstance_ = Repository;
    type Condition_ = String;

    async fn create(&self, obj: Self::Item_) -> Result<Self::ItemInstance_> {
        let config = self.config.as_ref().unwrap();

        let path = config.unmanaged_repo_file(&obj.name)?;
        let file = File::create(&path)?;
        serde_yaml::to_writer(file, &obj)?;

        self.download_save_pkgs_file(&obj.name, &obj.url).await?;

        Ok(obj)
    }

    async fn update(&self, _obj: &Self::Item_) -> Result<Self::ItemInstance_> {
        unimplemented!()
    }

    async fn find(&self, _condition: &Self::Condition_) -> Result<Vec<Self::ItemInstance_>> {
        unimplemented!()
    }
}

impl RepoTrait for RepoService {
    fn get_packages_by_repo(&self, name: &str) -> Result<Vec<Package>> {
        let config = self.config.as_ref().unwrap();
        let f = config.unmanaged_repo_pkgs_file(name)?;
        let f = File::open(&f)?;

        Ok(serde_yaml::from_reader(f)?)
    }
}

#[async_trait]
impl RepoAsyncTrait for RepoService {
    async fn download_save_pkgs_file(&self, name: &str, url: &str) -> Result<()> {
        let config = self.config.as_ref().unwrap();

        let path = config.unmanaged_repo_pkgs_file(&name)?;
        if path.exists() {
            let _ = remove_file(&path);
        }

        info!("Saving {} to {:?}", &url, &path);

        let mut url = url.to_string();
        url = url.replace("github.com", "raw.githubusercontent.com") + "/master/huber.yaml";

        url = if let Some(Credentials::Token(token)) = config.github_credentials.clone() {
            let re = regex::Regex::new(r"(http|https)://")?;
            re.replace(&url, format!("$1://{}@", token).as_str())
                .to_string()
        } else {
            format!("{}/master/huber.yaml", url)
        };

        let response = reqwest::get(&url.to_string()).await?;
        match response.error_for_status() {
            Err(e) => Err(anyhow!("{:?}", e)),
            Ok(response) => {
                let mut f = File::create(&path)?;
                let bytes = response.bytes().await?;
                f.write(&bytes)?;

                Ok(())
            }
        }
    }
}
