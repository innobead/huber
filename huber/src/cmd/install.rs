use std::sync::Arc;

use async_trait::async_trait;
use clap::{Args, ValueHint};
use huber_common::model::config::Config;
use huber_common::model::release::Release;
use log::{debug, info, warn};
use simpledi_rs::di::{DIContainer, DIContainerTrait};
use tokio::task::JoinHandle;

use crate::cmd::CommandTrait;
use crate::lock_huber_ops;
use crate::opt::parse_pkg_name_optional_semver;
use crate::service::cache::{CacheAsyncTrait, CacheService};
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait, ItemSearchTrait};

#[derive(Args)]
pub struct InstallArgs {
    #[arg(
        help = "Package name (e.g. 'package-name' or 'package-name@version')",
        num_args = 1,
        required = true,
        value_parser = parse_pkg_name_optional_semver,
        value_hint = ValueHint::Unknown,
    )]
    name_version: Vec<(String, String)>,
}

#[async_trait]
impl CommandTrait for InstallArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        lock_huber_ops!(config);

        let release_service = Arc::new(container.get::<ReleaseService>().unwrap().clone());
        let pkg_service = Arc::new(container.get::<PackageService>().unwrap().clone());

        let cache_service = container.get::<CacheService>().unwrap();
        cache_service.update_repositories().await?;

        install_packages(release_service, pkg_service, &self.name_version).await?;

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
    pkg_versions: &[(String, String)],
) -> anyhow::Result<()> {
    let mut join_handles = vec![];

    for (pkg, version) in pkg_versions.iter().cloned() {
        let pkg_service = pkg_service.clone();
        let release_service = release_service.clone();

        let handle: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
            if !pkg_service.has(&pkg)? {
                warn!("Skipped installing package, as {} not found", pkg);
                return Ok(());
            }

            let mut pkg = pkg_service.get(&pkg)?;
            let is_latest = version.is_empty();
            let version = if version.is_empty() {
                let v = release_service.get_latest(&pkg).await?.version;
                info!(
                    "{} version not specified, getting the latest version ({})",
                    pkg.name, v
                );
                v
            } else {
                version
            };

            let releases: Vec<Release> = release_service.search(Some(&pkg.name), None, None)?;
            if releases.iter().any(|r| r.version == version) {
                warn!("{}@{} already installed", pkg.name, version);
                return Ok(());
            }

            let msg = if is_latest {
                format!("{}@latest/{}", pkg.name, version)
            } else {
                format!("{}@{}", pkg.name, version)
            };

            info!("Installing package {}", msg);
            pkg.version = Some(version.clone());
            release_service.update(&pkg).await?;
            info!("{} installed", msg);

            Ok(())
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.await??;
    }

    Ok(())
}
