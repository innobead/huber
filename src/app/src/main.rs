#![allow(dead_code)]

#[macro_use]
extern crate huber_common;

use std::env;
use std::path::Path;
use std::process::exit;

use chrono::{Local, Utc};
use log::error;
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::DIContainer;
use huber_common::di::DIObjectTrait;

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
use crate::service::release::ReleaseService;
use std::borrow::Borrow;

mod cmd;
mod service;
mod component;

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

    // init DI managed objects
    di!(ReleaseService);
    di!(DatastoreService);
    di!(ContextService);
    di!(CacheService dir=Path::new("/tmp/huber").to_owned());

    // process command
    let runtime = Runtime::new().unwrap();
    if let Err(err) = cmd::process_cmds(&runtime, &config, &matches, DIContainer::new()) {
        error!("Failed to run command: {:?}", err);
        exit(1)
    }
}
