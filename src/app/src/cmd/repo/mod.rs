use clap::{App, ArgMatches};

use huber_common::config::Config;
use huber_common::di::{di_container, DIContainer};
use huber_common::result::Result;

use crate::cmd;
use crate::cmd::repo::add::RepoAddCmd;
use crate::cmd::repo::list::RepoListCmd;
use crate::cmd::repo::remove::RepoRemoveCmd;
use crate::cmd::CommandTrait;

pub(crate) mod add;
pub(crate) mod list;
pub(crate) mod remove;

pub(crate) const CMD_NAME: &str = "repo";

pub(crate) struct RepoCmd;

impl RepoCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for RepoCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .about("Manages repositories")
            .subcommands(vec![
                di!(RepoAddCmd.app()),
                di!(RepoRemoveCmd.app()),
                di!(RepoListCmd.app()),
            ])
    }

    fn run(&self, config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        match matches.subcommand() {
            (cmd::repo::add::CMD_NAME, Some(sub_matches)) => di_container()
                .get::<RepoAddCmd>()
                .unwrap()
                .run(config, sub_matches),

            (cmd::repo::remove::CMD_NAME, Some(sub_matches)) => di_container()
                .get::<RepoRemoveCmd>()
                .unwrap()
                .run(config, sub_matches),

            (cmd::repo::list::CMD_NAME, Some(sub_matches)) => di_container()
                .get::<RepoListCmd>()
                .unwrap()
                .run(config, sub_matches),

            _ => {
                println!("{}", matches.usage());
                Ok(())
            }
        }
    }
}
