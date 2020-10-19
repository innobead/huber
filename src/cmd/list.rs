use clap::{App, ArgMatches};

use crate::base::di::{DIContainer, DIObjectTrait, MutableRc};
use crate::base::result::Result;
use crate::cmd::CommandTrait;

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
