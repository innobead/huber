use async_trait::async_trait;
use clap::{App, Arg, ArgMatches};
use simpledi_rs::di::DIContainer;
use simpledi_rs::di::DIContainerTrait;

use huber_common::model::config::Config;
use huber_common::model::config::ConfigPath;
use huber_common::progress::progress;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::package::PackageService;
use crate::service::release::{ReleaseAsyncTrait, ReleaseService};
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};
use huber_common::log::println_many;

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
                    .value_name("package version")
                    .help("Package version")
                    .required(true)
                    .takes_value(true),
            ])
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for CurrentCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        matches: &ArgMatches<'a>,
    ) -> Result<()> {
        process_lock!();

        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        let name = matches.value_of("name").unwrap();
        if !pkg_service.has(name)? {
            return Err(anyhow!("{} not installed"));
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
