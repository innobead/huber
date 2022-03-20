use async_trait::async_trait;
use clap::{ArgMatches, Command};
use simpledi_rs::di::DIContainer;
use simpledi_rs::di::DIContainerTrait;

use huber_common::model::config::Config;
use huber_common::result::Result;

use crate::cmd;
use crate::cmd::config::show::ConfigShowCmd;
use crate::cmd::config::update::ConfigUpdateCmd;
use crate::cmd::{CommandAsyncTrait, CommandTrait};

pub(crate) mod show;
pub(crate) mod update;

pub(crate) const CMD_NAME: &str = "config";

#[derive(Debug)]
pub(crate) struct ConfigCmd;

unsafe impl Send for ConfigCmd {}

unsafe impl Sync for ConfigCmd {}

impl ConfigCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'help> CommandTrait<'help> for ConfigCmd {
    fn app(&self) -> Command<'help> {
        Command::new(CMD_NAME).about("Manages the configuration")
    }
}

#[async_trait]
impl CommandAsyncTrait for ConfigCmd {
    async fn run(
        &self,
        config: &Config,
        container: &DIContainer,
        matches: &ArgMatches,
    ) -> Result<()> {
        match matches.subcommand() {
            Some((cmd::config::show::CMD_NAME, sub_matches)) => {
                container
                    .get::<ConfigShowCmd>()
                    .unwrap()
                    .run(config, container, sub_matches)
                    .await
            }

            Some((cmd::config::update::CMD_NAME, sub_matches)) => {
                container
                    .get::<ConfigUpdateCmd>()
                    .unwrap()
                    .run(config, container, sub_matches)
                    .await
            }

            _ => Err(anyhow!("Command not found")),
        }
    }
}

pub(crate) const ARG_LOG_LEVEL: &str = "log-level";
pub(crate) const ARG_OUTPUT_TYPE: &str = "output";
pub(crate) const ARG_GITHUB_TOKEN: &str = "github-token";
pub(crate) const ARG_GITHUB_KEY: &str = "github-key";
