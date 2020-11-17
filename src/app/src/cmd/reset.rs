use async_trait::async_trait;
use clap::{App, ArgMatches};

use huber_common::di::DIContainer;
use huber_common::model::config::Config;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::update::{UpdateService, UpdateTrait};
use huber_common::model::config::ConfigPath;

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

impl<'a, 'b> CommandTrait<'a, 'b> for ResetCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("r")
            .about("Resets huber")
            .long_about(
                "Resetting huber means the generated data by huber will be removed \
            like the installed packages, created caches and index files, then have the clean state.",
            )
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for ResetCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        _matches: &ArgMatches<'a>,
    ) -> Result<()> {
        process_lock!();

        let update_service = container.get::<UpdateService>().unwrap();

        println!(
            "Resetting huber by removing created caches, downloaded files and installed packages"
        );
        update_service.reset()?;
        println!("Done");

        Ok(())
    }
}
