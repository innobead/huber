use std::cmp::Ordering;
use std::collections::HashMap;

use anyhow::anyhow;
use async_trait::async_trait;
use clap::Args;
use huber_common::model::config::Config;
use huber_common::model::release::Release;
use log::info;
use maplit::hashmap;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

#[derive(Args)]
pub struct UpdateArgs {
    #[arg(help = "Package name")]
    name: Vec<String>,

    #[arg(help = "Dry run to show available updates", long)]
    dryrun: bool,
}

#[async_trait]
impl CommandTrait for UpdateArgs {
    async fn run(&self, _: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        let mut installed_latest_pkg_releases: HashMap<String, Release> = hashmap! {};

        for ref name in self.name.iter() {
            info!("Checking for updates for {}", name);
            if !release_service.has(name)? {
                return Err(anyhow!(
                    "Unable to update {}, because it's not installed",
                    name
                ));
            }
        }

        for release in release_service.list()? {
            if let Some(existing_release) = installed_latest_pkg_releases.get(&release.name) {
                if release.compare(existing_release)? == Ordering::Greater {
                    installed_latest_pkg_releases.insert(release.name.clone(), release);
                }
            } else {
                installed_latest_pkg_releases.insert(release.name.clone(), release);
            }
        }

        for (name, installed_release) in installed_latest_pkg_releases.iter() {
            info!(
                "Checking for updates for {}. The latest installed version is {}",
                name, installed_release.version
            );

            let pkg = pkg_service.get(name)?;
            let new_release = release_service.get_latest(&pkg).await?;

            if new_release.compare(installed_release)? == Ordering::Greater {
                info!(
                    "Found an update for {}. Installed version: {}, Latest version: {}",
                    name, installed_release.version, new_release.version
                );
                update(
                    release_service,
                    self.dryrun,
                    &new_release,
                    installed_release,
                )
                .await?;
                info!(
                    "Successfully updated {}. Installed version: {}, Latest version: {}",
                    name, installed_release.version, new_release.version
                );
            }
        }

        Ok(())
    }
}

async fn update(
    release_service: &ReleaseService,
    dryrun: bool,
    new_release: &Release,
    installed_release: &Release,
) -> anyhow::Result<()> {
    if dryrun {
        info!("Available update {} to {}", installed_release, new_release);
    } else {
        info!("Updating {} to {}", installed_release, new_release);
        release_service.update(&new_release.package).await?;
    }

    Ok(())
}
