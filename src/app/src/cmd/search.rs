use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;

use clap::{App, Arg, ArgMatches};
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::{container, DIContainer, MutableRc};
use huber_common::output;
use huber_common::output::OutputTrait;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::service::{ItemOperationTrait, ItemSearchTrait};
use crate::service::release::ReleaseService;

pub(crate) const CMD_NAME: &str = "search";

pub(crate) struct SearchCmd;

impl SearchCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for SearchCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Search package").args(&[
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Package name")
                .takes_value(true),
        ])
    }

    fn run(&self, runtime: &Runtime, config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let container = container();
        let release_service = container.get::<ReleaseService>().unwrap();
        let results = release_service.search(runtime, "")?;

        output::new(&config.output_format).display(
            std::io::stdout(),
            &results,
            Some(vec!["name"]),
            None,
        )
    }
}
