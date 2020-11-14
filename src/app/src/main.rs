#![allow(dead_code)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate huber_common;
#[macro_use]
extern crate maplit;

use std::process::exit;
use std::sync::Arc;

use huber_common::config::Config;
use huber_common::di::DIContainer;

use crate::cmd::CommandTrait;
use crate::cmd::current::CurrentCmd;
use crate::cmd::flush::FlushCmd;
use crate::cmd::info::InfoCmd;
use crate::cmd::install::InstallCmd;
use crate::cmd::repo::RepoCmd;
use crate::cmd::reset::ResetCmd;
use crate::cmd::root::RootCmd;
use crate::cmd::search::SearchCmd;
use crate::cmd::show::ShowCmd;
use crate::cmd::uninstall::UninstallCmd;
use crate::cmd::update::UpdateCmd;
use crate::service::cache::CacheService;
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::repo::RepoService;
use crate::service::update::UpdateService;

mod cmd;
mod component;
mod service;

fn main() {
    // create CLI app
    let app = {
        let app = RootCmd::new().app().subcommands(vec![
            di!(InstallCmd.app()),
            di!(UpdateCmd.app()),
            di!(UninstallCmd.app()),
            di!(SearchCmd.app()),
            di!(ShowCmd.app()),
            di!(InfoCmd.app()),
            // di!(SelfUpdateCmd.app()),
            di!(CurrentCmd.app()),
            di!(ResetCmd.app()),
            di!(FlushCmd.app()),
            di!(RepoCmd.app()),
        ]);

        app
    };

    // do CLI args/commands match
    let matches = cmd::prepare_arg_matches(app);

    // process global args and init config
    let mut config = Config::new();
    cmd::process_arg_matches(&mut config, &matches);
    let _ = config.init();
    let config = Arc::new(config);

    // init DI managed objects
    // FIXME when reqwest upgrade with tokio 0.3, https://github.com/seanmonstar/reqwest/issues/1060
    // let runtime = Arc::new(Runtime::new().unwrap());
    di!(PackageService
        config=Some(config.clone()));
    di!(ReleaseService
        config=Some(config.clone()));
    di!(CacheService
        config=Some(config.clone()));
    di!(UpdateService
        config=Some(config.clone()));
    di!(RepoService
        config=Some(config.clone()));

    // process command
    // if let Err(e) = cmd::process_cmds(&runtime, &config, &matches, DIContainer::new()) {
    if let Err(e) = cmd::process_cmds(&config, &matches, DIContainer::new()) {
        eprintln!("Error: {:?}", e);
        exit(1)
    }
}
