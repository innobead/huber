use std::io::stdout;

use async_trait::async_trait;
use clap::Args;
use huber_common::model::config::Config;
use huber_common::model::package::PackageSummary;
use libcli_rs::output;
use libcli_rs::output::{OutputFactory, OutputTrait};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::service::cache::{CacheAsyncTrait, CacheService};
use crate::service::package::PackageService;
use crate::service::ItemSearchTrait;

#[derive(Args)]
pub struct SearchArgs {
    #[arg(help = "Package name or regex search with --pattern")]
    name: Option<String>,

    #[arg(help = "Regex search", long)]
    pattern: bool,

    #[arg(help = "Package owner", long)]
    owner: Option<String>,

    #[arg(help = "Show all the released versions", long, requires = "name")]
    all: bool,
}

#[async_trait]
impl CommandTrait for SearchArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let pkg_service = container.get::<PackageService>().unwrap();
        let cache_service = container.get::<CacheService>().unwrap();

        let _ = cache_service.update_repositories().await?;

        if self.all {
            let pkgs = pkg_service
                .find_summary(&self.name.clone().unwrap(), false)
                .await?;

            return output!(
                config.output_format,
                .display(
                    stdout(),
                    &pkgs,
                    None,
                    Some(vec!["name", "description", "source"]),
                )
            );
        }

        let pkgs: Vec<PackageSummary> = pkg_service
            .search(
                self.name.as_deref(),
                if self.pattern {
                    self.name.as_deref()
                } else {
                    None
                },
                self.owner.as_deref(),
            )?
            .into_iter()
            .map(|it| PackageSummary::from(it))
            .collect();

        output!(
            config.output_format,
            .display(
                stdout(),
                &pkgs,
                None,
                Some(vec!["version", "kind"]),
            )
        )
    }
}
