use std::io::stdout;

use async_trait::async_trait;
use clap::{Args, Subcommand};
use libcli_rs::output;
use libcli_rs::output::{OutputFactory, OutputTrait};
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::lock_huber_ops;
use crate::model::config::Config;
use crate::service::config::{ConfigService, ConfigTrait};

#[derive(Subcommand)]
pub enum ConfigCommands {
    #[command(about = "Show Huber configurations", bin_name = "show")]
    Show(ConfigShowArgs),

    #[command(
        about = "Save Huber configurations via global options",
        bin_name = "save"
    )]
    Save(ConfigSaveArgs),
}

#[derive(Args)]
pub struct ConfigShowArgs {}

#[async_trait]
impl CommandTrait for ConfigShowArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let config_service = container.get::<ConfigService>().unwrap();
        let saved_config = config_service.get()?;

        output!(
            config.output_format,
            .display(
                stdout(),
                &saved_config,
                None,
                None,
            )
        )
    }
}

#[derive(Args)]
pub struct ConfigSaveArgs {}

#[async_trait]
impl CommandTrait for ConfigSaveArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        lock_huber_ops!(config);

        let config_service = container.get::<ConfigService>().unwrap();
        let config_path = config.config_file()?;
        info!("Saving config to {:?}: {:#?}", config_path, config);
        config_service.update(config)?;
        info!("Config saved");

        Ok(())
    }
}

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}
