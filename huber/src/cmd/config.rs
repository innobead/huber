use std::io::stdout;

use async_trait::async_trait;
use clap::{Args, Subcommand};
use huber_common::model::config::{Config, ConfigPath};
use huber_procmacro::process_lock;
use libcli_rs::output;
use libcli_rs::output::{OutputFactory, OutputTrait};
use log::{debug, info};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::service::config::{ConfigService, ConfigTrait};

#[derive(Subcommand)]
pub enum ConfigCommands {
    #[command(about = "Show Huber config")]
    Show(ConfigShowArgs),

    #[command(about = "Save Huber config via global options")]
    Save(ConfigSaveArgs),
}

#[derive(Args)]
pub struct ConfigShowArgs {}

#[async_trait]
impl CommandTrait for ConfigShowArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let config_service = container.get::<ConfigService>().unwrap();
        let saved_config = config_service.get()?;

        debug!("Runtime config: {:?}", config);

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
        let config_service = container.get::<ConfigService>().unwrap();
        let lock_file = config.lock_file()?;
        let config_path = config.config_file()?;

        process_lock!(lock_file);

        info!("Updating {:?}: {:#?}", config_path, config);
        config_service.update(config)?;

        Ok(())
    }
}

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}
