use std::env;
use std::process::exit;

use log::error;

use crate::base::Config;
use crate::cmd::{Command, InfoCmd, InstallCmd, ListCmd, RootCmd, SearchCmd, ShowCmd, UninstallCmd};

mod cmd;
mod base;

fn main() {
    let app = RootCmd::app()
        .subcommands(vec![
            InstallCmd::app(),
            UninstallCmd::app(),
            SearchCmd::app(),
            ListCmd::app(),
            ShowCmd::app(),
            InfoCmd::app(),
        ]);

    let mut args = env::args();
    let matches = if args.len() == 1 {
        app.get_matches_from(vec![
            args.nth(0).unwrap(),
            "help".to_string(),
        ])
    } else {
        app.get_matches()
    };

    let mut config = Config::new();
    cmd::process_args(&mut config, &matches);
    let _ = config.init();

    if let Err(err) = cmd::process_cmds(&config, &matches) {
        error!("Failed to run command: {:?}", err);
        exit(1)
    }
}

