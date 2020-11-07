use clap::{App, ArgMatches};

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::model::release::VecExtensionTrait;
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
        App::new(CMD_NAME)
            .about("Flushes inactive artifacts")
            .long_about("Flushing inactive artifacts includes removing non-current packages.")
    }

    fn run(&self, _config: &Config, _matches: &ArgMatches<'a>) -> Result<()> {
        let container = di_container();
        let release_service = container.get::<ReleaseService>().unwrap();

        let current_releases = release_service.list()?;
        for cr in current_releases.iter() {
            let mut releases = release_service.find(&cr.package)?;

            if releases.len() == 1 {
                println!(
                    "Bypassed {}, no inactive releases to remove",
                    cr.package.name
                );
                continue;
            }

            releases.sort_by_version();

            for r in releases {
                if !r.current {
                    println!("Removing {}", &r);
                    release_service.delete_release(&r)?;
                }
            }
        }

        Ok(())
    }
}
