use anyhow::anyhow;
use async_trait::async_trait;
use clap::{Args, ValueHint};
use log::info;
use semver::Version;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::{get_updated_package_version, CommandTrait};
use crate::error::HuberError::PackageNotInstalled;
use crate::lock_huber_ops;
use crate::model::config::Config;
use crate::parse::parse_pkg_name_optional_semver;
use crate::service::package::PackageService;
use crate::service::release::{ReleaseAsyncTrait, ReleaseService};
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

#[derive(Args)]
pub struct CurrentArgs {
    #[arg(
        help = "Package name with version (e.g. 'package-name@version')",
        num_args = 1,
        value_hint = ValueHint::Unknown,
        value_parser = parse_pkg_name_optional_semver,
        required = true,
    )]
    name_version: Vec<(String, String)>,
}

#[async_trait]
impl CommandTrait for CurrentArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        lock_huber_ops!(config);

        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        for (name, version) in self.name_version.iter() {
            if !pkg_service.has(name)? {
                return Err(anyhow!(PackageNotInstalled(name.clone())));
            }

            let pkg = pkg_service.get(name)?;
            let releases = release_service.find(&pkg).await?;

            if Version::parse(version.trim_start_matches('v')).is_err() {
                if let Some(mut r) = releases.clone().into_iter().find(|r| r.version.eq(version)) {
                    info!("Updating the current version of {} to {}", name, version);
                    release_service.set_current(&mut r).await?;
                    info!("{}@{} is now the current version", name, version);

                    return Ok(());
                } else {
                    anyhow::bail!("No installed version '{}' found for {}", version, name);
                }
            }

            let mut latest_release = release_service.get_latest(&pkg).await?;
            let version = get_updated_package_version(version, &latest_release.version);

            if let Some(mut r) = releases.into_iter().find(|r| r.version == version) {
                info!("Updating the current version of {} to {}", name, version);
                release_service.set_current(&mut r).await?;
                info!("{}@{} is now the current version", name, version);
            } else if !version.is_empty() {
                anyhow::bail!("No installed version {} found for {}", version, name);
            } else {
                info!(
                    "No version provided, trying to find the latest version of {}",
                    name
                );
                release_service.set_current(&mut latest_release).await?;
                info!(
                    "{}@{} is now the current version",
                    latest_release.package.name, latest_release.version
                );
            }
        }

        Ok(())
    }
}
