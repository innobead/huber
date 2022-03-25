use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use huber_common::model::config::Config;
use huber_common::model::config::ConfigPath;
use huber_common::progress::progress;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::cache::{CacheAsyncTrait, CacheService};
use crate::service::package::PackageService;
use crate::service::release::{ReleaseService, ReleaseTrait};
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

pub(crate) const CMD_NAME: &str = "install";

#[derive(Debug)]
pub(crate) struct InstallCmd;

unsafe impl Send for InstallCmd {}

unsafe impl Sync for InstallCmd {}

impl InstallCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'help> CommandTrait<'help> for InstallCmd {
    fn app(&self) -> Command<'help> {
        Command::new(CMD_NAME)
            .visible_alias("in")
            .about("Installs the package")
            .args([
                Arg::new("name")
                    .value_name("package name")
                    .multiple_occurrences(true)
                    .help("Package name(s)")
                    .required(true)
                    .takes_value(true),
                Arg::new("version")
                    .value_name("string")
                    .help("Package version")
                    .short('v')
                    .long("version")
                    .takes_value(true),
            ])
    }
}

#[async_trait]
impl CommandAsyncTrait for InstallCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        matches: &ArgMatches,
    ) -> Result<()> {
        process_lock!();

        let names: Vec<&str> = matches.values_of("name").unwrap().collect();

        for name in names {
            let release_service = container.get::<ReleaseService>().unwrap();
            let pkg_service = container.get::<PackageService>().unwrap();

            let cache_service = container.get::<CacheService>().unwrap();
            let _ = cache_service.update_repositories().await?;

            if !pkg_service.has(name)? {
                return Err(anyhow!("{} not found", name));
            }

            let mut pkg = pkg_service.get(name)?;
            pkg.version = matches.value_of("version").map(|it| it.to_string());

            if release_service.has(name)? {
                let release = release_service.current(&pkg)?;

                return match &matches {
                    _ if matches.is_present("version") => {
                        if release.version == matches.value_of("version").unwrap() {
                            Err(anyhow!("{} already installed", release))
                        } else {
                            progress(&format!("Updating {} to {}", release, pkg.version.as_ref().unwrap()))?;

                            let release = release_service.update(&pkg).await?;
                            println!("{} updated", release);
                            Ok(())
                        }
                    }

                    _ => {
                        Err(anyhow!("{} already installed. Use '--version' to install a specific version or 'update' command to update to the latest version", release))
                    }
                };
            }

            progress(&format!("Installing {}", &pkg))?;
            let release = release_service.create(pkg).await?;
            println!("{} installed", release);
        }

        Ok(())
    }
}
