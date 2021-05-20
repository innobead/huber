use std::io::stdout;

use async_trait::async_trait;
use clap::{App, ArgMatches};
use simpledi_rs::di::DIContainer;
use simpledi_rs::di::DIContainerTrait;

use huber_common::model::config::Config;
use huber_common::result::Result;
use libcli_rs::output::{OutputFactory, OutputTrait};

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

impl<'a, 'b> CommandTrait<'a, 'b> for ConfigShowCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("s")
            .about("Shows the configuration")
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for ConfigShowCmd {
    async fn run(
        &self,
        config: &Config,
        container: &DIContainer,
        _matches: &ArgMatches<'a>,
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
