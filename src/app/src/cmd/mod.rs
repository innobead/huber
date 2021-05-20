use std::env;
use std::str::FromStr;

use async_trait::async_trait;
use clap::{App, ArgMatches};
use log::Level;

use config::{ARG_GITHUB_KEY, ARG_GITHUB_TOKEN, ARG_LOG_LEVEL, ARG_OUTPUT_TYPE};
use current::CurrentCmd;
use flush::FlushCmd;
use huber_common::model::config::Config;
use huber_common::result::Result;
use info::InfoCmd;
use install::InstallCmd;
use repo::RepoCmd;
use reset::ResetCmd;
use search::SearchCmd;
use self_update::SelfUpdateCmd;
use show::ShowCmd;
use simpledi_rs::di::DIContainer;
use simpledi_rs::di::DIContainerTrait;
use uninstall::UninstallCmd;
use update::UpdateCmd;

use crate::cmd;
use crate::cmd::config::ConfigCmd;
use libcli_rs::output::OutputFormat;

pub(crate) mod config;
pub(crate) mod current;
pub(crate) mod flush;
pub(crate) mod info;
pub(crate) mod install;
pub(crate) mod repo;
pub(crate) mod reset;
pub(crate) mod root;
pub(crate) mod search;
pub(crate) mod self_update;
pub(crate) mod show;
pub(crate) mod uninstall;
pub(crate) mod update;

pub(crate) trait CommandTrait<'a, 'b>: CommandAsyncTrait<'a, 'b> {
    fn app(&self) -> App<'a, 'b>;
}

#[async_trait]
pub(crate) trait CommandAsyncTrait<'a, 'b> {
    async fn run(
        &self,
        config: &Config,
        container: &DIContainer,
        matches: &ArgMatches<'a>,
    ) -> Result<()>;
}

pub(crate) fn prepare_arg_matches<'a, 'b>(app: App<'a, 'b>) -> ArgMatches<'a> {
    let mut args = env::args();
    match args {
        _ if args.len() == 1 => {
            app.get_matches_from(vec![args.nth(0).unwrap(), "help".to_string()])
        }
        _ if args.len() == 2 => {
            let arg1 = args.nth(0).unwrap();
            let arg2 = args.nth(0).unwrap();

            if ["repo", "config"].contains(&arg2.as_str()) {
                app.get_matches_from(vec![arg1, arg2, "help".to_string()])
            } else {
                app.get_matches()
            }
        }

        _ => app.get_matches(),
    }
}

pub(crate) fn process_arg_matches(config: &mut Config, matches: &ArgMatches) -> bool {
    let mut updated = false;

    if let Some(arg) = matches.value_of(ARG_LOG_LEVEL) {
        if let Ok(result) = Level::from_str(&arg.to_lowercase()) {
            config.log_level = result.to_string();
            updated = true;
        }
    }

    if let Some(arg) = matches.value_of(ARG_OUTPUT_TYPE) {
        if let Ok(result) = OutputFormat::from_str(arg) {
            config.output_format = result;
            updated = true;
        }
    }

    if let Some(arg) = matches.value_of(ARG_GITHUB_TOKEN) {
        config.github_token = Some(arg.to_string());
        updated = true;
    }

    if let Some(arg) = matches.value_of(ARG_GITHUB_KEY) {
        config.github_key = Some(arg.to_string());
        updated = true;
    }

    updated
}

pub(crate) async fn process_cmds(
    // runtime: &Runtime,
    config: &Config,
    container: &DIContainer,
    matches: &ArgMatches<'_>,
) -> Result<()> {
    match matches.subcommand() {
        (cmd::current::CMD_NAME, Some(sub_matches)) => {
            container
                .get::<CurrentCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        (cmd::flush::CMD_NAME, Some(sub_matches)) => {
            container
                .get::<FlushCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        (cmd::info::CMD_NAME, Some(sub_matches)) => {
            container
                .get::<InfoCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        (cmd::install::CMD_NAME, Some(sub_matches)) => {
            container
                .get::<InstallCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        (cmd::reset::CMD_NAME, Some(sub_matches)) => {
            container
                .get::<ResetCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        (cmd::search::CMD_NAME, Some(sub_matches)) => {
            container
                .get::<SearchCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        (cmd::self_update::CMD_NAME, Some(sub_matches)) => {
            container
                .get::<SelfUpdateCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        (cmd::show::CMD_NAME, Some(sub_matches)) => {
            container
                .get::<ShowCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        (cmd::uninstall::CMD_NAME, Some(sub_matches)) => {
            container
                .get::<UninstallCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        (cmd::update::CMD_NAME, Some(sub_matches)) => {
            container
                .get::<UpdateCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        (cmd::repo::CMD_NAME, Some(sub_matches)) => {
            container
                .get::<RepoCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        (cmd::config::CMD_NAME, Some(sub_matches)) => {
            container
                .get::<ConfigCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        _ => unimplemented!("command not implemented"),
    }
}
