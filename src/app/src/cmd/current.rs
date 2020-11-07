use clap::{App, Arg, ArgMatches};

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
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
            .visible_alias("c")
            .about("Updates the current package version")
            .args(&[
                Arg::with_name("name")
                    .value_name("package name")
                    .help("Package name")
                    .required(true)
                    .takes_value(true),
                Arg::with_name("version")
                    .value_name("string")
                    .long("version")
                    .short("v")
                    .help("Package version")
                    .required(true)
                    .takes_value(true),
            ])
    }

    fn run(&self, _config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let container = di_container();
        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        let name = matches.value_of("name").unwrap();
        if !pkg_service.has(name)? {
            return Err(anyhow!("{} not installed"));
        }

        let pkg = pkg_service.get(name)?;
        let releases = release_service.find(&pkg)?;
        let version = matches.value_of("version").unwrap();

        match releases.into_iter().find(|it| it.version == version) {
            Some(mut r) => {
                println!("Setting {} as the current package", &r);
                release_service.set_current(&mut r)?;
                println!("{} as current updated", &r);

                Ok(())
            }

            None => Err(anyhow!("{} not found", version)),
        }
    }
}
