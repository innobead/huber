use anyhow::anyhow;
use async_trait::async_trait;
use clap::{Args, ValueHint};
use huber_common::model::config::Config;
use log::{info, warn};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::lock_huber_ops;
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::{check_pkg_installed, ItemOperationTrait};

#[derive(Args)]
pub struct UninstallArgs {
    #[arg(help = "Package name", num_args = 1, value_hint = ValueHint::Unknown)]
    name: Vec<String>,
}

#[async_trait]
impl CommandTrait for UninstallArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        lock_huber_ops!(config);

        let pkg_service = container.get::<PackageService>().unwrap();
        let release_service = container.get::<ReleaseService>().unwrap();

        for name in self.name.iter() {
            if let Err(e) = check_pkg_installed(pkg_service, release_service, name) {
                warn!("{}", e);
                continue;
            }

            info!("Uninstalling {}", name);
            release_service.delete(name)?;
            info!("Uninstalled {}", name);
        }

        Ok(())
    }
}
