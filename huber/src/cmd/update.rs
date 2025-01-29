use std::cmp::Ordering;
use std::collections::HashMap;

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

    #[arg(
        help = "Prefer standard library (only for Linux or Windows)",
        long,
        num_args = 1,
        value_enum,
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

        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        for name in self.name.iter() {
            if !release_service.has(name)? {
                return Err(anyhow!(PackageUnableToUpdate(anyhow!(
                    PackageNotInstalled(name.clone())
                ))));
            }
        }

        let mut installed_latest_pkg_releases: HashMap<String, Release> = hashmap! {};
        get_installed_latest_pkg_releases(release_service, &mut installed_latest_pkg_releases)?;

        for (name, installed_release) in installed_latest_pkg_releases.iter() {
            info!(
                "Checking for updates for {}. The latest installed version is {}",
                name, installed_release.version
            );

            let pkg = pkg_service.get(name)?;
            let new_release = release_service.get_latest(&pkg).await?;

            info!(
                "Found the latest version of {}: {}",
                name, new_release.version
            );
            if !is_pkg_updatable(config, &pkg, &new_release) {
                warn!(
                    "Package {} is locked to version {}. Skipping update to {}",
                    pkg.name,
                    config.lock_pkg_versions.get(&pkg.name).unwrap(),
                    new_release.version
                );
                continue;
            }

            if new_release.compare(installed_release)? == Ordering::Greater {
                info!(
                    "Updating package {} from {} to {}",
                    name, installed_release.version, new_release.version
                );
                update(
                    release_service,
                    self.dryrun,
                    &new_release,
                    installed_release,
                    self.prefer_stdlib,
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
        }

        Ok(())
    }
}

fn get_installed_latest_pkg_releases(
    release_service: &ReleaseService,
    installed_latest_pkg_releases: &mut HashMap<String, Release>,
) -> anyhow::Result<()> {
    for release in release_service.list()? {
        if let Some(existing_release) = installed_latest_pkg_releases.get(&release.name) {
            if release.compare(existing_release)? == Ordering::Greater {
                installed_latest_pkg_releases.insert(release.name.clone(), release);
            }
        } else {
            installed_latest_pkg_releases.insert(release.name.clone(), release);
        }
    }
    Ok(())
}

fn is_pkg_updatable(config: &Config, pkg: &Package, new_release: &Release) -> bool {
    if let Some(lock_version) = config.lock_pkg_versions.get(&pkg.name) {
        let lock_version = lock_version.trim_start_matches("v");
        let new_version = Version::parse(new_release.version.trim_start_matches("v")).unwrap();

        if Version::parse(lock_version).is_ok() {
            return false;
        }

        let lock_version = VersionReq::parse(lock_version).unwrap();
        return lock_version.matches(&new_version);
    }

    true
}

async fn update(
    release_service: &ReleaseService,
    dryrun: bool,
    new_release: &Release,
    installed_release: &Release,
    prefer_stdlib: Option<PlatformStdLib>,
) -> anyhow::Result<()> {
    if dryrun {
        info!("Available update {} to {}", installed_release, new_release);
    } else {
        info!("Updating {} to {}", installed_release, new_release);
        release_service.update(&new_release.package, prefer_stdlib).await?;
    }

    Ok(())
}
