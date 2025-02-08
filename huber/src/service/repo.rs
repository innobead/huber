use std::fs::{read_dir, remove_dir_all, remove_file, File};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use log::debug;
use simpledi_rs::di::{DIContainer, DIContainerExtTrait, DependencyInjectTrait};

use crate::model::config::{Config, ConfigPath};
use crate::model::package::Package;
use crate::model::repo::Repository;
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait, ItemSearchTrait, ServiceTrait};

pub trait RepoTrait {
    fn get_packages_by_repo(&self, name: &str) -> anyhow::Result<Vec<Package>>;
}

#[async_trait]
pub trait RepoAsyncTrait {
    async fn download_save_pkgs_file_from_remote_github(
        &self,
        name: &str,
        url: &str,
    ) -> anyhow::Result<()>;
    async fn download_save_pkgs_file_from_local<P: AsRef<Path> + Send>(
        &self,
        name: &str,
        url: P,
    ) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct RepoService {
    pub container: Option<Arc<DIContainer>>,
}

unsafe impl Send for RepoService {}

unsafe impl Sync for RepoService {}

impl ServiceTrait for RepoService {}

impl DependencyInjectTrait for RepoService {
    fn inject(&mut self, container: Arc<DIContainer>) {
        self.container = Some(container);
    }
}

impl Default for RepoService {
    fn default() -> Self {
        Self::new()
    }
}

impl RepoService {
    pub fn new() -> Self {
        Self { container: None }
    }
}

impl ItemSearchTrait for RepoService {
    type SearchItem = Repository;

    fn search(
        &self,
        name: Option<&str>,
        _pattern: Option<&str>,
        _owner: Option<&str>,
        _repo: Option<&str>,
    ) -> anyhow::Result<Vec<Self::SearchItem>> {
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

    fn delete(&self, name: &str) -> anyhow::Result<()> {
        let config = self.container.get::<Config>().unwrap();

        let path = config.external_repo_dir(name)?;
        if path.exists() {
            debug!("{:?} removed", path);
            let _ = remove_dir_all(path);
        }

        Ok(())
    }

    // FIXME enhance performance
    fn list(&self) -> anyhow::Result<Vec<Self::ItemInstance>> {
        let config = self.container.get::<Config>().unwrap();

        let mut repos: Vec<Repository> = vec![];
        let path = config.repo_root_dir()?;

        for entry in read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let dir_name = path.file_name().unwrap().to_str().unwrap();
                // not include managed repo
                if dir_name == "huber" {
                    continue;
                }

                let repo_f = config.external_repo_file(dir_name)?;
                if repo_f.exists() {
                    let f = File::open(&repo_f)?;
                    let result: Repository = serde_yaml::from_reader(f)?;
                    repos.push(result);
                }
            }
        }

        Ok(repos)
    }

    fn get(&self, _name: &str) -> anyhow::Result<Self::ItemInstance> {
        unimplemented!()
    }
}

#[async_trait]
impl ItemOperationAsyncTrait for RepoService {
    type Item_ = Repository;
    type ItemInstance_ = Repository;
    type Condition_ = String;

    async fn create(&self, obj: Self::Item_) -> anyhow::Result<Self::ItemInstance_> {
        let config = self.container.get::<Config>().unwrap();

        debug!("Creating external repo: {:?}", &obj);
        match &obj {
            _ if obj.url.is_some() => {
                self.download_save_pkgs_file_from_remote_github(
                    &obj.name,
                    obj.url.as_ref().unwrap(),
                )
                .await?
            }

            _ if obj.file.is_some() => {
                self.download_save_pkgs_file_from_local(&obj.name, obj.file.as_ref().unwrap())
                    .await?
            }

            _ => return Err(anyhow!("Repo file or url not provided: {:?}", &obj)),
        }

        let path = config.external_repo_file(&obj.name)?;
        let file = File::create(&path)?;
        serde_yaml::to_writer(file, &obj)?;

        Ok(obj)
    }

    async fn update(&self, _obj: &Self::Item_) -> anyhow::Result<Self::ItemInstance_> {
        unimplemented!()
    }

    async fn find(
        &self,
        _condition: &Self::Condition_,
    ) -> anyhow::Result<Vec<Self::ItemInstance_>> {
        unimplemented!()
    }
}

impl RepoTrait for RepoService {
    fn get_packages_by_repo(&self, name: &str) -> anyhow::Result<Vec<Package>> {
        let config = self.container.get::<Config>().unwrap();
        let f = config.external_repo_pkgs_file(name)?;
        let f = File::open(&f)?;

        Ok(serde_yaml::from_reader(f)?)
    }
}

#[async_trait]
impl RepoAsyncTrait for RepoService {
    async fn download_save_pkgs_file_from_remote_github(
        &self,
        name: &str,
        url: &str,
    ) -> anyhow::Result<()> {
        let config = self.container.get::<Config>().unwrap();

        let path = config.external_repo_pkgs_file(name)?;
        if path.exists() {
            let _ = remove_file(&path);
        }

        let mut url = url.to_string();
        debug!("Saving {} to {:?}", &url, &path);

        let from_github = url.contains("raw.githubusercontent.com");
        if from_github {
            if let Some(token) = config.github_token.clone() {
                let re = regex::Regex::new(r"(http|https)://")?;
                url = re
                    .replace(&url, format!("$1://{}@", token).as_str())
                    .to_string()
            }
        }

        let response = reqwest::get(&url.to_string()).await?;
        match response.error_for_status() {
            Err(e) => Err(anyhow!("{:?}", e)),
            Ok(response) => {
                let mut f = File::create(&path)?;
                let bytes = response.bytes().await?;
                f.write_all(&bytes)?;

                Ok(())
            }
        }
    }

    async fn download_save_pkgs_file_from_local<P: AsRef<Path> + Send>(
        &self,
        name: &str,
        url: P,
    ) -> anyhow::Result<()> {
        let f = File::open(&url)?;
        let pkgs: Vec<Package> = get_packages_from_file(f)?;

        let config = self.container.get::<Config>().unwrap();
        let path = config.external_repo_pkgs_file(name)?;
        if path.exists() {
            let _ = remove_file(&path);
        }

        debug!("Saving {:?} to {:?}", url.as_ref(), &path);

        let f = File::create(&path)?;
        serde_yaml::to_writer(&f, &pkgs)?;

        Ok(())
    }
}

fn get_packages_from_file(f: File) -> anyhow::Result<Vec<Package>> {
    Ok(serde_yaml::from_reader(&f)?)
}
