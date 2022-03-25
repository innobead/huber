use std::io::stdout;

use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use libcli_rs::output::{OutputFactory, OutputTrait};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use huber_common::model::config::Config;
use huber_common::result::Result;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::ItemOperationTrait;

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

impl<'help> CommandTrait<'help> for InfoCmd {
    fn app(&self) -> Command<'help> {
        Command::new(CMD_NAME)
            .visible_alias("i")
            .about("Shows the package info")
            .arg(
                Arg::new("name")
                    .value_name("package name")
                    .help("Package name")
                    .required(true)
                    .takes_value(true),
            )
    }
}

#[async_trait]
impl CommandAsyncTrait for InfoCmd {
    async fn run(
        &self,
        config: &Config,
        container: &DIContainer,
        matches: &ArgMatches,
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
