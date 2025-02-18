use std::io::stdout;
use std::path::PathBuf;

use anyhow::anyhow;
use async_trait::async_trait;
use clap::{Args, Subcommand, ValueHint};
use libcli_rs::output;
use libcli_rs::output::{OutputFactory, OutputTrait};
use log::{info, warn};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::error::HuberError::{RepoAlreadyExist, RepoNotFound};
use crate::lock_huber_ops;
use crate::model::config::Config;
use crate::model::repo::{Repository, LOCAL_REPO};
use crate::service::repo::RepoService;
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

#[derive(Args)]
pub struct RepoArgs {
    #[command(subcommand)]
    pub command: RepoCommands,
}

#[derive(Subcommand)]
pub enum RepoCommands {
    #[command(about = "Add a new repository", bin_name = "add")]
    Add(RepoAddArgs),

    #[command(about = "Remove a repository", bin_name = "remove")]
    Remove(RepoRemoveArgs),

    #[command(about = "Show all repositories", bin_name = "list")]
    Show(RepoShowArgs),
}

#[derive(Args)]
pub struct RepoAddArgs {
    #[arg(help = "Repo name", num_args = 1, value_hint = ValueHint::Unknown)]
    name: String,

    #[arg(
        help = "URL of the Huber package index file",
        long,
        num_args = 1,
        group = "repo",
        required_unless_present_any = &["file"],
        value_hint = ValueHint::Url
    )]
    url: Option<String>,

    #[arg(
        help = "File path of the Huber package index file",
        long,
        num_args = 1,
        group = "repo",
        required_unless_present_any = &["url"],
        value_hint = ValueHint::FilePath
    )]
    file: Option<String>,
}

#[async_trait]
impl CommandTrait for RepoAddArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        lock_huber_ops!(config);

        let repo_service = container.get::<RepoService>().unwrap();

        if repo_service.has(&self.name)? {
            return Err(anyhow!(RepoAlreadyExist(self.name.clone())));
        }

        let repo = Repository {
            name: self.name.clone(),
            url: self.url.clone(),
            file: self.file.clone().map(PathBuf::from),
        };
        info!("Adding repo {}", repo.name);
        if let Err(err) = repo_service.create(repo.clone()).await {
            return Err(anyhow!("Failed to add repo {}: {}", repo.name, err));
        };
        info!("Repo {} added", repo.name);

        Ok(())
    }
}

#[derive(Args)]
pub struct RepoRemoveArgs {
    #[arg(help = "Repo names", num_args = 1, value_hint = ValueHint::Unknown)]
    name: Vec<String>,
}

#[async_trait]
impl CommandTrait for RepoRemoveArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        lock_huber_ops!(config);

        let repo_service = container.get::<RepoService>().unwrap();

        for repo in &self.name {
            if repo == LOCAL_REPO {
                warn!("Cannot remove builtin local repo");
                continue;
            }

            if !repo_service.has(repo)? {
                return Err(anyhow!(RepoNotFound(repo.clone())));
            }

            info!("Removing repo {}", repo);
            repo_service.delete(repo)?;
            info!("Repo {} removed", repo);
        }

        Ok(())
    }
}

#[derive(Args)]
pub struct RepoShowArgs {}

#[async_trait]
impl CommandTrait for RepoShowArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let repo_service = container.get::<RepoService>().unwrap();

        let repos = repo_service.list()?;
        if repos.is_empty() {
            info!("No repositories added");
            return Ok(());
        }

        output!(
            config.output_format,
            .display(
                stdout(),
                &repos,
                None,
                None,
            )
        )
    }
}
