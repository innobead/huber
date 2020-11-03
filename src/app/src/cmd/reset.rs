use clap::{App, ArgMatches};
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::result::Result;

use crate::cmd::CommandTrait;

pub(crate) const CMD_NAME: &str = "reset";

pub(crate) struct ResetCmd;

impl ResetCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for ResetCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Reset huber (ex: installed packages)")
    }

    fn run(&self, _runtime: &Runtime, _config: &Config, _matches: &ArgMatches<'a>) -> Result<()> {
        unimplemented!()
    }
}
