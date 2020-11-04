use std::io::stdout;

use clap::{App, Arg, ArgMatches};
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::output::factory::FactoryConsole;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::huber_common::output::OutputTrait;
use crate::service::package::PackageService;
use crate::service::release::{ReleaseService, ReleaseTrait};
use crate::service::ItemOperationTrait;

pub(crate) const CMD_NAME: &str = "current";

pub(crate) struct CurrentCmd;

impl CurrentCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for CurrentCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .about("Update current package version")
            .args(&[
                Arg::with_name("name")
                    .value_name("string")
                    .help("Package name")
                    .required(true)
                    .takes_value(true),
                Arg::with_name("version")
                    .value_name("string")
                    .long("version")
                    .short("v")
                    .help("Package version")
                    .takes_value(true),
            ])
    }

    fn run(&self, _runtime: &Runtime, config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let container = di_container();
        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        let name = matches.value_of("name").unwrap();

        if !matches.is_present("version") {
            let name = matches.value_of("name").unwrap();
            let package = pkg_service.get(name)?;
            let release = release_service.current(&package)?;

            return output!(
            config.output_format,
            .display(
                stdout(),
                &release,
                None,
                None,
            ));
        }

        let releases = release_service.find(&name.to_string())?;
        let version = matches.value_of("version").unwrap();

        return match releases.iter().find(|it| it.version == version) {
            Some(r) => release_service.set_current(r),

            None => Err(anyhow!("{} (version: {}) not found", name, version)),
        };
    }
}
