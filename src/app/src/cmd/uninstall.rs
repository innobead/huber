use clap::{App, Arg, ArgMatches};

use crate::cmd::CommandTrait;
use huber_common::config::Config;
use huber_common::result::Result;
use tokio::runtime::Runtime;

pub(crate) const CMD_NAME: &str = "uninstall";

pub(crate) struct UninstallCmd;

impl UninstallCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for UninstallCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Uninstall package").arg(
            Arg::with_name("name")
                .value_name("string")
                .help("Package name")
                .required(true)
                .takes_value(true),
        )
    }

    fn run(&self, _runtime: &Runtime, _config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let _name = matches.value_of("name").unwrap();

        unimplemented!()
    }
}
