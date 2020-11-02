use clap::{App, Arg, ArgMatches};

use crate::cmd::CommandTrait;
use huber_common::config::Config;
use huber_common::di::{DIContainer, DIObjectTrait, MutableRc};
use huber_common::result::Result;
use tokio::runtime::Runtime;

pub(crate) const CMD_NAME: &str = "install";

pub(crate) struct InstallCmd {
    container: MutableRc<DIContainer>,
}

impl DIObjectTrait for InstallCmd {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self {
        Self { container }
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for InstallCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Install package").arg(
            Arg::with_name("name")
                .help("Package name")
                .required(true)
                .takes_value(true),
        )
    }

    fn run(&self, runtime: &Runtime, config: &Config, matches: &ArgMatches) -> Result<()> {
        let name = matches.value_of("name").unwrap();

        // check name [--version=<v>]---> huber-packages/name/huber.yaml exists

        // check name (--url https://github.com/prj/repo) [--version=--tag] ----> <url>/huber.yaml exists

        // download huber.yaml, then download package, then install to ~/.huber/bin/<name>/<version>/<....>
        // symbolic links to ~/.huber/bin/...

        // save metadata (name, installed path, ) to database

        unimplemented!()
    }
}
