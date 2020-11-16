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

use crate::cmd::current::CurrentCmd;
use crate::cmd::flush::FlushCmd;
use crate::cmd::info::InfoCmd;
use crate::cmd::install::InstallCmd;
use crate::cmd::repo::add::RepoAddCmd;
use crate::cmd::repo::list::RepoListCmd;
use crate::cmd::repo::remove::RepoRemoveCmd;
use crate::cmd::repo::RepoCmd;
use crate::cmd::reset::ResetCmd;
use crate::cmd::root::RootCmd;
use crate::cmd::search::SearchCmd;
use crate::cmd::show::ShowCmd;
use crate::cmd::uninstall::UninstallCmd;
use crate::cmd::update::UpdateCmd;
use crate::cmd::CommandTrait;
use crate::service::cache::CacheService;
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::repo::RepoService;
use crate::service::update::UpdateService;
use crate::service::ServiceTrait;

mod cmd;
mod component;
mod service;

#[tokio::main]
async fn main() {
    let mut container = DIContainer::new();
    let mut config = Config::new();

    // create CLI app, do CLI args/commands match
    let cmds = vec![
        create_dep!(InstallCmd, container, .app()),
        create_dep!(UninstallCmd, container, .app()),
        create_dep!(UpdateCmd, container, .app()),
        create_dep!(SearchCmd, container, .app()),
        create_dep!(InfoCmd, container, .app()),
        create_dep!(ShowCmd, container, .app()),
        create_dep!(CurrentCmd, container, .app()),
        create_dep!(FlushCmd, container, .app()),
        create_dep!(ResetCmd, container, .app()),
        // nested commands
        create_dep!(RepoCmd, container, .app()).subcommands(vec![
            create_dep!(RepoAddCmd, container, .app()),
            create_dep!(RepoRemoveCmd, container, .app()),
            create_dep!(RepoListCmd, container, .app()),
        ]),
    ];

    let app = RootCmd::new().app().subcommands(cmds);
    let matches = cmd::prepare_arg_matches(app);

    // process global args and init config
    cmd::process_arg_matches(&mut config, &matches);
    let _ = config.init();

    // init services
    create_dep!(PackageService, container);
    create_dep!(ReleaseService, container);
    create_dep!(CacheService, container);
    create_dep!(UpdateService, container);
    create_dep!(RepoService, container);

    // inject dependencies to the container objects
    let config_arc = Arc::new(config);
    let container_arc = Arc::new(container);

    inject_dep!(PackageService, config_arc.clone(), container_arc.clone());
    inject_dep!(ReleaseService, config_arc.clone(), container_arc.clone());
    inject_dep!(CacheService, config_arc.clone(), container_arc.clone());
    inject_dep!(UpdateService, config_arc.clone(), container_arc.clone());
    inject_dep!(RepoService, config_arc.clone(), container_arc.clone());

    // process command
    if let Err(e) = cmd::process_cmds(&config_arc.clone(), &container_arc.clone(), &matches).await {
        eprintln!("Error: {:?}", e);
        exit(1)
    }
}
