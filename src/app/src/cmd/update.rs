use std::collections::HashMap;

use async_trait::async_trait;
use clap::{App, Arg, ArgMatches};
use simpledi_rs::di::{DIContainer, DIContainerTrait};

use huber_common::model::config::Config;
use huber_common::model::config::ConfigPath;
use huber_common::model::release::Release;
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

impl<'a, 'b> CommandTrait<'a, 'b> for UpdateCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("u")
            .about("Updates the installed package(s)")
            .args(&vec![Arg::with_name("name")
                .multiple(true)
                .value_name("package name")
                .help("Package name(s)")
                .required(false)
                .takes_value(true)])
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for UpdateCmd {
    async fn run(
        &self,
        _config: &Config,
        container: &DIContainer,
        matches: &ArgMatches<'a>,
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
            let release_latest = release_service.get_latest(&pkg).await?;

            if release.version == release_latest.version {
                println!("{}, the latest version already installed", release);
                continue;
            }

            println!(
                "Updating {} to the latest version {}",
                release, release_latest
            );
            release_service.update(&pkg).await?;
            println!("{} updated", pkg);
        }

        Ok(())
    }
}
