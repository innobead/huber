use std::cmp::Ordering;
use std::collections::HashMap;

use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use log::warn;
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use huber_common::model::config::Config;
use huber_common::model::config::ConfigPath;
use huber_common::model::release::Release;
use huber_common::progress::progress;
use huber_common::result::Result;
use huber_procmacro::process_lock;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use crate::service::package::PackageService;
use crate::service::release::{ReleaseService, ReleaseTrait};
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait};

pub(crate) const CMD_NAME: &str = "update";

#[derive(Debug)]
pub(crate) struct UpdateCmd;

unsafe impl Send for UpdateCmd {}

unsafe impl Sync for UpdateCmd {}

impl UpdateCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'help> CommandTrait<'help> for UpdateCmd {
    fn app(&self) -> Command<'help> {
        Command::new(CMD_NAME)
            .visible_alias("up")
            .about("Updates the installed package(s)")
            .args([
                Arg::new("name")
                    .multiple_occurrences(true)
                    .value_name("package name")
                    .help("Package name(s)")
                    .required(false)
                    .takes_value(true),
                Arg::new("dryrun")
                    .short('d')
                    .long("dryrun")
                    .help("Dry run to show available updates")
                    .required(false),
            ])
    }
}

#[async_trait]
impl CommandAsyncTrait for UpdateCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        matches: &ArgMatches,
    ) -> Result<()> {
        process_lock!();

        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        let mut names: Vec<String> = vec![];
        let mut release_caches: HashMap<String, Release> = hashmap! {};

        if matches.is_present("name") {
            let _names: Vec<&str> = matches.values_of("name").unwrap().collect();
            for n in _names {
                names.push(n.to_string())
            }
        } else {
            for r in release_service.list()? {
                names.push(r.name.clone());
                release_caches.insert(r.name.clone(), r);
            }
        }

        for ref name in names {
            if !release_service.has(name)? {
                return Err(anyhow!("{} not found", name));
            }

            let pkg = pkg_service.get(name)?;

            if !release_caches.contains_key(name) {
                let r = release_service.current(&pkg)?;
                release_caches.insert(name.to_string(), r);
            };

            let release = release_caches.get(name).unwrap();

            match release_service.get_latest(&pkg).await {
                Ok(mut release_latest) => match release_latest.compare(&release)? {
                    Ordering::Greater => {
                        update(&release_service, &matches, &release_latest, &release).await?;
                    }

                    _ => {
                        let sorted_pkgs_sum = pkg_service.find_summary(name, true).await?;
                        let latest_pkg_sum = sorted_pkgs_sum.first().unwrap();

                        if latest_pkg_sum.version.as_ref().unwrap() != &release.version {
                            release_latest.version = latest_pkg_sum.version.clone().unwrap();
                            release_latest.package.version = Some(release_latest.version.clone());

                            update(&release_service, &matches, &release_latest, &release).await?;
                        } else {
                            println!("The installed {} is the latest version already", release);
                            continue;
                        }
                    }
                },

                Err(e) => {
                    warn!(
                        "{}, the latest release not found (possibly prerelease available only): {}",
                        release, e,
                    )
                }
            }
        }

        Ok(())
    }
}

async fn update(
    release_service: &ReleaseService,
    matches: &ArgMatches,
    new_release: &Release,
    installed_release: &Release,
) -> Result<()> {
    if matches.is_present("dryrun") {
        println!("{} -> {}", installed_release, new_release);
    } else {
        progress(&format!(
            "Updating {} to {}",
            installed_release, new_release
        ))?;
        release_service.update(&new_release.package).await?;
        println!("{} to {} updated", installed_release, new_release);
    }

    Ok(())
}
