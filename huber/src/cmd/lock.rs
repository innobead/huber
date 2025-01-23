use std::io::stdout;

use anyhow::anyhow;
use async_trait::async_trait;
use clap::{Args, Subcommand, ValueHint};
use huber_common::model::config::Config;
use libcli_rs::output;
use libcli_rs::output::{OutputFactory, OutputTrait};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::lock_huber_ops;
use crate::opt::parse_pkg_name_semver_req;
use crate::service::config::{ConfigService, ConfigTrait};
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::{check_pkg_installed, ItemOperationTrait};

#[derive(Args)]
pub struct LockArgs {
    #[arg(
        help = "Package name (e.g. 'package-name', 'package-name@semver' or \
        'package-name@<semver-requirement>' \
        using Cargo's dependency version requirement format)",
        num_args = 1,
        group = "lock",
        value_hint = ValueHint::Unknown,
        value_parser = parse_pkg_name_semver_req,
    )]
    pub name_version: Vec<(String, String)>,

    #[arg(
        help = "Lock all installed `current` packages",
        long,
        group = "lock",
        conflicts_with = "name_version",
        num_args = 0,
        value_hint = ValueHint::Unknown
    )]
    pub all: bool,

    #[arg(
        help = "Treat version requirement as a caret requirement if \
        no version requirement is specified",
        long,
        num_args = 0,
        value_hint = ValueHint::Unknown
    )]
    pub caret_required: bool,

    #[arg(
        help = "Treat version requirement as a tilde requirement if \
        no version requirement is specified",
        long,
        num_args = 0,
        value_hint = ValueHint::Unknown
    )]
    pub tilde_required: bool,

    #[command(subcommand)]
    pub command: Option<LockCommands>,
}

#[derive(Subcommand)]
pub enum LockCommands {
    #[command(about = "Show locked versions", bin_name = "show")]
    Show(LockShowArgs),
}

#[derive(Args)]
pub struct LockShowArgs {}

#[async_trait]
impl CommandTrait for LockShowArgs {
    async fn run(&self, config: &Config, _: &DIContainer) -> anyhow::Result<()> {
        display_locked_pkgs(config)
    }
}

#[async_trait]
impl CommandTrait for LockArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        lock_huber_ops!(config);

        let pkg_service = container.get::<PackageService>().unwrap();
        let release_service = container.get::<ReleaseService>().unwrap();
        let config_service = container.get::<ConfigService>().unwrap();

        info!("Locking packages");

        let old_config = config.clone();
        let mut config = config.clone();

        if self.all {
            lock_installed_current_pkgs(
                &mut config,
                release_service,
                self.caret_required,
                self.tilde_required,
            )?;
        } else {
            if self.name_version.is_empty() {
                info!("No packages specified to lock");
                return Ok(());
            }

            lock_pkgs(
                &mut config,
                pkg_service,
                release_service,
                &self.name_version,
                self.caret_required,
                self.tilde_required,
            )?;
        }

        if old_config.lock_pkg_versions != config.lock_pkg_versions {
            config_service.update(&config)?;
            info!(
                "Packages locked successfully: {:#?}",
                config.lock_pkg_versions
            );
            return Ok(());
        }

        Ok(())
    }
}

fn lock_pkgs(
    config: &mut Config,
    pkg_service: &PackageService,
    release_service: &ReleaseService,
    name_versions: &Vec<(String, String)>,
    caret_required: bool,
    tilde_required: bool,
) -> anyhow::Result<()> {
    for (pkg, version) in name_versions {
        if let Err(e) = check_pkg_installed(pkg_service, release_service, pkg) {
            warn!("Skipped locking package {}@{}: {}", pkg, version, e);
            continue;
        }

        let version = get_version_requirement(caret_required, tilde_required, version);
        info!("Locking package: {}@{}", pkg, version);

        let versions = &mut config.lock_pkg_versions;
        versions.insert(pkg.clone(), version);
    }

    Ok(())
}

fn get_version_requirement(caret_required: bool, tilde_required: bool, version: &str) -> String {
    let version = version.trim_start_matches("v");
    if caret_required {
        format!("^{}", version)
    } else if tilde_required {
        format!("~{}", version)
    } else {
        version.to_string()
    }
}

fn lock_installed_current_pkgs(
    config: &mut Config,
    release_service: &ReleaseService,
    caret_required: bool,
    tilde_required: bool,
) -> anyhow::Result<()> {
    let releases = release_service.list()?;
    if releases.is_empty() {
        info!("No packages installed. Nothing to lock");
        return Ok(());
    }

    for ref r in releases {
        if !r.current {
            continue;
        }

        let version = get_version_requirement(caret_required, tilde_required, &r.version);
        info!("Locking package: {}@{}", r.name, version);
        let versions = &mut config.lock_pkg_versions;
        versions.insert(r.name.clone(), version);
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
        info!("No packages locked");
        return Ok(());
    }

    output!(
        config.output_format,
        .display(stdout(), &pkg_version_infos, None, None)
    )
}
