use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use simpledi_rs::di::DIContainer;
use simpledi_rs::di::DIContainerTrait;

use huber_common::log::println_many;
use huber_common::model::config::Config;
use huber_common::model::config::ConfigPath;
use huber_common::progress::progress;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::package::PackageService;
use crate::service::release::{ReleaseAsyncTrait, ReleaseService};
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

pub(crate) const CMD_NAME: &str = "current";

#[derive(Debug)]
pub(crate) struct CurrentCmd;

unsafe impl Send for CurrentCmd {}

unsafe impl Sync for CurrentCmd {}

impl CurrentCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'help> CommandTrait<'help> for CurrentCmd {
    fn app(&self) -> Command<'help> {
        Command::new(CMD_NAME)
            .visible_alias("c")
            .about("Updates the current package version")
            .args(&[
                Arg::new("name")
                    .value_name("package name")
                    .help("Package name")
                    .required(true)
                    .takes_value(true),
                Arg::new("version")
                    .value_name("package version")
                    .help("Package version")
                    .required(true)
                    .takes_value(true),
            ])
    }
}

#[async_trait]
impl CommandAsyncTrait for CurrentCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        matches: &ArgMatches,
    ) -> Result<()> {
        process_lock!();

        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        let name = matches.value_of("name").unwrap();
        if !pkg_service.has(name)? {
            return Err(anyhow!("{} not installed", name));
        }

        let pkg = pkg_service.get(name)?;
        let releases = release_service.find(&pkg).await?;
        let version = matches.value_of("version").unwrap();

        if let Some(mut r) = releases.into_iter().find(|it| it.version == version) {
            progress(&format!("Setting {} as the current package", &r))?;
            let executables = release_service.set_current(&mut r).await?;
            println_many("Updated executables", &executables);
            println!("{} as current updated", &r);

            Ok(())
        } else {
            Err(anyhow!("{} not found", version))
        }
    }
}
