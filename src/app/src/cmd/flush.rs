use clap::{App, ArgMatches};
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::container;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::service::release::{ReleaseService, ReleaseTrait};
use crate::service::ItemOperationTrait;

pub(crate) const CMD_NAME: &str = "flush";

pub(crate) struct FlushCmd;

impl FlushCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for FlushCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Flush inactive artifacts (ex: remove non-current packages)")
    }

    fn run(&self, _runtime: &Runtime, _config: &Config, _matches: &ArgMatches<'a>) -> Result<()> {
        let container = container();
        let release_service = container.get::<ReleaseService>().unwrap();

        let releases = release_service.list()?;
        for r in releases {
            if !r.is_current {
                release_service.delete_release(&r)?;
            }
        }

        Ok(())
    }
}
