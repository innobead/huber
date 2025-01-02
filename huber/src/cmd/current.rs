use anyhow::anyhow;
use async_trait::async_trait;
use clap::Args;
use huber_common::model::config::Config;
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::option::parser::parse_name_version;
use crate::cmd::CommandTrait;
use crate::service::package::PackageService;
use crate::service::release::{ReleaseAsyncTrait, ReleaseService};
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

#[derive(Args)]
pub struct CurrentArgs {
    #[arg(
        help = "Package name with version (e.g. 'package-name@version')",
        value_parser = parse_name_version
    )]
    name_version: Vec<(String, String)>,
}

#[async_trait]
impl CommandTrait for CurrentArgs {
    async fn run(&self, _: &Config, container: &DIContainer) -> anyhow::Result<()> {
        if self.name_version.is_empty() {
            return Err(anyhow!("No package name with version provided"));
        }

        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        for (name, version) in self.name_version.iter() {
            if !pkg_service.has(name)? {
                return Err(anyhow!("{} not installed", name));
            }

            let pkg = pkg_service.get(name)?;
            let releases = release_service.find(&pkg).await?;

            if let Some(mut r) = releases
                .into_iter()
                .find(|it| it.version == version.to_string())
            {
                info!("Updating the current version of {} to {}", name, version);
                release_service.set_current(&mut r).await?;
                info!("{}@{} is now the current version", name, version);
            } else {
                info!(
                    "No version provided, trying to find the latest version of {}",
                    name
                );
                let mut release = release_service.get_latest(&pkg).await?;
                release_service.set_current(&mut release).await?;
                info!(
                    "{}@{} is now the current version",
                    release.package.name, release.version
                );
            }
        }

        Ok(())
    }
}
