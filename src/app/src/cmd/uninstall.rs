use async_trait::async_trait;
use clap::{App, Arg, ArgMatches};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use huber_common::model::config::Config;
use huber_common::model::config::ConfigPath;
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

impl<'a, 'b> CommandTrait<'a, 'b> for UninstallCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_aliases(&["un", "rm"])
            .about("Uninstalls package")
            .arg(
                Arg::with_name("name")
                    .multiple(true)
                    .value_name("package name")
                    .help("Package name(s)")
                    .required(true)
                    .takes_value(true),
            )
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for UninstallCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        matches: &ArgMatches<'a>,
    ) -> Result<()> {
        process_lock!();

        let names: Vec<&str> = matches.values_of("name").unwrap().collect();

        for name in names {
            let release_service = container.get::<ReleaseService>().unwrap();

            if !release_service.has(name)? {
                return Err(anyhow!("{} not found", name));
            }

            println!("Uninstalling {}", name);
            release_service.delete(name)?;
            println!("{} uninstalled", name);
        }

        Ok(())
    }
}
