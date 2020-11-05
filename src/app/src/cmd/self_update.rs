use clap::{App, ArgMatches};

use huber_common::config::Config;
use huber_common::di::di_container;
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

    fn run(&self, _config: &Config, _matches: &ArgMatches<'a>) -> Result<()> {
        let container = di_container();
        let update_service = container.get::<UpdateService>().unwrap();

        if update_service.has_update()? {
            update_service.update()?;
            return Ok(());
        }

        println!("{}", "No update available");
        Ok(())
    }
}
