#![allow(dead_code)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate huber_common;

use std::env;
use std::process::exit;
use std::sync::Arc;

use log::debug;

use huber_common::config::Config;
use huber_common::di::{di_container, DIContainer};

use crate::cmd::current::CurrentCmd;
use crate::cmd::flush::FlushCmd;
use crate::cmd::info::InfoCmd;
use crate::cmd::install::InstallCmd;
use crate::cmd::reset::ResetCmd;
use crate::cmd::root::RootCmd;
use crate::cmd::search::SearchCmd;
use crate::cmd::self_update::SelfUpdateCmd;
use crate::cmd::show::ShowCmd;
use crate::cmd::uninstall::UninstallCmd;
use crate::cmd::CommandTrait;
use crate::service::cache::{CacheService, CacheTrait};
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::update::UpdateService;

mod cmd;
mod component;
mod service;

fn main() {
    // create CLI app
    let app = {
        let app = RootCmd::new().app().subcommands(vec![
            di!(InstallCmd.app()),
            di!(UninstallCmd.app()),
            di!(SearchCmd.app()),
            di!(ShowCmd.app()),
            di!(InfoCmd.app()),
            di!(SelfUpdateCmd.app()),
            di!(CurrentCmd.app()),
            di!(ResetCmd.app()),
            di!(FlushCmd.app()),
        ]);

        app
    };

    // do CLI args/commands match
    let mut args = env::args();
    let matches = if args.len() == 1 {
        app.get_matches_from(vec![args.nth(0).unwrap(), "help".to_string()])
    } else {
        app.get_matches()
    };

    // process global args and init config
    let mut config = Config::new();
    cmd::process_args(&mut config, &matches);
    let _ = config.init();
    let config = Arc::new(config);

    // init DI managed objects
    // FIXME when reqwest upgrade with tokio 0.3, https://github.com/seanmonstar/reqwest/issues/1060
    // let runtime = Arc::new(Runtime::new().unwrap());

    di!(PackageService
        config=Some(config.clone()));
    // runtime=Some(runtime.clone()));

    di!(ReleaseService
        config=Some(config.clone()));
    // runtime=Some(runtime.clone()));

    di!(CacheService
        config=Some(config.clone()));
    // runtime=Some(runtime.clone()));

    di!(UpdateService
        config=Some(config.clone()));
    // runtime=Some(runtime.clone()));

    // update cache
    update_cache();

    // process command
    // if let Err(e) = cmd::process_cmds(&runtime, &config, &matches, DIContainer::new()) {
    if let Err(e) = cmd::process_cmds(&config, &matches, DIContainer::new()) {
        eprintln!("Error: {:?}", e);
        exit(1)
    }
}

fn update_cache() {
    let container = di_container();
    let cache_service = container.get::<CacheService>().unwrap();
    if cache_service.update().is_err() {
        debug!("Failed to update cache");
    }
}
