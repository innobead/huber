use anyhow::anyhow;
use async_trait::async_trait;
use clap::{Args, ValueHint};
use huber_common::model::config::Config;
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::service::config::{ConfigService, ConfigTrait};
use crate::service::package::PackageService;
use crate::service::ItemOperationTrait;

#[derive(Args)]
pub struct UnlockArgs {
    #[arg(help = "Package name", num_args = 1, value_hint = ValueHint::Unknown)]
    name: Vec<String>,
}

#[async_trait]
impl CommandTrait for UnlockArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let pkg_service = container.get::<PackageService>().unwrap();
        let mut config = config.clone();
        let mut require_update = false;

        for pkg in &self.name {
            if pkg_service.has(pkg)? {
                return Err(anyhow!("Package {} not found", pkg));
            }

            if !require_update {
                require_update = true;
            }

            info!("Unlocking package: {}", pkg);
            config.lock_pkg_versions.remove(pkg);
        }

        if !require_update {
            info!("No packages to unlock");
            return Ok(());
        }

        let config_service = container.get::<ConfigService>().unwrap();
        config_service.update(&config)?;
        info!("Unlocked packages");

        Ok(())
    }
}
