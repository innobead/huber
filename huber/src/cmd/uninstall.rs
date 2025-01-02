use anyhow::anyhow;
use async_trait::async_trait;
use clap::Args;
use huber_common::model::config::Config;
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::ItemOperationTrait;

#[derive(Args)]
pub struct UninstallArgs {
    #[arg(help = "Package name")]
    name: Vec<String>,
}

#[async_trait]
impl CommandTrait for UninstallArgs {
    async fn run(&self, _: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let pkg_service = container.get::<PackageService>().unwrap();
        let release_service = container.get::<ReleaseService>().unwrap();

        for name in self.name.iter() {
            if !pkg_service.has(name)? {
                return Err(anyhow!("{} package not found", name));
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
