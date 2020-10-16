use clap::{App, Arg, crate_name, crate_version, ArgMatches};
use crate::cmd::Command;

pub(crate) const ARG_LOG_LEVEL: &str = "log-level";

pub(crate) struct RootCmd;

impl<'a, 'b> Command<'a, 'b> for RootCmd {
    fn app() -> App<'a, 'b> {
        App::new(crate_name!())
            .version(crate_version!())
            .long_version(crate_version!())
            .about("Huber, simplify package management of github release")
            .args(&[
                Arg::with_name(ARG_LOG_LEVEL)
                    .short("l")
                    .long(ARG_LOG_LEVEL)
                    .help("Log level")
                    .takes_value(true)
                    .global(true)
                    .possible_values(&["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"]),
            ])
    }

    fn run(matches: &ArgMatches) -> anyhow::Result<()> {
        unimplemented!()
    }
}
