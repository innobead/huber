use std::str::FromStr;

use clap::{App, ArgMatches};
use hubcaps::Credentials;
use log::Level;

use huber_common::config::Config;
use huber_common::di::{di_container, DIContainer, MutableArc};
use huber_common::output::OutputFormat;
use huber_common::result::Result;

use crate::cmd;
use crate::cmd::current::CurrentCmd;
use crate::cmd::flush::FlushCmd;
use crate::cmd::info::InfoCmd;
use crate::cmd::install::InstallCmd;
use crate::cmd::reset::ResetCmd;
use crate::cmd::root::{ARG_GITHUB_TOKEN, ARG_LOG_LEVEL, ARG_OUTPUT_TYPE};
use crate::cmd::search::SearchCmd;
use crate::cmd::self_update::SelfUpdateCmd;
use crate::cmd::show::ShowCmd;
use crate::cmd::uninstall::UninstallCmd;

pub(crate) mod current;
pub(crate) mod flush;
pub(crate) mod info;
pub(crate) mod install;
pub(crate) mod reset;
pub(crate) mod root;
pub(crate) mod search;
pub(crate) mod self_update;
pub(crate) mod show;
pub(crate) mod uninstall;

pub(crate) trait CommandTrait<'a, 'b> {
    fn app(&self) -> App<'a, 'b>;
    fn run(&self, config: &Config, matches: &ArgMatches<'a>) -> Result<()>;
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

    if let Some(output) = matches.value_of(ARG_GITHUB_TOKEN) {
        config.github_credentials = Some(Credentials::Token(output.to_string()))
    }
}

pub(crate) fn process_cmds(
    // runtime: &Runtime,
    config: &Config,
    matches: &ArgMatches,
    _container_rc: MutableArc<DIContainer>,
) -> Result<()> {
    match matches.subcommand() {
        (cmd::current::CMD_NAME, Some(sub_matches)) => di_container()
            .get::<CurrentCmd>()
            .unwrap()
            .run(config, sub_matches),

        (cmd::flush::CMD_NAME, Some(sub_matches)) => di_container()
            .get::<FlushCmd>()
            .unwrap()
            .run(config, sub_matches),

        (cmd::info::CMD_NAME, Some(sub_matches)) => di_container()
            .get::<InfoCmd>()
            .unwrap()
            .run(config, sub_matches),

        (cmd::install::CMD_NAME, Some(sub_matches)) => di_container()
            .get::<InstallCmd>()
            .unwrap()
            .run(config, sub_matches),

        (cmd::reset::CMD_NAME, Some(sub_matches)) => di_container()
            .get::<ResetCmd>()
            .unwrap()
            .run(config, sub_matches),

        (cmd::search::CMD_NAME, Some(sub_matches)) => di_container()
            .get::<SearchCmd>()
            .unwrap()
            .run(config, sub_matches),

        (cmd::self_update::CMD_NAME, Some(sub_matches)) => di_container()
            .get::<SelfUpdateCmd>()
            .unwrap()
            .run(config, sub_matches),

        (cmd::show::CMD_NAME, Some(sub_matches)) => di_container()
            .get::<ShowCmd>()
            .unwrap()
            .run(config, sub_matches),

        (cmd::uninstall::CMD_NAME, Some(sub_matches)) => di_container()
            .get::<UninstallCmd>()
            .unwrap()
            .run(config, sub_matches),

        _ => unimplemented!("command not implemented"),
    }
}
