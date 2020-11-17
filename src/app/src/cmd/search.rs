use std::io::stdout;

use async_trait::async_trait;
use clap::{App, Arg, ArgMatches};

use huber_common::di::DIContainer;
use huber_common::model::config::Config;
use huber_common::model::package::PackageSummary;
use huber_common::model::release::VecExtensionTrait;
use huber_common::output::factory::FactoryConsole;
use huber_common::output::OutputTrait;
use huber_common::result::Result;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::cache::{CacheAsyncTrait, CacheService};
use crate::service::package::PackageService;
use crate::service::{ItemOperationAsyncTrait, ItemSearchTrait};

pub(crate) const CMD_NAME: &str = "search";

#[derive(Debug)]
pub(crate) struct SearchCmd;
unsafe impl Send for SearchCmd {}
unsafe impl Sync for SearchCmd {}

impl SearchCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for SearchCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("se")
            .about("Searches package")
            .args(&[
                Arg::with_name("name")
                    .value_name("package name")
                    .help("Package name")
                    .takes_value(true),
                Arg::with_name("owner")
                    .value_name("string")
                    .short("r")
                    .long("owner")
                    .help("Package owner")
                    .takes_value(true),
                Arg::with_name("pattern")
                    .value_name("string")
                    .short("p")
                    .long("pattern")
                    .help("Regex pattern")
                    .takes_value(true),
                Arg::with_name("all")
                    .short("a")
                    .long("all")
                    .help("Show all the released versions of package given '--name' specified"),
            ])
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for SearchCmd {
    async fn run(
        &self,
        config: &Config,
        container: &DIContainer,
        matches: &ArgMatches<'a>,
    ) -> Result<()> {
        let pkg_service = container.get::<PackageService>().unwrap();
        let cache_service = container.get::<CacheService>().unwrap();

        let _ = cache_service.update_repositories().await?;

        if matches.is_present("name") && matches.is_present("all") {
            let mut pkgs: Vec<PackageSummary> = pkg_service
                .find(&matches.value_of("name").unwrap().to_string())
                .await?
                .into_iter()
                .map(|it| PackageSummary::from(it))
                .collect();
            pkgs.sort_by_version();

            return output!(config.output_format, .display(
                stdout(),
                &pkgs,
                None,
                Some(vec!["name", "description", "source"]),
            ));
        }

        let pkgs: Vec<PackageSummary> = pkg_service
            .search(
                matches.value_of("name"),
                matches.value_of("pattern"),
                matches.value_of("owner"),
            )?
            .into_iter()
            .map(|it| PackageSummary::from(it))
            .collect();

        output!(config.output_format, .display(
            stdout(),
            &pkgs,
            None,
            Some(vec!["version", "kind"]),
        ))
    }
}
