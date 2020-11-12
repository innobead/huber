use clap::{App, Arg, ArgMatches};

use huber_common::config::Config;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::service::repo::RepoService;
use crate::service::ItemOperationTrait;
use huber_common::di::di_container;
use huber_common::model::repo::Repository;

pub(crate) const CMD_NAME: &str = "add";

pub(crate) struct RepoAddCmd;

impl RepoAddCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for RepoAddCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("a")
            .about("Add repositories")
            .args(&vec![
                Arg::with_name("name")
                    .value_name("repo name")
                    .help("Repository name")
                    .takes_value(true)
                    .required(true),
                Arg::with_name("url")
                    .value_name("repo url")
                    .help("Github repo URL")
                    .takes_value(true)
                    .required(true),
            ])
    }

    fn run(&self, _config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let name = matches.value_of("name").unwrap();
        let url = matches.value_of("url").unwrap();

        let container = di_container();
        let repo_service = container.get::<RepoService>().unwrap();

        if repo_service.has(name)? {
            return Err(anyhow!("{} already exists", name));
        }

        let repo = Repository {
            name: name.to_string(),
            url: url.to_string(),
        };
        let repo = repo_service.create(repo)?;

        println!("{} added", repo);

        Ok(())
    }
}
