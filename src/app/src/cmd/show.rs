use std::io::stdout;

use anyhow::Result;
use clap::{App, Arg, ArgMatches};

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::output::factory::FactoryConsole;
use huber_common::output::OutputTrait;

use crate::cmd::CommandTrait;
use crate::service::package::PackageService;
use crate::service::release::{ReleaseService, ReleaseTrait};
use crate::service::ItemOperationTrait;
use huber_common::model::release::VecExtensionTrait;

pub(crate) const CMD_NAME: &str = "show";

pub(crate) struct ShowCmd;

impl ShowCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for ShowCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .about("Show installed packages")
            .args(&vec![
                Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .value_name("string")
                    .help("Package name")
                    .takes_value(true),
                Arg::with_name("all")
                    .short("a")
                    .long("all")
                    .help("Show all installed versions of package given '--name' specified)"),
            ])
    }

    fn run(&self, config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let container = di_container();
        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        if matches.is_present("name") {
            let name = matches.value_of("name").unwrap();

            if !release_service.has(name)? {
                return Err(anyhow!("{} not found", name));
            }

            let pkg = pkg_service.get(name)?;
            let release = release_service.current(&pkg)?;

            if matches.is_present("all") {
                let mut releases = release_service.find(&pkg)?;
                releases.sort_by_version();

                return output!(config.output_format, .display(
                    stdout(),
                    &releases,
                    None,
                    Some(vec!["package"]),
                ));
            }

            return output!(config.output_format, .display(
                stdout(),
                &release,
                None,
                None,
            ));
        }

        let mut releases = release_service.list()?;
        releases.sort_by_version();

        output!(config.output_format, .display(
            stdout(),
            &releases,
            None,
            Some(vec!["package"]),
        ))
    }
}
