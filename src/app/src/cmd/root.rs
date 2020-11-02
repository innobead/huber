use clap::{crate_name, crate_version, App, Arg, ArgMatches};

use huber_common::di::{DIContainer, MutableRc};
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use huber_common::config::Config;
use tokio::runtime::Runtime;

pub(crate) const ARG_LOG_LEVEL: &str = "log-level";
pub(crate) const ARG_OUTPUT_TYPE: &str = "output";

pub(crate) struct RootCmd {}

impl RootCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for RootCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(crate_name!())
            .version(crate_version!())
            .long_version(crate_version!())
            .about("Huber, simplify package management of github release")
            .args(&[Arg::with_name(ARG_LOG_LEVEL)
                .short("l")
                .long(ARG_LOG_LEVEL)
                .help("Log level")
                .takes_value(true)
                .global(true)
                .default_value("info")
                .possible_values(&["off", "error", "warn", "info", "debug", "trace"])])
            .args(&[Arg::with_name(ARG_OUTPUT_TYPE)
                .short("o")
                .long(ARG_OUTPUT_TYPE)
                .help("Output format")
                .takes_value(true)
                .global(true)
                .default_value("console")
                .possible_values(&["console", "json", "yaml"])])
    }

    fn run(&self, runtime: &Runtime, config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        unimplemented!()
    }
}
