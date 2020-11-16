use async_trait::async_trait;
use clap::{App, ArgMatches};

use huber_common::config::Config;
use huber_common::di::DIContainer;
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

impl<'a, 'b> CommandTrait<'a, 'b> for RepoCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Manages repositories")
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for RepoCmd {
    async fn run(
        &self,
        config: &Config,
        container: &DIContainer,
        matches: &ArgMatches<'a>,
    ) -> Result<()> {
        match matches.subcommand() {
            (cmd::repo::add::CMD_NAME, Some(sub_matches)) => {
                container
                    .get::<RepoAddCmd>()
                    .unwrap()
                    .run(config, container, sub_matches)
                    .await
            }

            (cmd::repo::remove::CMD_NAME, Some(sub_matches)) => {
                container
                    .get::<RepoRemoveCmd>()
                    .unwrap()
                    .run(config, container, sub_matches)
                    .await
            }

            (cmd::repo::list::CMD_NAME, Some(sub_matches)) => {
                container
                    .get::<RepoListCmd>()
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
