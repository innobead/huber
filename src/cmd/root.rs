use clap::{crate_name, crate_version, App, Arg, ArgMatches};

use crate::base::di::{DIContainer, DIObjectTrait, MutableRc};
use crate::base::result::Result;
use crate::cmd::CommandTrait;

pub(crate) const ARG_LOG_LEVEL: &str = "log-level";

pub(crate) struct RootCmd {
    container: MutableRc<DIContainer>,
}

impl DIObjectTrait for RootCmd {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self {
        Self { container }
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
                .possible_values(&["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"])])
    }

    fn run(&self, matches: &ArgMatches) -> Result<()> {
        unimplemented!()
    }
}
