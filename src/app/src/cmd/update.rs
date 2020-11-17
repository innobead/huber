use async_trait::async_trait;
use clap::{App, Arg, ArgMatches};

use huber_common::di::DIContainer;
use huber_common::model::config::Config;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use huber_common::model::config::ConfigPath;
use crate::service::package::PackageService;
use crate::service::release::{ReleaseService, ReleaseTrait};
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

pub(crate) const CMD_NAME: &str = "update";

#[derive(Debug)]
pub(crate) struct UpdateCmd;

unsafe impl Send for UpdateCmd {}

unsafe impl Sync for UpdateCmd {}

impl UpdateCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for UpdateCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("u")
            .about("Updates the installed package")
            .args(&vec![Arg::with_name("name")
                .value_name("package name")
                .help("Package name")
                .required(true)
                .takes_value(true)])
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for UpdateCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        matches: &ArgMatches<'a>,
    ) -> Result<()> {
        process_lock!();

        let name = matches.value_of("name").unwrap();
        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        if !release_service.has(name)? {
            return Err(anyhow!("{} not found", name));
        }

        let pkg = pkg_service.get(name)?;
        let release = release_service.current(&pkg)?;

        println!("Updating {} to the latest version", release);
        release_service.update(&pkg).await?;
        println!("{} updated", pkg);

        Ok(())
    }
}
