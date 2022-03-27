use std::path::PathBuf;

use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use huber_common::model::config::Config;
use huber_common::model::config::ConfigPath;
use huber_common::model::repo::Repository;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::repo::RepoService;
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

pub(crate) const CMD_NAME: &str = "add";

#[derive(Debug)]
pub(crate) struct RepoAddCmd;

unsafe impl Send for RepoAddCmd {}

unsafe impl Sync for RepoAddCmd {}

impl RepoAddCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'help> CommandTrait<'help> for RepoAddCmd {
    fn app(&self) -> Command<'help> {
        Command::new(CMD_NAME)
            .visible_alias("a")
            .about("Add repositories")
            .args([
                Arg::new("name")
                    .value_name("repo name")
                    .help("Repository name")
                    .takes_value(true)
                    .required(true),
                Arg::new("url")
                    .short('u')
                    .long("url")
                    .value_name("repo url")
                    .help("Github repo URL")
                    .takes_value(true)
                    .required(true)
                    .conflicts_with("file"),
                Arg::new("file")
                    .short('f')
                    .long("file")
                    .value_name("repo config file")
                    .help("Local repo config file path")
                    .takes_value(true)
                    .required(true)
                    .conflicts_with("url"),
            ])
    }
}

#[async_trait]
impl CommandAsyncTrait for RepoAddCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        matches: &ArgMatches,
    ) -> Result<()> {
        process_lock!();

        let name = matches.value_of("name").unwrap();
        let url = matches.value_of("url").map(|it| it.to_string());
        let file = matches.value_of("file").map(|it| PathBuf::from(it));

        let repo_service = container.get::<RepoService>().unwrap();

        if repo_service.has(name)? {
            return Err(anyhow!("{} already exists", name));
        }

        let repo = Repository {
            name: name.to_string(),
            url,
            file,
        };
        let repo = repo_service.create(repo).await?;

        println!("{} added", repo);

        Ok(())
    }
}
