use std::io::stdout;

use anyhow::anyhow;
use async_trait::async_trait;
use clap::Args;
use huber_common::model::config::Config;
use libcli_rs::output;
use libcli_rs::output::{OutputFactory, OutputTrait};
use log::info;
use serde::{Deserialize, Serialize};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::option::parser::parse_name_version;
use crate::cmd::CommandTrait;
use crate::service::config::{ConfigService, ConfigTrait};
use crate::service::package::PackageService;
use crate::service::ItemOperationTrait;

#[derive(Args)]
pub struct LockArgs {
    #[arg(
        help = "Package name (e.g. 'package-name' or 'package-name@version')",
        value_parser = parse_name_version
    )]
    name_version: Vec<(String, String)>,
}

#[async_trait]
impl CommandTrait for LockArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        if self.name_version.is_empty() {
            return display_pkg_configs(config);
        }

        let pkg_service = container.get::<PackageService>().unwrap();
        let mut config = config.clone();
        let mut require_update = false;

        for (pkg, version) in &self.name_version {
            if !pkg_service.has(&pkg)? {
                return Err(anyhow!("{} package not found", pkg));
            }

            info!("Locking package: {}@{}", pkg, version);
            if let Some(versions) = config.lock_pkg_versions.get_mut(pkg) {
                if !versions.contains(version) {
                    versions.push(version.clone());
                } else {
                    info!("Package {}@{} already locked", pkg, version);
                }
            } else {
                config
                    .lock_pkg_versions
                    .insert(pkg.clone(), vec![version.clone()]);
            }

            if !require_update {
                require_update = true;
            }
        }

        if !require_update {
            info!("No packages to lock");
            return Ok(());
        }

        let config_service = container.get::<ConfigService>().unwrap();
        config_service.update(&config)?;
        info!("Packages locked successfully");

        Ok(())
    }
}

fn display_pkg_configs(config: &Config) -> anyhow::Result<()> {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct PkgVersionInfo {
        name: String,
        versions: Vec<String>,
    }

    let pkg_version_infos: Vec<_> = config
        .lock_pkg_versions
        .iter()
        .map(|(name, versions)| PkgVersionInfo {
            name: name.clone(),
            versions: versions.clone(),
        })
        .collect();

    output!(
        config.output_format,
        .display(stdout(), &pkg_version_infos, None, None)
    )
}
