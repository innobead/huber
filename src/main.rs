#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

use std::env;
use std::path::Path;
use std::process::exit;

use log::error;

use base::di::DIContainer;

use crate::base::config::Config;
use crate::base::di::DIObjectTrait;
use crate::cmd::{
    CommandTrait, InfoCmd, InstallCmd, ListCmd, RootCmd, SearchCmd, ShowCmd, UninstallCmd,
};
use crate::service::cache::CacheService;
use crate::service::context::ContextService;
use crate::service::datastore::DatastoreService;
use crate::service::release::ReleaseService;

mod base;
mod cmd;
mod component;
mod model;
mod service;

fn main() {
    // create CLI app
    let app = {
        let app = di!(RootCmd.app()).subcommands(vec![
            di!(InstallCmd.app()),
            di!(UninstallCmd.app()),
            di!(SearchCmd.app()),
            di!(ListCmd.app()),
            di!(ShowCmd.app()),
            di!(InfoCmd.app()),
        ]);

        di!(ReleaseService);
        di!(DatastoreService);
        di!(ContextService);
        di!(CacheService dir=Path::new("/tmp/huber").to_owned());

        app
    };

    // do app match
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

    // process command
    if let Err(err) = cmd::process_cmds(&config, &matches, DIContainer::new()) {
        error!("Failed to run command: {:?}", err);
        exit(1)
    }
}
