use clap::{App, Arg, ArgMatches};

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::service::package::PackageService;
use crate::service::release::{ReleaseService, ReleaseTrait};
use crate::service::ItemOperationTrait;

pub(crate) const CMD_NAME: &str = "update";

pub(crate) struct UpdateCmd;

impl UpdateCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for UpdateCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .about("Updates the installed package")
            .args(&vec![Arg::with_name("name")
                .value_name("package name")
                .help("Package name")
                .required(true)
                .takes_value(true)])
    }

    fn run(&self, _config: &Config, matches: &ArgMatches) -> Result<()> {
        let name = matches.value_of("name").unwrap();

        let container = di_container();
        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        if !release_service.has(name)? {
            return Err(anyhow!("{} not found", name));
        }

        let pkg = pkg_service.get(name)?;
        let release = release_service.current(&pkg)?;

        println!("Updating {} to the latest version", release);
        release_service.update(&pkg)?;
        println!("{} updated", pkg);

        Ok(())
    }
}
