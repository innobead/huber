use std::io::stdout;

use async_trait::async_trait;
use clap::{Args, ValueHint};
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
    #[arg(
        help = "Package name or regex search with --pattern",
        num_args = 1,
        value_hint = ValueHint::Unknown
    )]
    name: Option<String>,

    #[arg(
        help = "Regex search",
        long,
        num_args = 0,
        value_hint = ValueHint::Unknown
    )]
    pattern: bool,

    #[arg(help = "Package owner", long,  num_args = 1, value_hint = ValueHint::Unknown)]
    owner: Option<String>,

    #[arg(
        help = "Show all the released versions",
        long,
        requires = "name",
        num_args = 0,
        value_hint = ValueHint::Unknown
    )]
    all: bool,

    #[arg(
        help = "Search in a specific repository",
        long,
        num_args = 1,
        value_hint = ValueHint::Unknown
    )]
    repo: Option<String>,
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
                self.repo.as_deref(),
            )?
            .into_iter()
            .map(PackageSummary::from)
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
