use clap::{App, Arg, ArgMatches};

use crate::cmd::CommandTrait;
use anyhow::Result;
use huber_common::config::Config;
use tokio::runtime::Runtime;

pub(crate) const CMD_NAME: &str = "show";

pub(crate) struct ShowCmd;

impl ShowCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for ShowCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Show installed package").arg(
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
