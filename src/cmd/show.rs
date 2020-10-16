use clap::{App, Arg, ArgMatches};

use crate::cmd::Command;

pub(crate) const CMD_NAME: &str = "show";

pub(crate) struct ShowCmd;

impl<'a, 'b> Command<'a, 'b> for ShowCmd {
    fn app() -> App<'a, 'b> {
        App::new(CMD_NAME)
            .about("Show installed package")
            .arg(
                Arg::with_name("name")
                    .help("Package name")
                    .required(true)
                    .takes_value(true)
            )
    }

    fn run(matches: &ArgMatches) -> anyhow::Result<()> {
        let name = matches.value_of("name").unwrap();

        unimplemented!()
    }
}