use std::io::stdout;

use anyhow::Result;
use clap::{App, Arg, ArgMatches};
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::container;
use huber_common::output::factory::FactoryConsole;
use huber_common::output::OutputTrait;

use crate::cmd::CommandTrait;
use crate::service::{ItemOperationTrait, ItemSearchTrait};
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;

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
                .required(false)
                .takes_value(true),
        )
    }

    fn run(&self, _runtime: &Runtime, config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let container = container();
        let release_service = container.get::<ReleaseService>().unwrap();

        if matches.is_present("name") {
            let name = matches.value_of("name").unwrap();
            if !release_service.has(name)? {
                return Err(anyhow!("{} not found", name));
            }

            let release = release_service.get(name)?;

            return output!(config.output_format, .display(
                stdout(),
                &release,
                None,
                None,
            ));
        }

        let releases = release_service.list()?;

        output!(config.output_format, .display(
            stdout(),
            &releases,
            None,
            None,
        ))
    }
}
