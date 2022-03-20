use std::io::stdout;

use async_trait::async_trait;
use clap::{ArgMatches, Command};
use libcli_rs::output::{OutputFactory, OutputTrait};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use huber_common::model::config::Config;
use huber_common::result::Result;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::repo::RepoService;
use crate::service::ItemOperationTrait;

pub(crate) const CMD_NAME: &str = "list";

#[derive(Debug)]
pub(crate) struct RepoListCmd;

unsafe impl Send for RepoListCmd {}

unsafe impl Sync for RepoListCmd {}

impl RepoListCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'help> CommandTrait<'help> for RepoListCmd {
    fn app(&self) -> Command<'help> {
        Command::new(CMD_NAME)
            .visible_alias("ls")
            .about("List repositories")
    }
}

#[async_trait]
impl CommandAsyncTrait for RepoListCmd {
    async fn run(
        &self,
        config: &Config,
        container: &DIContainer,
        _matches: &ArgMatches,
    ) -> Result<()> {
        let repo_service = container.get::<RepoService>().unwrap();

        let repos = repo_service.list()?;
        output!(config.output_format, .display(
            stdout(),
            &repos,
            None,
            None,
        ))
    }
}
