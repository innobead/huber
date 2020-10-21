use clap::{App, ArgMatches};

use crate::cmd::CommandTrait;
use huber_common::di::{DIContainer, DIObjectTrait, MutableRc};
use huber_common::result::Result;

pub(crate) const CMD_NAME: &str = "list";

pub(crate) struct ListCmd {
    container: MutableRc<DIContainer>,
}

impl DIObjectTrait for ListCmd {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self {
        Self { container }
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for ListCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("List installed package")
    }

    fn run(&self, matches: &ArgMatches) -> Result<()> {
        unimplemented!()
    }
}
