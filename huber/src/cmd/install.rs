use std::sync::Arc;

use async_trait::async_trait;
use clap::{Args, ValueHint};
use log::{debug, info, warn};
use simpledi_rs::di::{DIContainer, DIContainerTrait};
use tokio::task::JoinHandle;

use crate::cmd::update::is_pkg_locked_for_release;
use crate::cmd::{get_updated_package_version, CommandTrait, PlatformStdLib};
use crate::lock_huber_ops;
use crate::model::config::Config;
use crate::model::release::Release;
use crate::parse::parse_pkg_name_optional_semver;
use crate::service::cache::{CacheAsyncTrait, CacheService};
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::{ItemOperationTrait, ItemSearchTrait};

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

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[arg(
        help = "Prefer standard library (only for Linux or Windows)",
        long,
        num_args = 1,
        value_enum
    )]
    prefer_stdlib: Option<PlatformStdLib>,

    #[cfg(target_os = "macos")]
    #[arg(
        help = "Prefer standard library (only for Linux or Windows)",
        long,
        hide = true,
        num_args = 1,
        value_enum
    )]
    prefer_stdlib: Option<PlatformStdLib>,
}

#[async_trait]
impl CommandTrait for InstallArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        lock_huber_ops!(config);

        let release_service = Arc::new(container.get::<ReleaseService>().unwrap().clone());
        let pkg_service = Arc::new(container.get::<PackageService>().unwrap().clone());
        let config = Arc::new(config.clone());

        let cache_service = container.get::<CacheService>().unwrap();
        cache_service.update_repositories().await?;

        install_packages(
            config,
            release_service,
            pkg_service,
            &self.name_version,
            self.prefer_stdlib,
        )
        .await?;

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
    config: Arc<Config>,
    release_service: Arc<ReleaseService>,
    pkg_service: Arc<PackageService>,
    pkg_versions: &[(String, String)],
    prefer_stdlib: Option<PlatformStdLib>,
) -> anyhow::Result<()> {
    let mut join_handles: Vec<JoinHandle<anyhow::Result<()>>> = vec![];

    #[allow(clippy::unnecessary_to_owned)]
    for (pkg, version) in pkg_versions.iter().cloned() {
        let pkg_service = pkg_service.clone();
        let release_service = release_service.clone();
        let config = config.clone();

        let handle: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
            if !pkg_service.has(&pkg)? {
                warn!("Skipped installing package, as {} not found", pkg);
                return Ok(());
            }

            let mut pkg = pkg_service.get(&pkg)?;
            let latest_version = release_service.get_latest(&pkg).await?.version;

            let (version, is_latest) = if version.is_empty() {
                info!(
                    "{} version not specified, getting the latest version ({})",
                    pkg.name, latest_version
                );
                (latest_version, true)
            } else {
                (
                    get_updated_package_version(&version, &latest_version),
                    false,
                )
            };

            if is_pkg_locked_for_release(&config, &pkg, &version) {
                warn!(
                    "Package {} is locked to version {}. Skipping installing {}",
                    pkg.name,
                    config.lock_pkg_versions.get(&pkg.name).unwrap(),
                    version
                );
                return Ok(());
            }

            let releases: Vec<Release> =
                release_service.search(Some(&pkg.name), None, None, None)?;
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
            release_service.update(&pkg, prefer_stdlib).await?;
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
