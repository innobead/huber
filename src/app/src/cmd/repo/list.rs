use std::io::stdout;

use clap::{App, ArgMatches};

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::output::factory::FactoryConsole;
use huber_common::output::OutputTrait;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::service::repo::RepoService;
use crate::service::ItemOperationTrait;

pub(crate) const CMD_NAME: &str = "list";

pub(crate) struct RepoListCmd;

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

    fn run(&self, config: &Config, _matches: &ArgMatches<'a>) -> Result<()> {
        let container = di_container();
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
