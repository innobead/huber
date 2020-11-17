use std::io::stdout;

use async_trait::async_trait;
use clap::{App, ArgMatches};

use huber_common::di::DIContainer;
use huber_common::model::config::Config;
use huber_common::output::factory::FactoryConsole;
use huber_common::result::Result;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use huber_common::output::OutputTrait;
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
            .visible_alias("rm")
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
