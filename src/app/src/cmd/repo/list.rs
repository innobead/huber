use std::io::stdout;

use async_trait::async_trait;
use clap::{App, ArgMatches};

use huber_common::config::Config;
use huber_common::di::DIContainer;
use huber_common::output::factory::FactoryConsole;
use huber_common::output::OutputTrait;
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

impl<'a, 'b> CommandTrait<'a, 'b> for RepoListCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("ls")
            .about("List repositories")
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for RepoListCmd {
    async fn run(
        &self,
        config: &Config,
        container: &DIContainer,
        _matches: &ArgMatches<'a>,
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
