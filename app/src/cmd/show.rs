use clap::{App, Arg, ArgMatches};

use huber_common::di::{DIContainer, DIObjectTrait, MutableRc};
use crate::cmd::CommandTrait;

pub(crate) const CMD_NAME: &str = "show";

pub(crate) struct ShowCmd {
    container: MutableRc<DIContainer>,
}

impl DIObjectTrait for ShowCmd {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self {
        Self { container }
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for ShowCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Show installed package").arg(
            Arg::with_name("name")
                .help("Package name")
                .required(true)
                .takes_value(true),
        )
    }

    fn run(&self, matches: &ArgMatches) -> anyhow::Result<()> {
        let name = matches.value_of("name").unwrap();

        unimplemented!()
    }
}
