use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use huber_common::model::config::Config;
use huber_common::model::config::ConfigPath;
use huber_common::progress::progress;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::release::ReleaseService;
use crate::service::ItemOperationTrait;

pub(crate) const CMD_NAME: &str = "uninstall";

#[derive(Debug)]
pub(crate) struct UninstallCmd;

unsafe impl Send for UninstallCmd {}

unsafe impl Sync for UninstallCmd {}

impl UninstallCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'help> CommandTrait<'help> for UninstallCmd {
    fn app(&self) -> Command<'help> {
        Command::new(CMD_NAME)
            .visible_aliases(&["un", "rm"])
            .about("Uninstalls package")
            .arg(
                Arg::new("name")
                    .multiple_occurrences(true)
                    .value_name("package name")
                    .help("Package name(s)")
                    .required(true)
                    .takes_value(true),
            )
    }
}

#[async_trait]
impl CommandAsyncTrait for UninstallCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        matches: &ArgMatches,
    ) -> Result<()> {
        process_lock!();

        let names: Vec<&str> = matches.values_of("name").unwrap().collect();

        for name in names {
            let release_service = container.get::<ReleaseService>().unwrap();

            if !release_service.has(name)? {
                return Err(anyhow!("{} not found", name));
            }

            progress(&format!("Uninstalling {}", name))?;
            release_service.delete(name)?;
            println!("{} uninstalled", name);
        }

        Ok(())
    }
}
