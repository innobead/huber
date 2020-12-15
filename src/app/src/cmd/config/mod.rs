use async_trait::async_trait;
use clap::{App, ArgMatches};

use huber_common::model::config::Config;
use huber_common::result::Result;
use simpledi_rs::di::DIContainer;

use crate::cmd;
use crate::cmd::config::show::ConfigShowCmd;
use crate::cmd::config::update::ConfigUpdateCmd;
use crate::cmd::{CommandAsyncTrait, CommandTrait};

use simpledi_rs::di::DIContainerTrait;

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

impl<'a, 'b> CommandTrait<'a, 'b> for ConfigCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Manages the configuration")
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for ConfigCmd {
    async fn run(
        &self,
        config: &Config,
        container: &DIContainer,
        matches: &ArgMatches<'a>,
    ) -> Result<()> {
        match matches.subcommand() {
            (cmd::config::show::CMD_NAME, Some(sub_matches)) => {
                container
                    .get::<ConfigShowCmd>()
                    .unwrap()
                    .run(config, container, sub_matches)
                    .await
            }

            (cmd::config::update::CMD_NAME, Some(sub_matches)) => {
                container
                    .get::<ConfigUpdateCmd>()
                    .unwrap()
                    .run(config, container, sub_matches)
                    .await
            }

            _ => {
                println!("{}", matches.usage());
                Ok(())
            }
        }
    }
}

pub(crate) const ARG_LOG_LEVEL: &str = "log-level";
pub(crate) const ARG_OUTPUT_TYPE: &str = "output";
pub(crate) const ARG_GITHUB_TOKEN: &str = "github-token";
pub(crate) const ARG_GITHUB_KEY: &str = "github-key";
