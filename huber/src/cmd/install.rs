use anyhow::anyhow;
use async_trait::async_trait;
use clap::Args;
use huber_common::model::config::Config;
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

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
        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        let cache_service = container.get::<CacheService>().unwrap();
        cache_service.update_repositories().await?;

        let versions: Vec<_> = parse_package_name_versions(&self.name_version);
        install_packages(release_service, pkg_service, versions).await?;

        Ok(())
    }
}

pub fn parse_package_name_versions(name_versions: &Vec<String>) -> Vec<(String, String)> {
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
    release_service: &ReleaseService,
    pkg_service: &PackageService,
    versions: Vec<(String, String)>,
) -> anyhow::Result<()> {
    for (name, _) in versions.iter() {
        if !pkg_service.has(name)? {
            return Err(anyhow!("{} package not found", name));
        }
    }

    for (name, version) in versions.iter() {
        let mut pkg = pkg_service.get(name)?;
        pkg.version = version.to_string().into();

        info!("Installing {}", pkg);
        if release_service.has(name)? {
            release_service.update(&pkg).await?;
        }
    }
    Ok(())
}
