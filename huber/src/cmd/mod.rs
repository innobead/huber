use async_trait::async_trait;
use clap::Subcommand;
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
mod lock;
mod option;
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
    #[command(about = "Manage Huber configuration")]
    Config(ConfigArgs),

    #[command(about = "Update the current package version")]
    Current(CurrentArgs),

    #[command(about = "Flush inactive artifacts")]
    Flush(FlushArgs),

    #[command(about = "Show package info")]
    Info(InfoArgs),

    #[command(about = "Install packages")]
    Install(InstallArgs),

    #[command(about = "Manage Huber repositories")]
    Repo(RepoArgs),

    #[command(about = "Reset Huber")]
    Reset(ResetArgs),

    #[command(about = "Search package")]
    Search(SearchArgs),

    #[command(about = "Update huber")]
    SelfUpdate(SelfUpdateArgs),

    #[command(about = "Show installed packages")]
    Show(ShowArgs),

    #[command(about = "Uninstall packages")]
    Uninstall(UninstallArgs),

    #[command(about = "Updates the installed packages")]
    Update(UpdateArgs),

    #[command(about = "Save installed packages to a file")]
    Save(SaveArgs),

    #[command(about = "Load installed packages from a file")]
    Load(LoadArgs),

    #[command(about = "Lock the package version")]
    Lock(LockArgs),

    #[command(about = "Unlock the package version")]
    Unlock(UnlockArgs),
}
