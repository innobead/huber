use clap::{App, Arg, ArgMatches};

use huber_common::config::Config;
use huber_common::di::di_container;
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
        App::new(CMD_NAME)
            .visible_alias("in")
            .about("Installs the package")
            .args(&vec![
                Arg::with_name("name")
                    .value_name("package name")
                    .help("Package name")
                    .required(true)
                    .takes_value(true),
                Arg::with_name("version")
                    .value_name("string")
                    .help("Package version")
                    .short("v")
                    .long("version")
                    .takes_value(true),
            ])
    }

    fn run(&self, _config: &Config, matches: &ArgMatches) -> Result<()> {
        let name = matches.value_of("name").unwrap();

        let container = di_container();
        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        if !pkg_service.has(name)? {
            return Err(anyhow!("{} not found", name));
        }

        let mut pkg = pkg_service.get(name)?;
        pkg.version = matches.value_of("version").map(|it| it.to_string());

        if release_service.has(name)? {
            let release = release_service.current(&pkg)?;

            return match &matches {
                _ if matches.is_present("version") => {
                    if release.version == matches.value_of("version").unwrap() {
                        Err(anyhow!("{} already installed", release))
                    } else {
                        println!("Updating {} to {}", pkg, release);

                        let release = release_service.update(&pkg)?;
                        println!("{} updated", release);
                        Ok(())
                    }
                }

                _ => {
                    Err(anyhow!("{} already installed. Use '--version' to install a specific version or 'update' command to update to the latest version", release))
                }
            };
        }

        println!("Installing {}", &pkg);
        let release = release_service.create(&pkg)?;
        println!("{} installed", release);

        Ok(())
    }
}
