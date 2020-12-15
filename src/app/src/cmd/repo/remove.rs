use async_trait::async_trait;
use clap::{App, Arg, ArgMatches};

use huber_common::model::config::Config;
use huber_common::result::Result;
use huber_procmacro::process_lock;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::repo::RepoService;
use crate::service::ItemOperationTrait;
use huber_common::model::config::ConfigPath;

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

impl<'a, 'b> CommandTrait<'a, 'b> for RepoRemoveCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("rm")
            .about("Remove repositories")
            .args(&vec![Arg::with_name("name")
                .value_name("repo name")
                .help("Repository name")
                .takes_value(true)
                .required(true)])
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for RepoRemoveCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        matches: &ArgMatches<'a>,
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
