use async_trait::async_trait;
use clap::Args;
use huber_common::model::config::Config;
use huber_common::model::release::{Release, SortModelTrait};
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::service::release::{ReleaseService, ReleaseTrait};
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

#[derive(Args)]
pub struct FlushArgs {}

#[async_trait]
impl CommandTrait for FlushArgs {
    async fn run(&self, _: &Config, container: &DIContainer) -> anyhow::Result<()> {
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
                    info!("Removing {}", r);
                    release_service.delete_release(&r)?;
                    info!("{} removed", r);

                    flushed_releases.push(r);
                }
            }
        }

        if flushed_releases.len() == 0 {
            info!("Nothing to flush");
        }

        Ok(())
    }
}
