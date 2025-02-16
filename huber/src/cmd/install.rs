use std::sync::Arc;

use async_trait::async_trait;
use clap::{Args, ValueHint};
use log::{debug, info, warn};
use simpledi_rs::di::{DIContainer, DIContainerTrait};
use tokio::task::JoinHandle;

use crate::cmd::get_default_stdlib;
use crate::cmd::update::is_pkg_locked_for_release;
use crate::cmd::{get_updated_package_version, CommandTrait, PlatformStdLib};
use crate::lock_huber_ops;
use crate::model::config::Config;
use crate::model::package::{default_targets, Package, PackageSource};
use crate::model::release::Release;
use crate::model::repo::LOCAL_REPO;
use crate::parse::parse_pkg_name_optional_semver;
use crate::service::cache::{CacheAsyncTrait, CacheService};
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::repo::{RepoAsyncTrait, RepoService};
use crate::service::{ItemOperationTrait, ItemSearchTrait};

#[derive(Args)]
pub struct InstallArgs {
    #[arg(
        help = "Package name (e.g. 'package-name', 'package-name@version')g. 'owner/repo', 'owner/repo@version') for unmanaged packages by repositories",
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
        default_value_t = get_default_stdlib(),
        value_enum
    )]
    prefer_stdlib: PlatformStdLib,

    #[cfg(target_os = "macos")]
    #[arg(
        help = "Prefer standard library (only for Linux or Windows)",
        long,
        hide = true,
        num_args = 1,
        default_value_t = get_default_stdlib(),
        value_enum
    )]
    prefer_stdlib: PlatformStdLib,
}

#[async_trait]
impl CommandTrait for InstallArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        lock_huber_ops!(config);

        let release_service = Arc::new(container.get::<ReleaseService>().unwrap().clone());
        let pkg_service = Arc::new(container.get::<PackageService>().unwrap().clone());
        let repo_service = Arc::new(container.get::<RepoService>().unwrap().clone());
        let config = Arc::new(config.clone());

        let cache_service = container.get::<CacheService>().unwrap();
        cache_service.update_repositories().await?;

        add_packages_to_local_repo(release_service.clone(), repo_service, &self.name_version)
            .await?;

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

async fn add_packages_to_local_repo(
    release_service: Arc<ReleaseService>,
    repo_service: Arc<RepoService>,
    name_versions: &[(String, String)],
) -> anyhow::Result<()> {
    let mut pkgs: Vec<Package> = vec![];

    for (name, _) in name_versions.iter() {
        let tokens: Vec<_> = name.splitn(2, '/').collect();
        if tokens.len() != 2 {
            continue;
        }

        let owner = tokens.first().unwrap();
        let repo = tokens.last().unwrap();

        let pkg = Package {
            name: name.clone(),
            source: PackageSource::Github {
                owner: owner.to_string(),
                repo: repo.to_string(),
            },
            targets: default_targets(),
            ..Default::default()
        };
        if let Err(err) = release_service.get_latest(&pkg).await {
            warn!("Skipped adding package {} to local repo: {}", name, err);
            continue;
        };

        pkgs.push(pkg);
    }

    repo_service.add_pkgs_to_repo(LOCAL_REPO, &pkgs).await?;
    Ok(())
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
    prefer_stdlib: PlatformStdLib,
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
            let latest_version = release_service
                .get_latest(&pkg)
                .await
                .map(|r| r.version)
                .or_else(|err| {
                    warn!(
                        "Failed to get the latest release version of {}: {}",
                        pkg.name, err
                    );
                    anyhow::Ok("".to_string())
                })?;
            let release_check = !latest_version.is_empty();
            let (version, is_latest) = get_version_to_install(&version, &pkg, &latest_version)?;

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
            release_service
                .update(&pkg, &prefer_stdlib, release_check)
                .await?;
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

fn get_version_to_install(
    version: &str,
    pkg: &Package,
    latest_version: &str,
) -> anyhow::Result<(String, bool)> {
    let (version, is_latest) = if version.is_empty() {
        info!(
            "{} version not specified, getting the latest version ({})",
            pkg.name, latest_version
        );

        if latest_version.is_empty() {
            anyhow::bail!(
                "Failed to get the latest release version of {} to install",
                pkg.name
            );
        }

        (latest_version.to_string(), true)
    } else {
        (get_updated_package_version(version, latest_version), false)
    };

    Ok((version, is_latest))
}
