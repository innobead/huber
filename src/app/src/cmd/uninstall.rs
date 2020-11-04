use clap::{App, Arg, ArgMatches};
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::service::release::ReleaseService;
use crate::service::ItemOperationTrait;

pub(crate) const CMD_NAME: &str = "uninstall";

pub(crate) struct UninstallCmd;

impl UninstallCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for UninstallCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Uninstall package").arg(
            Arg::with_name("name")
                .value_name("string")
                .help("Package name")
                .required(true)
                .takes_value(true),
        )
    }

    fn run(&self, _runtime: &Runtime, _config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let name = matches.value_of("name").unwrap();

        let container = di_container();
        let release_service = container.get::<ReleaseService>().unwrap();

        if !release_service.has(name)? {
            return Err(anyhow!("{} not found", name));
        }

        release_service.delete(name)?;
        println!("{} uninstalled!", name);

        Ok(())
    }
}
