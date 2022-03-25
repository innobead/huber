use async_trait::async_trait;
use clap::{ArgMatches, Command};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use huber_common::model::config::Config;
use huber_common::model::config::ConfigPath;
use huber_common::progress::progress;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::update::{UpdateService, UpdateTrait};

pub(crate) const CMD_NAME: &str = "reset";

#[derive(Debug)]
pub(crate) struct ResetCmd;

unsafe impl Send for ResetCmd {}

unsafe impl Sync for ResetCmd {}

impl ResetCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'help> CommandTrait<'help> for ResetCmd {
    fn app(&self) -> Command<'help> {
        Command::new(CMD_NAME)
            .visible_alias("r")
            .about("Resets huber")
            .long_about(
                "Resetting huber means the generated data by huber will be removed \
            like the installed packages, created caches and index files, then have the clean state.",
            )
    }
}

#[async_trait]
impl CommandAsyncTrait for ResetCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        _matches: &ArgMatches,
    ) -> Result<()> {
        process_lock!();

        let update_service = container.get::<UpdateService>().unwrap();

        progress(
            "Resetting huber by removing created caches, downloaded files and installed packages",
        )?;
        update_service.reset()?;

        println!("Done");
        Ok(())
    }
}
