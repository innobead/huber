use async_trait::async_trait;
use clap::{App, ArgMatches};

use huber_common::model::config::Config;
use huber_common::result::Result;
use huber_procmacro::process_lock;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::cache::{CacheAsyncTrait, CacheService};
use crate::service::update::{UpdateAsyncTrait, UpdateService};
use huber_common::model::config::ConfigPath;

pub(crate) const CMD_NAME: &str = "self-update";

#[derive(Debug)]
pub(crate) struct SelfUpdateCmd;

unsafe impl Send for SelfUpdateCmd {}

unsafe impl Sync for SelfUpdateCmd {}

impl SelfUpdateCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for SelfUpdateCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("su")
            .about("Updates huber")
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for SelfUpdateCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        _matches: &ArgMatches<'a>,
    ) -> Result<()> {
        process_lock!();

        let cache_service = container.get::<CacheService>().unwrap();
        let _ = cache_service.update_repositories().await?;

        let update_service = container.get::<UpdateService>().unwrap();

        let r = update_service.has_update().await?;
        if r.0 {
            println!("Updating huber to {}", r.1);
            if let Err(e) = update_service.update().await {
                return Err(anyhow!("Failed to update, {:?}", e));
            }

            println!("huber {} Updated", r.1);
            return Ok(());
        }

        println!("No update available");
        Ok(())
    }
}
