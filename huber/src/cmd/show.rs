use std::io::stdout;

use anyhow::anyhow;
use async_trait::async_trait;
use clap::Args;
use huber_common::model::config::Config;
use huber_common::model::release::SortModelTrait;
use libcli_rs::output;
use libcli_rs::output::{OutputFactory, OutputTrait};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::service::package::PackageService;
use crate::service::release::{ReleaseService, ReleaseTrait};
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

#[derive(Args)]
pub struct ShowArgs {
    #[arg(help = "Package name")]
    name: Option<String>,

    #[arg(help = "Show all the installed versions", long)]
    all: bool,

    #[arg(help = "Show the detailed artifact info", long, requires = "name")]
    detail: bool,
}

#[async_trait]
impl CommandTrait for ShowArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let pkg_service = container.get::<PackageService>().unwrap();
        let release_service = container.get::<ReleaseService>().unwrap();

        let mut exc_keys = vec![];
        if !self.detail {
            exc_keys = vec!["package"];
        }

        if let Some(name) = self.name.as_deref() {
            if !release_service.has(name)? {
                return Err(anyhow!("Failed to find package {}", name));
            }

            let pkg = pkg_service.get(name)?;
            let release = release_service.current(&pkg)?;

            if self.all {
                let mut releases = release_service.find(&pkg).await?;
                releases.sort_by_version();
                releases.iter_mut().for_each(|it| {
                    if it.current {
                        *it = release_service.current(&it.package).unwrap()
                    }
                });

                return output!(
                    config.output_format,
                    .display(
                        stdout(),
                        &releases,
                        None,
                        Some(exc_keys),
                    )
                );
            }

            return output!(config.output_format, .display(
                stdout(),
                &release,
                None,
                Some(exc_keys),
            ));
        }

        let mut cur_releases = release_service.list()?;
        cur_releases.sort_by_name();
        exc_keys.push("executables");

        let releases = if self.all {
            let mut releases = vec![];
            for rel in cur_releases.iter() {
                let mut rels = release_service.find(&rel.package).await?;
                rels.sort_by_version();
                releases.append(&mut rels);
            }

            releases
        } else {
            cur_releases
        };

        output!(config.output_format, .display(
            stdout(),
            &releases,
            None,
            Some(exc_keys),
        ))
    }
}
