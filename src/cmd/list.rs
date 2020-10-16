use clap::{App, Arg, ArgMatches};

use crate::cmd::Command;

pub(crate) const CMD_NAME: &str = "list";

pub(crate) struct ListCmd;

impl<'a, 'b> Command<'a, 'b> for ListCmd {
    fn app() -> App<'a, 'b> {
        App::new(CMD_NAME)
            .about("List installed package")
    }

    fn run(matches: &ArgMatches) -> anyhow::Result<()> {
        unimplemented!()
    }
}