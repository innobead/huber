use std::fs::File;
use std::io::{BufRead, BufReader};

use async_trait::async_trait;
use clap::Args;
use huber_common::model::config::Config;
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::install::{install_packages, parse_package_name_versions};
use crate::cmd::CommandTrait;
use crate::service::cache::{CacheAsyncTrait, CacheService};
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;

#[derive(Args)]
pub struct LoadArgs {
    #[arg(help = "Load a package list to install", long)]
    file: String,
}

#[async_trait]
impl CommandTrait for LoadArgs {
    async fn run(&self, _: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        let cache_service = container.get::<CacheService>().unwrap();
        cache_service.update_repositories().await?;

        info!("Loading packages from {}", self.file);
        let file = File::open(&self.file)?;
        let reader = BufReader::new(file);
        let versions: Vec<_> = reader.lines().filter_map(Result::ok).collect();

        info!("Installing packages: total {}", versions.len());
        let versions: Vec<_> = parse_package_name_versions(&versions);
        install_packages(release_service, pkg_service, versions).await
    }
}
