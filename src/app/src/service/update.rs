use std::fs::remove_dir_all;
use std::sync::Arc;

use clap::crate_version;
use semver::Version;
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::result::Result;

use crate::component::github::{GithubClient, GithubClientTrait};

pub(crate) trait UpdateTrait {
    fn has_update(&self) -> Result<bool>;
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
    fn has_update(&self) -> Result<bool> {
        let current_version = crate_version!();

        // Note: async closure is not stable yet. ex: async || -> Result<>, so can't use ? in async {}
        self.runtime.as_ref().unwrap().block_on(async {
            let client = GithubClient::new(None);

            match client.get_latest_release("innobead", "huber").await {
                Err(e) => Err(e),

                Ok(r) => Ok(Version::parse(current_version) >= Version::parse(&r.version)),
            }
        })
    }

    fn update(&self) -> Result<bool> {
        if !self.has_update()? {
            return Ok(false);
        }

        let config = self.config.as_ref().unwrap();

        self.runtime.as_ref().unwrap().block_on(async {
            let client = GithubClient::new(None);

            match client.get_latest_release("innobead", "huber").await {
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

        let _ = remove_dir_all(config.huber_repo_dir()?);
        let _ = remove_dir_all(config.installed_pkg_root_dir()?);

        Ok(())
    }
}
