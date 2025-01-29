use std::fmt::{Display, Formatter};
use std::str::FromStr;

use async_trait::async_trait;
use clap::builder::PossibleValue;
use clap::{Subcommand, ValueEnum};
use clap_complete::Shell;
use config::ConfigArgs;
use current::CurrentArgs;
use flush::FlushArgs;
use huber_common::model::config::Config;
use info::InfoArgs;
use install::InstallArgs;
use repo::RepoArgs;
use reset::ResetArgs;
use search::SearchArgs;
use self_update::SelfUpdateArgs;
use show::ShowArgs;
use simpledi_rs::di::DIContainer;
use uninstall::UninstallArgs;
use update::UpdateArgs;

use crate::cmd::load::LoadArgs;
use crate::cmd::lock::LockArgs;
use crate::cmd::save::SaveArgs;
use crate::cmd::unlock::UnlockArgs;

pub mod config;
mod current;
mod flush;
mod info;
mod install;
mod load;
pub mod lock;
pub mod repo;
mod reset;
mod save;
mod search;
mod self_update;
mod show;
mod uninstall;
mod unlock;
mod update;

#[async_trait]
pub trait CommandTrait {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()>;
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Manage Huber configuration", bin_name = "config")]
    Config(ConfigArgs),

    #[command(about = "Update the current package version", bin_name = "current")]
    Current(CurrentArgs),

    #[command(about = "Show completions", bin_name = "completions")]
    Completions {
        #[arg(help = "Shell name", num_args = 1, value_enum)]
        shell: Shell,
    },

    #[command(about = "Flush inactive artifacts", bin_name = "flush")]
    Flush(FlushArgs),

    #[command(about = "Show package info", bin_name = "info")]
    Info(InfoArgs),

    #[command(about = "Install packages", bin_name = "install")]
    Install(InstallArgs),

    #[command(about = "Manage Huber repositories", bin_name = "repo")]
    Repo(RepoArgs),

    #[command(about = "Reset Huber", bin_name = "reset")]
    Reset(ResetArgs),

    #[command(about = "Search package", bin_name = "search")]
    Search(SearchArgs),

    #[command(about = "Update huber", bin_name = "self-update")]
    SelfUpdate(SelfUpdateArgs),

    #[command(about = "Show installed packages", bin_name = "show")]
    Show(ShowArgs),

    #[command(about = "Uninstall packages", bin_name = "uninstall")]
    Uninstall(UninstallArgs),

    #[command(about = "Updates the installed packages", bin_name = "update")]
    Update(UpdateArgs),

    #[command(about = "Save installed packages to a file", bin_name = "save")]
    Save(SaveArgs),

    #[command(about = "Load installed packages from a file", bin_name = "load")]
    Load(LoadArgs),

    #[command(about = "Lock packages or Show locked packages", bin_name = "lock")]
    Lock(LockArgs),

    #[command(about = "Unlock packages", bin_name = "unlock", bin_name = "unlock")]
    Unlock(UnlockArgs),
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlatformStdLib {
    Gnu,
    Musl,
    Msvc,
}

impl Display for PlatformStdLib {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl FromStr for PlatformStdLib {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}

impl ValueEnum for PlatformStdLib {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            PlatformStdLib::Gnu,
            PlatformStdLib::Musl,
            PlatformStdLib::Msvc,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            PlatformStdLib::Gnu => PossibleValue::new("gnu"),
            PlatformStdLib::Musl => PossibleValue::new("musl"),
            PlatformStdLib::Msvc => PossibleValue::new("msvc"),
        })
    }
}

#[macro_export]
macro_rules! lock_huber_ops {
    ($config:ident) => {
        use huber_common::model::config::ConfigPath;
        use huber_procmacro::process_lock;

        #[cfg(not(target_os = "windows"))]
        {
            let lock_file = $config.lock_file()?;
            process_lock!(lock_file);
        }
    };
}
