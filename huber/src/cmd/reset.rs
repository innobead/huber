use async_trait::async_trait;
use clap::Args;
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::lock_huber_ops;
use crate::model::config::Config;
use crate::service::update::{HuberUpdateService, UpdateTrait};

#[derive(Args)]
pub struct ResetArgs {}

#[async_trait]
impl CommandTrait for ResetArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        lock_huber_ops!(config);

        let update_service = container.get::<HuberUpdateService>().unwrap();

        info!(
            "Resetting Huber by removing created caches, downloaded files and installed packages"
        );
        update_service.reset()?;
        info!("Huber reset");

        Ok(())
    }
}
