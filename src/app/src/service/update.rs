use std::fs::{remove_dir_all, read_dir};
use std::sync::Arc;

use clap::crate_version;
use semver::Version;
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::model::package::{Package, PackageSource};
use huber_common::result::Result;

use crate::component::github::{GithubClient, GithubClientTrait};


pub(crate) trait UpdateTrait {
    fn has_update(&self) -> Result<(bool, String)>;
    fn update(&self) -> Result<bool>;
    fn reset(&self) -> Result<()>;
}

#[derive(Debug)]
pub(crate) struct UpdateService {
    pub(crate) config: Option<Arc<Config>>,
    pub(crate) runtime: Option<Arc<Runtime>>,
}

impl UpdateService {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            runtime: None,
        }
    }
}

impl UpdateTrait for UpdateService {
    fn has_update(&self) -> Result<(bool, String)> {
        let config = self.config.as_ref().unwrap();
        let current_version = crate_version!();

        // Note: async closure is not stable yet. ex: async || -> Result<>, so can't use ? in async {}
        //FIXME let runtime = self.runtime.as_ref().unwrap();
        let mut runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            let client = GithubClient::new(
                config.github_credentials.clone(),
                config.git_ssh_key.clone(),
            );

            let pkg = create_huber_package();
            match client.get_latest_release("innobead", "huber", &pkg).await {
                Err(e) => Err(e),
                Ok(r) => Ok((
                    Version::parse(current_version) >= Version::parse(&r.version),
                    r.version,
                )),
            }
        })
    }

    fn update(&self) -> Result<bool> {
        if !self.has_update()?.0 {
            return Ok(false);
        }

        let config = self.config.as_ref().unwrap();

        //FIXME let runtime = self.runtime.as_ref().unwrap();
        let mut runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            let client = GithubClient::new(
                config.github_credentials.clone(),
                config.git_ssh_key.clone(),
            );

            let pkg = create_huber_package();
            match client.get_latest_release("innobead", "huber", &pkg).await {
                Err(e) => Err(e),

                Ok(r) => {
                    match client
                        .download_artifacts(&r, config.bin_dir().unwrap())
                        .await
                    {
                        Err(e) => Err(e),
                        Ok(_r_) => Ok(true),
                    }
                }
            }
        })
    }

    fn reset(&self) -> Result<()> {
        let config = self.config.as_ref().unwrap();

        let bin_dir_path = config.bin_dir()?;
        if bin_dir_path.exists() {
            for entry in read_dir(bin_dir_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.file_name().unwrap().to_str().unwrap() == "huber" {
                    continue
                }

                let _ = remove_dir_all(path);
            }
        }

        let _ = remove_dir_all(config.installed_pkg_root_dir()?);
        let _ = remove_dir_all(config.temp_dir()?);
        let _ = remove_dir_all(config.repo_root_dir()?);

        Ok(())
    }
}

fn create_huber_package() -> Package {
    Package {
        name: "huber".to_string(),
        source: PackageSource::Github {
            owner: "innobead".to_string(),
            repo: "huber".to_string(),
        },
        targets: vec![],
        detail: None,
        version: None,
        description: None,
        release_kind: None,
    }
}
