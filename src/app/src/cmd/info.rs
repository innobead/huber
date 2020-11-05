use std::io::stdout;

use clap::{App, Arg, ArgMatches};

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::output::factory::FactoryConsole;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::huber_common::output::OutputTrait;
use crate::service::package::PackageService;
use crate::service::ItemOperationTrait;

pub(crate) const CMD_NAME: &str = "info";

pub(crate) struct InfoCmd;

impl InfoCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for InfoCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Show package info").arg(
            Arg::with_name("name")
                .value_name("string")
                .help("Package name")
                .required(true)
                .takes_value(true),
        )
    }

    fn run(&self, config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let container = di_container();
        let pkg_service = container.get::<PackageService>().unwrap();
        let result = pkg_service.get(matches.value_of("name").unwrap())?;

        output!(config.output_format, .display(
            stdout(),
            &result,
            None,
            Some(vec!["detail"]),
        ))
    }
}
