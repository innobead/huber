use clap::{App, Arg, ArgMatches};

use crate::base::di::{DIContainer, DIObjectTrait, MutableRc};
use crate::base::result::Result;
use crate::cmd::CommandTrait;

pub(crate) const CMD_NAME: &str = "info";

pub(crate) struct InfoCmd {
    container: MutableRc<DIContainer>,
}

impl DIObjectTrait for InfoCmd {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self {
        Self { container }
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for InfoCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Show package info").arg(
            Arg::with_name("name")
                .help("Package name")
                .required(true)
                .takes_value(true),
        )
    }

    fn run(&self, matches: &ArgMatches) -> Result<()> {
        let name = matches.value_of("name").unwrap();
        unimplemented!()
    }
}
