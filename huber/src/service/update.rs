use std::fs::{read_dir, remove_dir_all, remove_file};
use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use huber_common::model::config::{Config, ConfigPath};
use log::info;
use semver::Version;
use simpledi_rs::di::{DIContainer, DIContainerExtTrait, DependencyInjectTrait};

use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait, ServiceTrait};

pub trait UpdateTrait {
    fn reset(&self) -> anyhow::Result<()>;
}

#[async_trait]
pub trait UpdateAsyncTrait {
    async fn has_update(&self) -> anyhow::Result<(bool, String)>;
    async fn update(&self) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct HuberUpdateService {
    pub container: Option<Arc<DIContainer>>,
}

unsafe impl Send for HuberUpdateService {}

unsafe impl Sync for HuberUpdateService {}

impl ServiceTrait for HuberUpdateService {}

impl DependencyInjectTrait for HuberUpdateService {
    fn inject(&mut self, container: Arc<DIContainer>) {
        self.container = Some(container);
    }
}

impl HuberUpdateService {
    pub fn new() -> Self {
        Self { container: None }
    }
}

impl UpdateTrait for HuberUpdateService {
    fn reset(&self) -> anyhow::Result<()> {
        let config = self.container.get::<Config>().unwrap();

        let bin_dir_path = config.bin_dir()?;
        if bin_dir_path.exists() {
            for entry in read_dir(bin_dir_path)? {
                let path = entry?.path();

                if path.file_name().unwrap().to_str().unwrap() == "huber" {
                    info!("Keeping huber executable");

                    let option = fs_extra::file::CopyOptions::new();
                    let temp_path = path.parent().unwrap().join("huber_temp");

                    info!("Coping {:?} to {:?}", &path, &temp_path);
                    let _ = remove_file(&temp_path);
                    fs_extra::file::copy(&path, &temp_path, &option)?;

                    info!("Moving {:?} to {:?}", &temp_path, &path);
                    let _ = remove_file(&path);
                    fs_extra::file::move_file(&temp_path, &path, &option)?;

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
impl UpdateAsyncTrait for HuberUpdateService {
    async fn has_update(&self) -> anyhow::Result<(bool, String)> {
        let pkg_service = self.container.get::<PackageService>().unwrap();
        let release_service = self.container.get::<ReleaseService>().unwrap();

        let current_version =
            Version::parse(env!("HUBER_VERSION").trim_start_matches("v")).unwrap();
        let pkg = pkg_service.get("huber")?;

        match release_service.get_latest(&pkg).await {
            Err(e) => Err(anyhow!("No update available: {:?}", e)),
            Ok(r) => {
                let result = Version::parse(r.version.trim_start_matches("v"))
                    .map(|ver| ver > current_version);

                match result {
                    Ok(update_needed) => Ok((update_needed, r.version)),
                    Err(e) => Err(anyhow!(
                        "A update available, but failed to continue: {:?}",
                        e
                    )),
                }
            }
        }
    }

    async fn update(&self) -> anyhow::Result<()> {
        let pkg_service = self.container.get::<PackageService>().unwrap();
        let release_service = self.container.get::<ReleaseService>().unwrap();

        let mut pkg = pkg_service.get("huber")?;
        let release = release_service.get_latest(&pkg).await?;
        pkg.version = Some(release.version);

        release_service.update(&pkg).await?;
        Ok(())
    }
}
