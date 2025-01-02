use std::io::stdout;

use async_trait::async_trait;
use clap::Args;
use huber_common::model::config::Config;
use libcli_rs::output;
use libcli_rs::output::{OutputFactory, OutputTrait};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use crate::cmd::CommandTrait;
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::ItemOperationTrait;

#[derive(Args)]
pub struct InfoArgs {
    #[arg(help = "Package name")]
    name: String,
}

#[async_trait]
impl CommandTrait for InfoArgs {
    async fn run(&self, config: &Config, container: &DIContainer) -> anyhow::Result<()> {
        let pkg_service = container.get::<PackageService>().unwrap();
        let release_service = container.get::<ReleaseService>().unwrap();

        let pkg = pkg_service.get(&self.name)?;
        let release = release_service.get_latest(&pkg).await?;

        output!(
            config.output_format,
            .display(
                stdout(),
                &release.package,
                None,
                Some(vec!["detail"]),
            )
        )
    }
}
