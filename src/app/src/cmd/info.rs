use std::io::stdout;

use async_trait::async_trait;
use clap::{App, Arg, ArgMatches};

use huber_common::model::config::Config;
use huber_common::output::factory::FactoryConsole;
use huber_common::result::Result;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::ItemOperationTrait;
use huber_common::output::OutputTrait;

pub(crate) const CMD_NAME: &str = "info";

#[derive(Debug)]
pub(crate) struct InfoCmd;

unsafe impl Send for InfoCmd {}
unsafe impl Sync for InfoCmd {}

impl InfoCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for InfoCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("i")
            .about("Shows the package info")
            .arg(
                Arg::with_name("name")
                    .value_name("package name")
                    .help("Package name")
                    .required(true)
                    .takes_value(true),
            )
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for InfoCmd {
    async fn run(
        &self,
        config: &Config,
        container: &DIContainer,
        matches: &ArgMatches<'a>,
    ) -> Result<()> {
        let pkg_service = container.get::<PackageService>().unwrap();
        let release_service = container.get::<ReleaseService>().unwrap();

        let pkg = pkg_service.get(matches.value_of("name").unwrap())?;
        let release = release_service.get_latest(&pkg).await?;

        output!(config.output_format, .display(
            stdout(),
            &release.package,
            None,
            Some(vec!["detail"]),
        ))
    }
}
