use clap::{App, ArgMatches};
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::result::Result;

use crate::cmd::CommandTrait;

pub(crate) const CMD_NAME: &str = "self-update";

pub(crate) struct SelfUpdateCmd;

impl SelfUpdateCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for SelfUpdateCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Update huber")
    }

    fn run(&self, _runtime: &Runtime, _config: &Config, _matches: &ArgMatches<'a>) -> Result<()> {
        unimplemented!()
    }
}
