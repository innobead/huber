use clap::{App, Arg, ArgMatches};

use crate::cmd::CommandTrait;
use huber_common::config::Config;
use huber_common::result::Result;
use tokio::runtime::Runtime;

pub(crate) const CMD_NAME: &str = "install";

pub(crate) struct InstallCmd;

impl InstallCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for InstallCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Install package").arg(
            Arg::with_name("name")
                .value_name("string")
                .help("Package name")
                .required(true)
                .takes_value(true),
        )
    }

    fn run(&self, _runtime: &Runtime, _config: &Config, matches: &ArgMatches) -> Result<()> {
        let _name = matches.value_of("name").unwrap();

        // check name [--version=<v>]---> huber-packages/name/huber.yaml exists

        // check name (--url https://github.com/prj/repo) [--version=--tag] ----> <url>/huber.yaml exists

        // download huber.yaml, then download package, then install to ~/.huber/bin/<name>/<version>/<....>
        // symbolic links to ~/.huber/bin/...

        // save metadata (name, installed path, ) to database

        unimplemented!()
    }
}
