use std::io::stdout;

use anyhow::anyhow;
use async_trait::async_trait;
use clap::{Args, ValueHint};
use huber_common::model::config::Config;
use huber_common::model::release::SortModelTrait;
use libcli_rs::output;
use libcli_rs::output::{OutputFactory, OutputTrait};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::error::HuberError::{NoPackagesInstalled, PackageNotFound};
use crate::service::package::PackageService;
use crate::service::release::{ReleaseService, ReleaseTrait};
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

#[derive(Args)]
pub struct ShowArgs {
    #[arg(help = "Package name", num_args = 1, value_hint = ValueHint::Unknown)]
    name: Option<String>,

    #[arg(
        help = "Show all the installed versions",
        long,
        value_hint = ValueHint::Unknown
    )]
    all: bool,

    #[arg(
        help = "Show the detailed artifact info",
        long,
        requires = "name",
        value_hint = ValueHint::Unknown
    )]
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
            return self
                .show_package(name, &exc_keys, config, pkg_service, release_service)
                .await;
        }

        let mut current_releases = release_service.list()?;
        current_releases.sort_by_name();
        exc_keys.push("executables");

        let releases = if self.all {
            let mut releases = vec![];
            for rel in current_releases.iter() {
                let mut rels = release_service.find(&rel.package).await?;
                rels.sort_by_version();
                releases.append(&mut rels);
            }

            releases
        } else {
            current_releases
        };

        if releases.is_empty() {
            return Err(anyhow!(NoPackagesInstalled));
        }

        output!(config.output_format, .display(
            stdout(),
            &releases,
            None,
            Some(exc_keys),
        ))
    }
}

impl ShowArgs {
    async fn show_package(
        &self,
        name: &str,
        exc_keys: &[&str],
        config: &Config,
        pkg_service: &PackageService,
        release_service: &ReleaseService,
    ) -> anyhow::Result<()> {
        if !release_service.has(name)? {
            return Err(anyhow!(PackageNotFound(name.to_string())));
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
                    Some(exc_keys.into()),
                )
            );
        }

        output!(
            config.output_format,
            .display(
                stdout(),
                &release,
                None,
                Some(exc_keys.into()),
            )
        )
    }
}
