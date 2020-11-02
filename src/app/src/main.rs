#![allow(dead_code)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate huber_common;

use std::borrow::Borrow;
use std::env;
use std::path::Path;
use std::process::exit;
use std::rc::Rc;
use std::sync::Arc;

use chrono::{Local, Utc};
use log::error;
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::DIContainer;

use crate::cmd::CommandTrait;
use crate::cmd::info::InfoCmd;
use crate::cmd::install::InstallCmd;
use crate::cmd::list::ListCmd;
use crate::cmd::root::RootCmd;
use crate::cmd::search::SearchCmd;
use crate::cmd::show::ShowCmd;
use crate::cmd::uninstall::UninstallCmd;
use crate::service::cache::CacheService;
use crate::service::context::ContextService;
use crate::service::datastore::DatastoreService;
use crate::service::package::PackageService;

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
            di!(ListCmd.app()),
            di!(ShowCmd.app()),
            di!(InfoCmd.app()),
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
    let runtime = Arc::new(Runtime::new().unwrap());

    di!(PackageService
        config=Some(config.clone())
        runtime=Some(runtime.clone()));

    di!(DatastoreService
        config=Some(config.clone())
        runtime=Some(runtime.clone()));

    di!(ContextService
        config=Some(config.clone())
        runtime=Some(runtime.clone()));

    di!(CacheService
        dir=Path::new("/tmp/huber").to_owned()
        config=Some(config.clone())
        runtime=Some(runtime.clone()));

    // process command
    if let Err(err) = cmd::process_cmds(&runtime, &config, &matches, DIContainer::new()) {
        error!("Failed to run command: {:?}", err);

        exit(1)
    }
}
