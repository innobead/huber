

use clap::{App, ArgMatches};

use huber_common::config::Config;
use huber_common::di::di_container;

use huber_common::result::Result;

use crate::cmd::CommandTrait;

use crate::service::cache::{CacheService, CacheTrait};

pub(crate) const CMD_NAME: &str = "refresh";

pub(crate) struct RefreshCmd;

impl RefreshCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for RefreshCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("rf")
            .about("Refresh the repositories, managed and unmanaged packages")
    }

    fn run(&self, _config: &Config, _matches: &ArgMatches<'a>) -> Result<()> {
        let container = di_container();
        let cache_service = container.get::<CacheService>().unwrap();

        let _ = cache_service.update_repositories()?;
        println!("{}", "Repositories updated");

        Ok(())
    }
}

