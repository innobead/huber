use clap::{App, Arg, ArgMatches};

use crate::base::di::{DIContainer, DIObjectTrait, MutableRc};
use crate::base::result::Result;
use crate::cmd::CommandTrait;

pub(crate) const CMD_NAME: &str = "search";

pub(crate) struct SearchCmd {
    container: MutableRc<DIContainer>,
}

impl DIObjectTrait for SearchCmd {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self {
        Self { container }
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
            Arg::with_name("repo")
                .short("r")
                .long("repo")
                .help("Github repo URL")
                .takes_value(true),
        ])
    }

    fn run(&self, matches: &ArgMatches) -> Result<()> {
        unimplemented!()
    }
}
