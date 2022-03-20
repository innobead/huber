use std::io::stdout;

use async_trait::async_trait;
use clap::{ArgMatches, Command};
use libcli_rs::output::{OutputFactory, OutputTrait};
use simpledi_rs::di::DIContainer;
use simpledi_rs::di::DIContainerTrait;

use huber_common::model::config::Config;
use huber_common::model::config::ConfigPath;
use huber_common::progress::progress;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{update_config_by_arg_matches, CommandAsyncTrait, CommandTrait};
use crate::service::config::{ConfigService, ConfigTrait};

pub(crate) const CMD_NAME: &str = "update";

#[derive(Debug)]
pub(crate) struct ConfigUpdateCmd;

unsafe impl Send for ConfigUpdateCmd {}

unsafe impl Sync for ConfigUpdateCmd {}

impl ConfigUpdateCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'help> CommandTrait<'help> for ConfigUpdateCmd {
    fn app(&self) -> Command<'help> {
        Command::new(CMD_NAME)
            .visible_alias("u")
            .about("Updates the configuration")
    }
}

#[async_trait]
impl CommandAsyncTrait for ConfigUpdateCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        matches: &ArgMatches,
    ) -> Result<()> {
        process_lock!();

        let config_service = container.get::<ConfigService>().unwrap();

        progress("Updating the configuration")?;

        let mut c = config_service.get()?;
        if update_config_by_arg_matches(&mut c, &matches) {
            config_service.update(&c)?;
            println!("The configuration updated\n");
        } else {
            println!("Nothing changed. Please specify options for updating the configuration\n")
        }

        output!(c.output_format, .display(
            stdout(),
            &c,
            None,
            Some(vec!["home_dir"]),
        ))
    }
}
