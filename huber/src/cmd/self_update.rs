use async_trait::async_trait;
use clap::Args;
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::get_default_stdlib;
use crate::cmd::{CommandTrait, PlatformStdLib};
use crate::lock_huber_ops;
use crate::model::config::Config;
use crate::service::cache::{CacheAsyncTrait, CacheService};
use crate::service::update::{HuberUpdateService, UpdateAsyncTrait};

#[derive(Args)]
pub struct SelfUpdateArgs {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[arg(
        help = "Prefer standard library (only for Linux or Windows)",
        long,
        num_args = 1,
        default_value_t = get_default_stdlib(),
        value_enum
    )]
    prefer_stdlib: PlatformStdLib,

    #[cfg(target_os = "macos")]
    #[arg(
        help = "Prefer standard library (only for Linux or Windows)",
        long,
        hide = true,
        num_args = 1,
        default_value_t = get_default_stdlib(),
        value_enum
    )]
    prefer_stdlib: PlatformStdLib,
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
            update_service.update(&self.prefer_stdlib).await?;
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
