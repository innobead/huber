use std::fs::{read_dir, remove_dir_all, remove_file};
use std::sync::Arc;

use async_trait::async_trait;
use clap::crate_version;
use semver::Version;

use huber_common::di::DIContainer;
use huber_common::model::config::{Config, ConfigPath};

use huber_common::result::Result;

use crate::service::{ServiceTrait, ItemOperationTrait, ItemOperationAsyncTrait};
use crate::service::release::ReleaseService;
use crate::service::package::PackageService;

pub(crate) trait UpdateTrait {
    fn reset(&self) -> Result<()>;
}

#[async_trait]
pub(crate) trait UpdateAsyncTrait {
    async fn has_update(&self) -> Result<(bool, String)>;
    async fn update(&self) -> Result<()>;
}

#[derive(Debug)]
pub(crate) struct UpdateService {
    pub(crate) config: Option<Arc<Config>>,
    pub(crate) container: Option<Arc<DIContainer>>,
}

unsafe impl Send for UpdateService {}

unsafe impl Sync for UpdateService {}

impl ServiceTrait for UpdateService {
    fn set_shared_properties(&mut self, config: Arc<Config>, container: Arc<DIContainer>) {
        self.config = Some(config);
        self.container = Some(container);
    }
}

impl UpdateService {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            container: None,
        }
    }
}

impl UpdateTrait for UpdateService {
    fn reset(&self) -> Result<()> {
        let config = self.config.as_ref().unwrap();

        let bin_dir_path = config.bin_dir()?;
        if bin_dir_path.exists() {
            for entry in read_dir(bin_dir_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.file_name().unwrap().to_str().unwrap() == "huber" {
                    continue;
                }

                let _ = remove_dir_all(path);
            }
        }

        let _ = remove_dir_all(config.installed_pkg_root_dir()?);
        let _ = remove_dir_all(config.temp_dir()?);
        let _ = remove_dir_all(config.repo_root_dir()?);
        let _ = remove_file(config.lock_file()?);
        let _ = remove_file(config.config_file()?);

        Ok(())
    }
}

#[async_trait]
impl UpdateAsyncTrait for UpdateService {
    async fn has_update(&self) -> Result<(bool, String)> {
        let container = self.container.as_ref().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();
        let release_service = container.get::<ReleaseService>().unwrap();

        let current_version = crate_version!();
        let pkg = pkg_service.get("huber")?;

        match release_service.get_latest(pkg).await {
            Err(e) => Err(anyhow!("No update available: {:?}", e)),
            Ok(r) => Ok((
                Version::parse(&r.version) > Version::parse(current_version),
                r.version,
            )),
        }
    }

    async fn update(&self) -> Result<()> {
        let container = self.container.as_ref().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();
        let release_service = container.get::<ReleaseService>().unwrap();

        let _current_version = crate_version!();
        let mut pkg = pkg_service.get("huber")?;
        let release = release_service.get_latest(pkg.clone()).await?;
        pkg.version = Some(release.version);

        release_service.update(&pkg).await?;
        Ok(())
    }
}

