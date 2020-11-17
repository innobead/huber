use async_trait::async_trait;
use clap::{App, Arg, ArgMatches};

use huber_common::di::DIContainer;
use huber_common::model::config::Config;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use huber_common::model::config::ConfigPath;
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

impl<'a, 'b> CommandTrait<'a, 'b> for InstallCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("in")
            .about("Installs the package")
            .args(&vec![
                Arg::with_name("name")
                    .value_name("package name")
                    .help("Package name")
                    .required(true)
                    .takes_value(true),
                Arg::with_name("version")
                    .value_name("string")
                    .help("Package version")
                    .short("v")
                    .long("version")
                    .takes_value(true),
            ])
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for InstallCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        matches: &ArgMatches<'a>,
    ) -> Result<()> {
        process_lock!();

        let name = matches.value_of("name").unwrap();

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
                        println!("Updating {} to {}", pkg, release);

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

        println!("Installing {}", &pkg);
        let release = release_service.create(pkg).await?;
        println!("{} installed", release);

        Ok(())
    }
}
