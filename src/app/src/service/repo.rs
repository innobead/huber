use std::fs::{read_dir, remove_dir_all, remove_file, File};
use std::io::Write;
use std::sync::Arc;

use anyhow::Result;
use hubcaps::Credentials;
use log::info;
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::model::package::Package;
use huber_common::model::repo::Repository;
use huber_common::str::OsStrExt;

use crate::service::{ItemOperationTrait, ItemSearchTrait};

pub(crate) trait RepoTrait {
    fn get_packages_by_repo(&self, name: &str) -> Result<Vec<Package>>;
    fn download_save_pkgs_file(&self, name: &str, url: &str) -> Result<()>;
}

pub(crate) struct RepoService {
    pub(crate) config: Option<Arc<Config>>,
    pub(crate) runtime: Option<Arc<Runtime>>,
}

impl RepoService {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            runtime: None,
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

    fn create(&self, obj: Self::Item) -> Result<Self::ItemInstance> {
        let config = self.config.as_ref().unwrap();

        let path = config.unmanaged_repo_file(&obj.name)?;
        let file = File::create(&path)?;
        serde_yaml::to_writer(file, &obj)?;

        self.download_save_pkgs_file(&obj.name, &obj.url)?;

        Ok(obj)
    }

    fn update(&self, _obj: &Self::Item) -> Result<Self::ItemInstance> {
        unimplemented!()
    }

    fn delete(&self, name: &str) -> Result<()> {
        let config = self.config.as_ref().unwrap();

        let path = config.unmanaged_repo_dir(&name)?;
        if path.exists() {
            info!("{:?} removed", path);
            let _ = remove_dir_all(path);
        }

        Ok(())
    }

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

    fn find(&self, _condition: &Self::Condition) -> Result<Vec<Self::ItemInstance>> {
        unimplemented!()
    }

    fn get(&self, _name: &str) -> Result<Self::ItemInstance> {
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

    fn download_save_pkgs_file(&self, name: &str, url: &str) -> Result<()> {
        let config = self.config.as_ref().unwrap();
        let mut runtime = Runtime::new().unwrap();

        runtime.block_on(async {
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
        })
    }
}
