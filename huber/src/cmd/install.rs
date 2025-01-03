use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use clap::Args;
use huber_common::model::config::Config;
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};
use tokio::task::JoinHandle;

use crate::cmd::CommandTrait;
use crate::service::cache::{CacheAsyncTrait, CacheService};
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

#[derive(Args)]
pub struct InstallArgs {
    #[arg(help = "Package name (e.g. 'package-name' or 'package-name@version')")]
    name_version: Vec<String>,

    #[arg(help = "Set the installed package as current", long)]
    current: bool,
}

#[async_trait]
impl CommandTrait for InstallArgs {
    async fn run(&self, _config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let release_service = Arc::new(container.get::<ReleaseService>().unwrap().clone());
        let pkg_service = Arc::new(container.get::<PackageService>().unwrap().clone());

        let cache_service = container.get::<CacheService>().unwrap();
        cache_service.update_repositories().await?;

        let versions: Vec<_> = parse_package_name_versions(&self.name_version);
        install_packages(release_service, pkg_service, versions).await?;

        Ok(())
    }
}

pub fn parse_package_name_versions(name_versions: &[String]) -> Vec<(String, String)> {
    name_versions
        .iter()
        .map(|name_version| {
            let mut split = name_version.splitn(2, '@');
            let name = split.next().unwrap();
            let version = split.next().unwrap_or_default();

            (name.to_string(), version.to_string())
        })
        .collect()
}

pub async fn install_packages(
    release_service: Arc<ReleaseService>,
    pkg_service: Arc<PackageService>,
    versions: Vec<(String, String)>,
) -> anyhow::Result<()> {
    for (name, _) in versions.iter() {
        if !pkg_service.has(name)? {
            return Err(anyhow!("{} package not found", name));
        }
    }

    let mut join_handles = vec![];
    for (name, version) in versions.clone().into_iter() {
        let pkg_service = pkg_service.clone();
        let release_service = release_service.clone();

        let handle: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
            let mut pkg = pkg_service.get(&name)?;
            pkg.version = if version.is_empty() {
                None
            } else {
                Some(version)
            };

            let version = pkg.version.clone().unwrap_or("latest".to_string());
            info!("Installing {}@{}", pkg.name, version);
            release_service.update(&pkg).await?;
            info!("{}@{} installed", pkg.name, version);

            Ok(())
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.await??;
    }

    Ok(())
}
