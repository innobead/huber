use clap::{App, ArgMatches};
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::container;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::service::update::UpdateService;
use crate::service::update::UpdateTrait;

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
        let container = container();
        let update_service = container.get::<UpdateService>().unwrap();

        if update_service.has_update()? {
            return update_service.update();
        }

        println!("{}", "No update available");
        Ok(())
    }
}
