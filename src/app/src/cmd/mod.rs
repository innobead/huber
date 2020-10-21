use std::str::FromStr;

use clap::{App, ArgMatches};
use log::Level;

use crate::cmd;
use crate::cmd::install::InstallCmd;
use crate::cmd::root::ARG_LOG_LEVEL;
use huber_common::config::Config;
use huber_common::di::{DIContainer, MutableRc};
use huber_common::result::Result;

pub(crate) mod info;
pub(crate) mod install;
pub(crate) mod list;
pub(crate) mod root;
pub(crate) mod search;
pub(crate) mod show;
pub(crate) mod uninstall;

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
