use std::io::stdout;

use anyhow::anyhow;
use async_trait::async_trait;
use clap::{Args, ValueHint};
use huber_common::model::config::Config;
use libcli_rs::output;
use libcli_rs::output::{OutputFactory, OutputTrait};
use log::info;
use serde::{Deserialize, Serialize};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::error::HuberError::{
    NoPackagesLocked, PackageNotFound, PackageNotInstalled, PackageUnableToLock,
};
use crate::opt::parse_pkg_name_semver_req;
use crate::service::config::{ConfigService, ConfigTrait};
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::ItemOperationTrait;

#[derive(Args)]
pub struct LockArgs {
    #[arg(
        help = "Package name (e.g. 'package-name', 'package-name@version' or \
        'package-name@<version-requirement>' \
        using Cargo's dependency version requirement format)",
        num_args = 1,
        value_hint = ValueHint::Unknown,
        value_parser = parse_pkg_name_semver_req,
    )]
    name_version: Vec<(String, String)>,

    #[arg(
        help = "Lock all installed `current` packages",
        long,
        conflicts_with = "name_version",
        value_hint = ValueHint::Unknown
    )]
    all: bool,
}

#[async_trait]
impl CommandTrait for LockArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        if !self.all && self.name_version.is_empty() {
            info!("No packages specified to lock. Showing locked packages instead");
            return display_locked_pkgs(config);
        }

        let pkg_service = container.get::<PackageService>().unwrap();
        let release_service = container.get::<ReleaseService>().unwrap();
        let config_service = container.get::<ConfigService>().unwrap();

        info!("Locking packages");

        let mut config = config.clone();
        if self.all {
            lock_installed_current_pkgs(&mut config, release_service)?;
        } else {
            lock_pkgs(
                &mut config,
                pkg_service,
                release_service,
                &self.name_version,
            )?;
        }

        config_service.update(&config)?;
        info!("Packages locked successfully");

        Ok(())
    }
}

fn lock_pkgs(
    config: &mut Config,
    pkg_service: &PackageService,
    release_service: &ReleaseService,
    name_versions: &Vec<(String, String)>,
) -> anyhow::Result<()> {
    for (pkg, version) in name_versions {
        if !pkg_service.has(pkg)? {
            return Err(anyhow!(PackageNotFound(pkg.clone())));
        }

        if !release_service.has(pkg)? {
            return Err(anyhow!(PackageUnableToLock(anyhow!(PackageNotInstalled(
                pkg.clone()
            )))));
        }

        info!("Locking package: {}@{}", pkg, version);
        let versions = &mut config.lock_pkg_versions;
        versions.insert(pkg.clone(), version.clone());
    }

    Ok(())
}

fn lock_installed_current_pkgs(
    config: &mut Config,
    release_service: &ReleaseService,
) -> anyhow::Result<()> {
    for r in &release_service.list()? {
        if !r.current {
            continue;
        }

        info!("Locking package: {}@{}", r.name, r.version);
        let versions = &mut config.lock_pkg_versions;
        versions.insert(r.name.clone(), r.version.clone());
    }

    Ok(())
}

fn display_locked_pkgs(config: &Config) -> anyhow::Result<()> {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct PkgVersionInfo {
        name: String,
        version: String,
    }

    let pkg_version_infos: Vec<_> = config
        .lock_pkg_versions
        .iter()
        .map(|(name, version)| PkgVersionInfo {
            name: name.clone(),
            version: version.clone(),
        })
        .collect();

    if pkg_version_infos.is_empty() {
        return Err(anyhow!(NoPackagesLocked));
    }

    output!(
        config.output_format,
        .display(stdout(), &pkg_version_infos, None, None)
    )
}
