use async_trait::async_trait;
use clap::{App, Arg, ArgMatches};

use huber_common::di::DIContainer;
use huber_common::model::config::Config;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::release::ReleaseService;
use crate::service::ItemOperationTrait;
use huber_common::model::config::ConfigPath;

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
                    .value_name("package name")
                    .help("Package name")
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

        let name = matches.value_of("name").unwrap();
        let release_service = container.get::<ReleaseService>().unwrap();

        if !release_service.has(name)? {
            return Err(anyhow!("{} not found", name));
        }

        println!("Uninstalling {}", name);
        release_service.delete(name)?;
        println!("{} uninstalled", name);

        Ok(())
    }
}
