use std::str::FromStr;

use async_trait::async_trait;
use clap::{ArgMatches, Command};
use libcli_rs::output::OutputFormat;
use log::Level;
use simpledi_rs::di::DIContainer;
use simpledi_rs::di::DIContainerTrait;

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
use uninstall::UninstallCmd;
use update::UpdateCmd;

use crate::cmd;
use crate::cmd::config::ConfigCmd;

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

pub(crate) trait CommandTrait<'help>: CommandAsyncTrait {
    fn app(&self) -> Command<'help>;
}

#[async_trait]
pub(crate) trait CommandAsyncTrait {
    async fn run(
        &self,
        config: &Config,
        container: &DIContainer,
        matches: &ArgMatches,
    ) -> Result<()>;
}

pub(crate) fn update_config_by_arg_matches(config: &mut Config, matches: &ArgMatches) -> bool {
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

pub(crate) async fn process_cmds(container: &DIContainer, matches: &ArgMatches) -> Result<()> {
    let config = container.get::<Config>().unwrap();

    match matches.subcommand() {
        Some((cmd::current::CMD_NAME, sub_matches)) => {
            container
                .get::<CurrentCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        Some((cmd::flush::CMD_NAME, sub_matches)) => {
            container
                .get::<FlushCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        Some((cmd::info::CMD_NAME, sub_matches)) => {
            container
                .get::<InfoCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        Some((cmd::install::CMD_NAME, sub_matches)) => {
            container
                .get::<InstallCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        Some((cmd::reset::CMD_NAME, sub_matches)) => {
            container
                .get::<ResetCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        Some((cmd::search::CMD_NAME, sub_matches)) => {
            container
                .get::<SearchCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        Some((cmd::self_update::CMD_NAME, sub_matches)) => {
            container
                .get::<SelfUpdateCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        Some((cmd::show::CMD_NAME, sub_matches)) => {
            container
                .get::<ShowCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        Some((cmd::uninstall::CMD_NAME, sub_matches)) => {
            container
                .get::<UninstallCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        Some((cmd::update::CMD_NAME, sub_matches)) => {
            container
                .get::<UpdateCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        Some((cmd::repo::CMD_NAME, sub_matches)) => {
            container
                .get::<RepoCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        Some((cmd::config::CMD_NAME, sub_matches)) => {
            container
                .get::<ConfigCmd>()
                .unwrap()
                .run(config, container, sub_matches)
                .await
        }

        _ => Ok(())
    }
}
