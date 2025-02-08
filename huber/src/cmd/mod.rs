use std::fmt::{Display, Formatter};
use std::str::FromStr;

use async_trait::async_trait;
use clap::builder::PossibleValue;
use clap::{Subcommand, ValueEnum};
use clap_complete::Shell;
use config::ConfigArgs;
use current::CurrentArgs;
use flush::FlushArgs;
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
use crate::model::config::Config;

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
    #[command(about = "Manage Huber configurations", bin_name = "config")]
    Config(ConfigArgs),

    #[command(about = "Update the current package versions", bin_name = "current")]
    Current(CurrentArgs),

    #[command(
        about = "Show command completions for the specified shell",
        bin_name = "completions"
    )]
    Completions {
        #[arg(help = "Shell name", num_args = 1, value_enum)]
        shell: Shell,
    },

    #[command(about = "Remove outdated installed artifacts", bin_name = "flush")]
    Flush(FlushArgs),

    #[command(about = "Show package information", bin_name = "info")]
    Info(InfoArgs),

    #[command(about = "Install packages", bin_name = "install")]
    Install(InstallArgs),

    #[command(about = "Manage repositories", bin_name = "repo")]
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

    #[command(about = "Save the installed package list to a file", bin_name = "save")]
    Save(SaveArgs),

    #[command(
        about = "Load installed packages from a file generated by save command",
        bin_name = "load"
    )]
    Load(LoadArgs),

    #[command(about = "Lock packages or Show locked packages", bin_name = "lock")]
    Lock(LockArgs),

    #[command(about = "Unlock packages", bin_name = "unlock", bin_name = "unlock")]
    Unlock(UnlockArgs),
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlatformStdLib {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    Gnu,
    #[cfg(target_os = "linux")]
    Musl,
    #[cfg(target_os = "windows")]
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
    #[cfg(target_os = "linux")]
    fn value_variants<'a>() -> &'a [Self] {
        &[PlatformStdLib::Gnu, PlatformStdLib::Musl]
    }

    #[cfg(target_os = "macos")]
    fn value_variants<'a>() -> &'a [Self] {
        &[]
    }

    #[cfg(target_os = "windows")]
    fn value_variants<'a>() -> &'a [Self] {
        &[PlatformStdLib::Gnu, PlatformStdLib::Msvc]
    }

    #[cfg(target_os = "linux")]
    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(if let PlatformStdLib::Gnu = self {
            PossibleValue::new("gnu")
        } else {
            PossibleValue::new("musl")
        })
    }

    #[cfg(target_os = "windows")]
    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(if let PlatformStdLib::Gnu = self {
            PossibleValue::new("gnu")
        } else {
            PossibleValue::new("msvc")
        })
    }

    #[cfg(target_os = "macos")]
    fn to_possible_value(&self) -> Option<PossibleValue> {
        None
    }
}

#[macro_export]
macro_rules! lock_huber_ops {
    ($config:ident) => {
        use huber_procmacro::process_lock;
        use $crate::model::config::ConfigPath;

        #[cfg(not(target_os = "windows"))]
        {
            let lock_file = $config.lock_file()?;
            process_lock!(lock_file);
        }
    };
}

pub fn get_updated_package_version(version: &str, latest_version: &str) -> String {
    if latest_version.starts_with("v") && !version.starts_with("v") {
        format!("v{}", version)
    } else {
        version.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_updated_package_version() {
        let version = "1.0.0";
        let latest_version = "v1.0.1";
        assert_eq!(
            get_updated_package_version(version, latest_version),
            "v1.0.0"
        );

        let version = "v1.0.0";
        let latest_version = "1.0.1";
        assert_eq!(
            get_updated_package_version(version, latest_version),
            "v1.0.0"
        );

        let version = "v1.0.0";
        let latest_version = "v1.0.1";
        assert_eq!(
            get_updated_package_version(version, latest_version),
            "v1.0.0"
        );

        let version = "1.0.0";
        let latest_version = "1.0.1";
        assert_eq!(
            get_updated_package_version(version, latest_version),
            "1.0.0"
        );
    }
}
