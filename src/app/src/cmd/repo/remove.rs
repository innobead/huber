use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use huber_common::model::config::Config;
use huber_common::model::config::ConfigPath;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::repo::RepoService;
use crate::service::ItemOperationTrait;

pub(crate) const CMD_NAME: &str = "remove";

#[derive(Debug)]
pub(crate) struct RepoRemoveCmd;

unsafe impl Send for RepoRemoveCmd {}

unsafe impl Sync for RepoRemoveCmd {}

impl RepoRemoveCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'help> CommandTrait<'help> for RepoRemoveCmd {
    fn app(&self) -> Command<'help> {
        Command::new(CMD_NAME)
            .visible_alias("rm")
            .about("Remove repositories")
            .args([Arg::new("name")
                .value_name("repo name")
                .help("Repository name")
                .takes_value(true)
                .required(true)])
    }
}

#[async_trait]
impl CommandAsyncTrait for RepoRemoveCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        matches: &ArgMatches,
    ) -> Result<()> {
        process_lock!();

        let name = matches.value_of("name").unwrap();
        let repo_service = container.get::<RepoService>().unwrap();

        if !repo_service.has(name)? {
            return Err(anyhow!("{} not found", name));
        }

        repo_service.delete(name)?;
        println!("{} removed", name);

        Ok(())
    }
}
