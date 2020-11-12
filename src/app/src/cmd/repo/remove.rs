use clap::{App, Arg, ArgMatches};

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::service::repo::RepoService;
use crate::service::ItemOperationTrait;

pub(crate) const CMD_NAME: &str = "remove";

pub(crate) struct RepoRemoveCmd;

impl RepoRemoveCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for RepoRemoveCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("rm")
            .about("Remove repositories")
            .args(&vec![
                Arg::with_name("name")
                    .value_name("repo name")
                    .help("Repository name")
                    .takes_value(true)
                    .required(true),
            ])
    }

    fn run(&self, _config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let name = matches.value_of("name").unwrap();

        let container = di_container();
        let repo_service = container.get::<RepoService>().unwrap();

        if !repo_service.has(name)? {
            return Err(anyhow!("{} not found", name))
        }

        repo_service.delete(name)?;
        println!("{} removed", name);

        Ok(())
    }
}
