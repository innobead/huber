use std::fs::File;
use std::io::Write;

use async_trait::async_trait;
use clap::{Args, ValueHint};
use filepath::FilePath;
use huber_common::model::config::Config;
use log::info;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::service::release::ReleaseService;
use crate::service::ItemOperationTrait;

#[derive(Args)]
pub struct SaveArgs {
    #[arg(
        help = "Save the list of installed 'current' packages to a file",
        long,
        num_args = 1,
        default_value = "huber-packages.txt",
        value_hint = ValueHint::FilePath
    )]
    file: String,
}

#[async_trait]
impl CommandTrait for SaveArgs {
    async fn run(&self, _: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let release_service = container.get::<ReleaseService>().unwrap();

        info!("Collecting installed current packages");
        let versions: Vec<_> = release_service
            .list()?
            .iter()
            .filter(|r| r.current)
            .map(|r| format!("{}@{}", r.package.name, r.version))
            .collect();

        if versions.is_empty() {
            info!("No packages installed");
            return Ok(())
        }

        info!("Saving the package list to {}", self.file);
        let mut file = File::create(&self.file)?;
        file.write_all(versions.join("\n").as_bytes())?;
        info!(
            "Saved the package list to {}",
            file.path()?.canonicalize()?.to_string_lossy()
        );

        Ok(())
    }
}
