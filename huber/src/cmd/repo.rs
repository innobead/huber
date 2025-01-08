use std::io::stdout;

use anyhow::anyhow;
use async_trait::async_trait;
use clap::{Args, Subcommand, ValueHint};
use huber_common::model::config::Config;
use huber_common::model::repo::Repository;
use libcli_rs::output;
use libcli_rs::output::{OutputFactory, OutputTrait};
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::error::HuberError::{RepoAlreadyExist, RepoNotFound};
use crate::service::repo::RepoService;
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

#[derive(Args)]
pub struct RepoArgs {
    #[command(subcommand)]
    pub command: RepoCommands,
}

#[derive(Subcommand)]
pub enum RepoCommands {
    #[command(about = "Add a new repo", bin_name = "add")]
    Add(RepoAddArgs),

    #[command(about = "Remove a repo", bin_name = "remove")]
    Remove(RepoRemoveArgs),

    #[command(about = "List all repos", bin_name = "list")]
    List(RepoListArgs),
}

#[derive(Args)]
pub struct RepoAddArgs {
    #[arg(help = "Repo name", num_args = 1, value_hint = ValueHint::Unknown)]
    name: String,

    #[arg(help = "GitHub repo URL", long, num_args = 1, value_hint = ValueHint::Url)]
    url: String,

    #[arg(
        help = "Local file path of the repo config",
        long,
        num_args = 1,
        value_hint = ValueHint::FilePath
    )]
    file: String,
}

#[async_trait]
impl CommandTrait for RepoAddArgs {
    async fn run(&self, _: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let repo_service = container.get::<RepoService>().unwrap();

        if repo_service.has(&self.name)? {
            return Err(anyhow!(RepoAlreadyExist(self.name.clone())));
        }

        let repo = Repository {
            name: self.name.clone(),
            url: Some(self.url.clone()),
            file: Some(self.file.clone().into()),
        };
        info!("Adding repo {}", repo);
        repo_service.create(repo).await?;
        info!("Repo added");

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
    async fn run(&self, _: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let repo_service = container.get::<RepoService>().unwrap();

        for repo in &self.name {
            if !repo_service.has(repo)? {
                return Err(anyhow!(RepoNotFound(repo.clone())));
            }

            info!("Removing repo {}", repo);
            repo_service.delete(repo)?;
        }

        Ok(())
    }
}

#[derive(Args)]
pub struct RepoListArgs {}

#[async_trait]
impl CommandTrait for RepoListArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let repo_service = container.get::<RepoService>().unwrap();

        let repos = repo_service.list()?;
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
