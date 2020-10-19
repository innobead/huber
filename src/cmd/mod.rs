use std::str::FromStr;

use clap::{App, ArgMatches};
use log::Level;

pub(crate) use info::InfoCmd;
pub(crate) use install::InstallCmd;
pub(crate) use list::ListCmd;
pub(crate) use root::RootCmd;
pub(crate) use search::SearchCmd;
pub(crate) use show::ShowCmd;
pub(crate) use uninstall::UninstallCmd;

use crate::base::config::Config;
use crate::base::di::{DIContainer, MutableRc};
use crate::base::result::Result;
use crate::cmd;
use crate::cmd::root::ARG_LOG_LEVEL;
use std::cell::RefMut;

mod info;
mod install;
mod list;
mod root;
mod search;
mod show;
mod uninstall;

pub(crate) trait CommandTrait<'a, 'b> {
    fn app(&self) -> App<'a, 'b>;
    fn run(&self, matches: &ArgMatches) -> Result<()>;
}

pub(crate) fn process_args(config: &mut Config, matches: &ArgMatches) {
    if matches.is_present(ARG_LOG_LEVEL) {
        if let Some(level) = matches.value_of(ARG_LOG_LEVEL) {
            if let Ok(level) = Level::from_str(level) {
                config.log_level = level;
            }
        }

        return;
    }
}

pub(crate) fn process_cmds(
    _config: &Config,
    matches: &ArgMatches,
    container_rc: MutableRc<DIContainer>,
) -> Result<()> {
    match matches.subcommand() {
        (cmd::install::CMD_NAME, Some(subcommand_matches)) => container_rc
            .borrow()
            .get::<InstallCmd>()
            .unwrap()
            .run(subcommand_matches),

        _ => Ok(()),
    }
}
