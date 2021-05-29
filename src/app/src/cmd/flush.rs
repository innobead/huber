use async_trait::async_trait;
use clap::{App, ArgMatches};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use huber_common::log::println_many;
use huber_common::model::config::Config;
use huber_common::model::config::ConfigPath;
use huber_common::model::release::{Release, SortModelTrait};
use huber_common::progress::progress;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::release::{ReleaseService, ReleaseTrait};
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

pub(crate) const CMD_NAME: &str = "flush";

#[derive(Debug)]
pub(crate) struct FlushCmd;

unsafe impl Send for FlushCmd {}

unsafe impl Sync for FlushCmd {}

impl FlushCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for FlushCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("f")
            .about("Flushes inactive artifacts")
            .long_about("Flushing inactive artifacts includes removing non-current packages.")
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for FlushCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        _matches: &ArgMatches<'a>,
    ) -> Result<()> {
        process_lock!();

        let release_service = container.get::<ReleaseService>().unwrap();

        let current_releases = release_service.list()?;
        let mut flushed_releases: Vec<Release> = vec![];
        for cr in current_releases.iter() {
            let mut releases = release_service.find(&cr.package).await?;

            if releases.len() == 1 {
                continue;
            }

            releases.sort_by_version();

            for r in releases {
                if !r.current {
                    progress(&format!("Removing {}", &r))?;

                    release_service.delete_release(&r)?;
                    flushed_releases.push(r);
                }
            }
        }

        if flushed_releases.len() == 0 {
            println!("No non-current releases to flush");
        } else {
            println_many(
                "Flushed releases",
                &flushed_releases
                    .iter()
                    .map(|it| it.to_string())
                    .collect::<Vec<_>>(),
            );
        }

        Ok(())
    }
}
