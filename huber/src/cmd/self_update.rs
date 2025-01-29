use async_trait::async_trait;
use clap::Args;
use huber_common::model::config::Config;
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::{CommandTrait, PlatformStdLib};
use crate::lock_huber_ops;
use crate::service::cache::{CacheAsyncTrait, CacheService};
use crate::service::update::{HuberUpdateService, UpdateAsyncTrait};

#[derive(Args)]
pub struct SelfUpdateArgs {
    #[arg(
        help = "Prefer standard library (only for Linux or Windows)",
        long,
        num_args = 1,
        value_enum,
    )]
    prefer_stdlib: Option<PlatformStdLib>,
}

#[async_trait]
impl CommandTrait for SelfUpdateArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        lock_huber_ops!(config);

        let cache_service = container.get::<CacheService>().unwrap();
        let _ = cache_service.update_repositories().await?;

        let update_service = container.get::<HuberUpdateService>().unwrap();
        let (has_update, version) = update_service.has_update().await?;

        if has_update {
            info!("Updating Huber {}", version);
            update_service.update(self.prefer_stdlib).await?;
            info!("Huber updated to {}", version);
        } else {
            info!(
                "No update available. The latest version {:?} already installed.",
                env!("HUBER_VERSION")
            );
        }

        Ok(())
    }
}
