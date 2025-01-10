use anyhow::anyhow;
use async_trait::async_trait;
use clap::{Args, ValueHint};
use huber_common::model::config::Config;
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::error::HuberError::{PackageNotFound, PackageNotInstalled};
use crate::service::config::{ConfigService, ConfigTrait};
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::ItemOperationTrait;

#[derive(Args)]
pub struct UnlockArgs {
    #[arg(
        help = "Package name",
        num_args = 1,
        group = "lock",
        required = true,
        value_hint = ValueHint::Unknown
    )]
    name: Vec<String>,

    #[arg(
        help = "Unlock all the locked packages",
        long,
        group = "lock",
        required = true,
        value_hint = ValueHint::Unknown
    )]
    all: bool,
}

#[async_trait]
impl CommandTrait for UnlockArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let pkg_service = container.get::<PackageService>().unwrap();
        let release_service = container.get::<ReleaseService>().unwrap();
        let config_service = container.get::<ConfigService>().unwrap();
        let mut config = config.clone();

        info!("Unlocking packages");

        if self.all {
            info!("Unlocking all packages");
            config.lock_pkg_versions.clear();
        } else {
            unlock_pkgs(&mut config, pkg_service, release_service, &self.name)?;
        }

        config_service.update(&config)?;
        info!("Unlocked packages");

        Ok(())
    }
}

fn unlock_pkgs(
    config: &mut Config,
    pkg_service: &PackageService,
    release_service: &ReleaseService,
    pkgs: &Vec<String>,
) -> anyhow::Result<()> {
    for pkg in pkgs {
        if !pkg_service.has(pkg)? {
            return Err(anyhow!(PackageNotFound(pkg.clone())));
        }

        if !release_service.has(pkg)? {
            return Err(anyhow!(PackageNotInstalled(pkg.clone())));
        }

        info!("Unlocking package: {}", pkg);
        config.lock_pkg_versions.remove(pkg);
    }

    Ok(())
}