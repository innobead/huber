use async_trait::async_trait;
use clap::{ArgMatches, Command};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use huber_common::model::config::Config;
use huber_common::result::Result;

use crate::cmd;
use crate::cmd::repo::add::RepoAddCmd;
use crate::cmd::repo::list::RepoListCmd;
use crate::cmd::repo::remove::RepoRemoveCmd;
use crate::cmd::{CommandAsyncTrait, CommandTrait};

pub(crate) mod add;
pub(crate) mod list;
pub(crate) mod remove;

pub(crate) const CMD_NAME: &str = "repo";

#[derive(Debug)]
pub(crate) struct RepoCmd;

unsafe impl Send for RepoCmd {}

unsafe impl Sync for RepoCmd {}

impl RepoCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'help> CommandTrait<'help> for RepoCmd {
    fn app(&self) -> Command<'help> {
        Command::new(CMD_NAME).about("Manages repositories")
    }
}

#[async_trait]
impl CommandAsyncTrait for RepoCmd {
    async fn run(
        &self,
        config: &Config,
        container: &DIContainer,
        matches: &ArgMatches,
    ) -> Result<()> {
        match matches.subcommand() {
            Some((cmd::repo::add::CMD_NAME, sub_matches)) => {
                container
                    .get::<RepoAddCmd>()
                    .unwrap()
                    .run(config, container, sub_matches)
                    .await
            }

            Some((cmd::repo::remove::CMD_NAME, sub_matches)) => {
                container
                    .get::<RepoRemoveCmd>()
                    .unwrap()
                    .run(config, container, sub_matches)
                    .await
            }

            Some((cmd::repo::list::CMD_NAME, sub_matches)) => {
                container
                    .get::<RepoListCmd>()
                    .unwrap()
                    .run(config, container, sub_matches)
                    .await
            }

            _ => Err(anyhow!("Command not found")),
        }
    }
}
