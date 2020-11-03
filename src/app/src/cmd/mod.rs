use std::str::FromStr;

use clap::{App, ArgMatches};
use log::Level;
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::{container, DIContainer, MutableArc};
use huber_common::output::OutputFormat;
use huber_common::result::Result;

use crate::cmd;
use crate::cmd::info::InfoCmd;
use crate::cmd::install::InstallCmd;
use crate::cmd::list::ListCmd;
use crate::cmd::root::{ARG_LOG_LEVEL, ARG_OUTPUT_TYPE};
use crate::cmd::search::SearchCmd;
use crate::cmd::show::ShowCmd;
use crate::cmd::uninstall::UninstallCmd;

pub(crate) mod info;
pub(crate) mod install;
pub(crate) mod list;
pub(crate) mod root;
pub(crate) mod search;
pub(crate) mod show;
pub(crate) mod uninstall;
pub(crate) mod current;
pub(crate) mod self_update;
pub(crate) mod reset;

pub(crate) trait CommandTrait<'a, 'b> {
    fn app(&self) -> App<'a, 'b>;
    fn run(&self, runtime: &Runtime, config: &Config, matches: &ArgMatches<'a>) -> Result<()>;
}

pub(crate) fn process_args(config: &mut Config, matches: &ArgMatches) {
    if let Some(level) = matches.value_of(ARG_LOG_LEVEL) {
        if let Ok(level) = Level::from_str(&level.to_lowercase()) {
            config.log_level = level;
        }
    }

    if let Some(output) = matches.value_of(ARG_OUTPUT_TYPE) {
        if let Ok(level) = OutputFormat::from_str(output) {
            config.output_format = level;
        }
    }
}

pub(crate) fn process_cmds(
    runtime: &Runtime,
    config: &Config,
    matches: &ArgMatches,
    _container_rc: MutableArc<DIContainer>,
) -> Result<()> {
    match matches.subcommand() {
        (cmd::install::CMD_NAME, Some(sub_matches)) => container()
            .get::<InstallCmd>()
            .unwrap()
            .run(runtime, config, sub_matches),

        (cmd::uninstall::CMD_NAME, Some(sub_matches)) => container()
            .get::<UninstallCmd>()
            .unwrap()
            .run(runtime, config, sub_matches),

        (cmd::search::CMD_NAME, Some(sub_matches)) => {
            container()
                .get::<SearchCmd>()
                .unwrap()
                .run(runtime, config, sub_matches)
        }

        (cmd::list::CMD_NAME, Some(sub_matches)) => {
            container()
                .get::<ListCmd>()
                .unwrap()
                .run(runtime, config, sub_matches)
        }

        (cmd::info::CMD_NAME, Some(sub_matches)) => {
            container()
                .get::<InfoCmd>()
                .unwrap()
                .run(runtime, config, sub_matches)
        }

        (cmd::show::CMD_NAME, Some(sub_matches)) => {
            container()
                .get::<ShowCmd>()
                .unwrap()
                .run(runtime, config, sub_matches)
        }

        _ => unimplemented!("command not implemented"),
    }
}