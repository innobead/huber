use std::io::stdout;

use async_trait::async_trait;
use clap::{ArgMatches, Command};
use libcli_rs::output::{OutputFactory, OutputTrait};
use simpledi_rs::di::DIContainer;
use simpledi_rs::di::DIContainerTrait;

use huber_common::model::config::Config;
use huber_common::result::Result;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::config::{ConfigService, ConfigTrait};

pub(crate) const CMD_NAME: &str = "show";

#[derive(Debug)]
pub(crate) struct ConfigShowCmd;

unsafe impl Send for ConfigShowCmd {}

unsafe impl Sync for ConfigShowCmd {}

impl ConfigShowCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'help> CommandTrait<'help> for ConfigShowCmd {
    fn app(&self) -> Command<'help> {
        Command::new(CMD_NAME)
            .visible_alias("s")
            .about("Shows the configuration")
    }
}

#[async_trait]
impl CommandAsyncTrait for ConfigShowCmd {
    async fn run(
        &self,
        config: &Config,
        container: &DIContainer,
        _matches: &ArgMatches,
    ) -> Result<()> {
        let config_service = container.get::<ConfigService>().unwrap();

        let c = config_service.get()?;
        output!(config.output_format, .display(
            stdout(),
            &c,
            None,
            Some(vec!["home_dir"]),
        ))
    }
}
