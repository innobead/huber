use clap::{App, Arg, ArgMatches};

use crate::cmd::CommandTrait;
use crate::service::package::PackageService;
use crate::service::ItemSearchTrait;
use anyhow::Result;
use huber_common::config::Config;
use huber_common::di::container;
use huber_common::output::factory::FactoryConsole;
use huber_common::output::OutputTrait;
use std::io::stdout;
use tokio::runtime::Runtime;

pub(crate) const CMD_NAME: &str = "show";

pub(crate) struct ShowCmd;

impl ShowCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for ShowCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Show installed package").arg(
            Arg::with_name("name")
                .value_name("string")
                .help("Package name")
                .required(true)
                .takes_value(true),
        )
    }

    fn run(&self, _runtime: &Runtime, config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let container = container();
        let package_service = container.get::<PackageService>().unwrap();
        let result = package_service.info(matches.value_of("name").unwrap())?;

        output!(config.output_format, .display(
            stdout(),
            &result,
            None,
            Some(vec!["detail"]),
        ))
    }
}
