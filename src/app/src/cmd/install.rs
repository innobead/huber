use clap::{App, Arg, ArgMatches};
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::container;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::service::package::PackageService;
use crate::service::release::{ReleaseService, ReleaseTrait};
use crate::service::ItemOperationTrait;

pub(crate) const CMD_NAME: &str = "install";

pub(crate) struct InstallCmd;

impl InstallCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for InstallCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Install package").args(&vec![
            Arg::with_name("name")
                .value_name("string")
                .help("Package name")
                .required(true)
                .takes_value(true),
            Arg::with_name("version")
                .value_name("string")
                .help("Package version")
                .short("v")
                .long("version")
                .takes_value(true),
            Arg::with_name("refresh")
                .help("Refresh package with the latest version")
                .short("r")
                .long("refresh"),
        ])
    }

    fn run(&self, _runtime: &Runtime, _config: &Config, matches: &ArgMatches) -> Result<()> {
        let name = matches.value_of("name").unwrap();

        let container = container();
        let release_service = container.get::<ReleaseService>().unwrap();
        let package_service = container.get::<PackageService>().unwrap();

        if !package_service.has(name)? {
            return Err(anyhow!("{} not found", name));
        }

        let pkg = package_service.get(name)?;

        if release_service.has(name)? {
            if matches.is_present("refresh") {
                let release = release_service.update(&pkg)?;
                println!("{} updated!", release);

                return Ok(());
            }

            let release = release_service.current(&pkg)?;
            println!(
                "{} already installed. Use '--fresh' to update to the latest version",
                release
            );

            return Ok(());
        }

        let release = release_service.create(&pkg)?;
        println!("{} installed!", release);

        Ok(())
    }
}
