use std::io::stdout;

use clap::{App, Arg, ArgMatches};
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::di_container;

use huber_common::output::OutputTrait;

use huber_common::output::factory::FactoryConsole;

use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::service::package::PackageService;
use crate::service::ItemSearchTrait;

pub(crate) const CMD_NAME: &str = "search";

pub(crate) struct SearchCmd;

impl SearchCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for SearchCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Search package").args(&[
            Arg::with_name("name")
                .value_name("string")
                .short("n")
                .long("name")
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
        ])
    }

    fn run(&self, _runtime: &Runtime, config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let container = di_container();
        let pkg_service = container.get::<PackageService>().unwrap();

        let results = pkg_service.search(
            matches.value_of("name"),
            matches.value_of("pattern"),
            matches.value_of("owner"),
        )?;

        output!(config.output_format, .display(
            stdout(),
            &results,
            Some(vec!["name", "source"]),
            None,
        ))
    }
}
