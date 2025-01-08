use anyhow::anyhow;
use async_trait::async_trait;
use clap::{Args, ValueHint};
use huber_common::model::config::Config;
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::error::HuberError::PackageNotFound;
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::ItemOperationTrait;

#[derive(Args)]
pub struct UninstallArgs {
    #[arg(help = "Package name", num_args = 1, value_hint = ValueHint::Unknown)]
    name: Vec<String>,
}

#[async_trait]
impl CommandTrait for UninstallArgs {
    async fn run(&self, _: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let pkg_service = container.get::<PackageService>().unwrap();
        let release_service = container.get::<ReleaseService>().unwrap();

        for name in self.name.iter() {
            if !pkg_service.has(name)? {
                return Err(anyhow!(PackageNotFound(name.clone())));
            }
        }

        for name in self.name.iter() {
            info!("Uninstalling {}", name);
            release_service.delete(name)?;
            info!("Uninstalled {}", name);
        }

        Ok(())
    }
}
