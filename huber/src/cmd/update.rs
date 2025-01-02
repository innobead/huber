use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use clap::{Args, ValueHint};
use huber_common::model::config::Config;
use huber_common::model::package::Package;
use huber_common::model::release::Release;
use log::{info, warn};
use maplit::hashmap;
use semver::{Version, VersionReq};
use simpledi_rs::di::{DIContainer, DIContainerTrait};
use tokio::task::JoinHandle;

use crate::cmd::{CommandTrait, PlatformStdLib};
use crate::error::HuberError::{PackageNotInstalled, PackageUnableToUpdate};
use crate::lock_huber_ops;
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::ItemOperationTrait;

#[derive(Args)]
pub struct UpdateArgs {
    #[arg(help = "Package name", num_args = 1, value_hint = ValueHint::Unknown)]
    name: Vec<String>,

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

    #[arg(
        help = "Dry run to show available updates",
        long,
        num_args = 0,
        value_hint = ValueHint::Unknown
    )]
    dryrun: bool,
}

#[async_trait]
impl CommandTrait for UpdateArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        lock_huber_ops!(config);

        let release_service = Arc::new(container.get::<ReleaseService>().unwrap().clone());
        let pkg_service = Arc::new(container.get::<PackageService>().unwrap().clone());
        let config = Arc::new(config.clone());

        for name in self.name.iter() {
            if !release_service.has(name)? {
                return Err(anyhow!(PackageUnableToUpdate(anyhow!(
                    PackageNotInstalled(name.clone())
                ))));
            }
        }

        let installed_latest_pkg_releases = if self.name.is_empty() {
            get_installed_latest_pkg_releases(&release_service)?
        } else {
            get_installed_latest_pkg_releases(&release_service)?
                .into_iter()
                .filter(|(name, _)| self.name.contains(name))
                .collect()
        };

        let mut join_handles: Vec<JoinHandle<anyhow::Result<()>>> = vec![];
        for (name, installed_release) in installed_latest_pkg_releases {
            let release_service = release_service.clone();
            let pkg_service = pkg_service.clone();
            let config = config.clone();
            let dryrun = self.dryrun;
            let prefer_stdlib = self.prefer_stdlib;

            let handle: JoinHandle<_> = tokio::spawn(async move {
                info!(
                    "Checking for updates for {}. The latest installed version is {}",
                    name, installed_release.version
                );

                let pkg = pkg_service.get(&name)?;
                let new_release = release_service.get_latest(&pkg).await?;

                info!(
                    "Found the latest version of {}: {}",
                    name, new_release.version
                );
                if is_pkg_locked_for_release(&config, &pkg, &new_release.version) {
                    warn!(
                        "Package {} is locked to version {}. Skipping updating to {}",
                        pkg.name,
                        config.lock_pkg_versions.get(&pkg.name).unwrap(),
                        new_release.version
                    );
                    return Ok(());
                }

                if new_release.compare(&installed_release)? == Ordering::Greater {
                    info!(
                        "Updating package {} from {} to {}",
                        name, installed_release.version, new_release.version
                    );
                    update(
                        release_service,
                        dryrun,
                        &new_release,
                        &installed_release,
                        prefer_stdlib,
                    )
                    .await?;
                    info!(
                        "Package {} updated to {} successfully",
                        name, new_release.version
                    );
                } else {
                    info!(
                    "Nothing to update, as the currently installed version ({}) is equal to or \
                higher than the found version ({})",
                    installed_release.version, new_release.version
                );
                }

                Ok(())
            });
            join_handles.push(handle);
        }

        for handle in join_handles.into_iter() {
            handle.await??;
        }

        Ok(())
    }
}

fn get_installed_latest_pkg_releases(
    release_service: &ReleaseService,
) -> anyhow::Result<HashMap<String, Release>> {
    let mut installed_latest_pkg_releases: HashMap<String, Release> = hashmap! {};

    for release in release_service.list()? {
        if let Some(existing_release) = installed_latest_pkg_releases.get(&release.name) {
            if release.compare(existing_release)? == Ordering::Greater {
                installed_latest_pkg_releases.insert(release.name.clone(), release);
            }
        } else {
            installed_latest_pkg_releases.insert(release.name.clone(), release);
        }
    }
    Ok(installed_latest_pkg_releases)
}

pub fn is_pkg_locked_for_release(
    config: &Config,
    pkg: &Package,
    new_release_version: &str,
) -> bool {
    if let Some(lock_version) = config.lock_pkg_versions.get(&pkg.name) {
        let lock_version = lock_version.trim_start_matches("v");
        let lock_version = VersionReq::parse(lock_version).unwrap();
        let new_version = Version::parse(new_release_version.trim_start_matches("v")).unwrap();

        return !lock_version.matches(&new_version);
    }

    false
}

async fn update(
    release_service: Arc<ReleaseService>,
    dryrun: bool,
    new_release: &Release,
    installed_release: &Release,
    prefer_stdlib: Option<PlatformStdLib>,
) -> anyhow::Result<()> {
    if dryrun {
        info!("Available update {} to {}", installed_release, new_release);
    } else {
        info!("Updating {} to {}", installed_release, new_release);
        release_service
            .update(&new_release.package, prefer_stdlib)
            .await?;
    }

    Ok(())
}
