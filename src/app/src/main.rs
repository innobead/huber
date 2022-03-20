#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate libcli_rs;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate simpledi_rs;

use std::process::exit;
use std::sync::Arc;

use simpledi_rs::di::{DependencyInjectTrait, DIContainer, DIContainerTrait};

use huber_common::model::config::Config;

use crate::cmd::CommandTrait;
use crate::cmd::config::ConfigCmd;
use crate::cmd::config::show::ConfigShowCmd;
use crate::cmd::config::update::ConfigUpdateCmd;
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
use crate::cmd::self_update::SelfUpdateCmd;
use crate::cmd::show::ShowCmd;
use crate::cmd::uninstall::UninstallCmd;
use crate::cmd::update::UpdateCmd;
use crate::service::cache::CacheService;
use crate::service::config::ConfigService;
use crate::service::package::PackageService;
use crate::service::release::ReleaseService;
use crate::service::repo::RepoService;
use crate::service::update::UpdateService;

mod cmd;
mod component;
mod service;

#[tokio::main]
async fn main() {
    let mut container = DIContainer::new();
    let mut config = Config::new();

    // Create CLI app, do CLI args/commands match
    let cmds = [
        create_dep!(InstallCmd::new(), container, .app()),
        create_dep!(UninstallCmd::new(), container, .app()),
        create_dep!(UpdateCmd::new(), container, .app()),
        create_dep!(SelfUpdateCmd::new(), container, .app()),
        create_dep!(SearchCmd::new(), container, .app()),
        create_dep!(InfoCmd::new(), container, .app()),
        create_dep!(ShowCmd::new(), container, .app()),
        create_dep!(CurrentCmd::new(), container, .app()),
        create_dep!(FlushCmd::new(), container, .app()),
        create_dep!(ResetCmd::new(), container, .app()),
        // nested commands
        create_dep!(RepoCmd::new(), container, .app()).subcommands(vec![
            create_dep!(RepoAddCmd::new(), container, .app()),
            create_dep!(RepoRemoveCmd::new(), container, .app()),
            create_dep!(RepoListCmd::new(), container, .app()),
        ]),
        create_dep!(ConfigCmd::new(), container, .app()).subcommands(vec![
            create_dep!(ConfigShowCmd::new(), container, .app()),
            create_dep!(ConfigUpdateCmd::new(), container, .app()),
        ]),
    ];
    let app = RootCmd::new().app().subcommands(cmds);
    let matches = cmd::prepare_arg_matches(app);

    // Init config
    cmd::update_config_by_arg_matches(&mut config, &matches);
    let container = init_config(config, container);

    // Init service
    let container = init_services(container);

    // Process command
    if let Err(e) = cmd::process_cmds(&container.clone(), &matches).await {
        eprintln!("{:?}", e);
        exit(1)
    }
}

fn init_config(config: Config, mut container: DIContainer) -> DIContainer {
    let _ = config.init();
    create_dep!(config, container);

    container
}

fn init_services(mut container: DIContainer) -> Arc<DIContainer> {
    create_dep!(PackageService::new(), container);
    create_dep!(ReleaseService::new(), container);
    create_dep!(CacheService::new(), container);
    create_dep!(UpdateService::new(), container);
    create_dep!(RepoService::new(), container);
    create_dep!(ConfigService::new(), container);

    let container = container.init().unwrap();

    // Inject dependencies to the container objects
    inject_dep!(PackageService, container.clone());
    inject_dep!(ReleaseService, container.clone());
    inject_dep!(CacheService, container.clone());
    inject_dep!(UpdateService, container.clone());
    inject_dep!(RepoService, container.clone());
    inject_dep!(ConfigService, container.clone());

    container
}
