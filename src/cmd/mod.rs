use std::str::FromStr;

use clap::{App, ArgMatches};
use log::Level;

pub(crate) use install::InstallCmd;
pub(crate) use uninstall::UninstallCmd;
pub(crate) use search::SearchCmd;
pub(crate) use list::ListCmd;
pub(crate) use info::InfoCmd;
pub(crate) use show::ShowCmd;
pub(crate) use root::RootCmd;

use crate::base::Config;
use crate::cmd;
use crate::cmd::root::ARG_LOG_LEVEL;

mod install;
mod root;
mod uninstall;
mod search;
mod list;
mod info;
mod show;

pub(crate) trait Command<'a, 'b> {
    fn app() -> App<'a, 'b>;
    fn run(matches: &ArgMatches) -> anyhow::Result<()>;
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

pub(crate) fn process_cmds(_config: &Config, matches: &ArgMatches) -> anyhow::Result<()> {
    match matches.subcommand() {
        (cmd::install::CMD_NAME, Some(subcommand_matches)) => {
            InstallCmd::run(subcommand_matches)
        }

        _ => {
            Ok(())
        }
    }
}
