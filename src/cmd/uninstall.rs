use clap::{App, Arg, ArgMatches};

use crate::base::di::{DIContainer, DIObjectTrait, MutableRc};
use crate::cmd::CommandTrait;

pub(crate) const CMD_NAME: &str = "uninstall";

pub(crate) struct UninstallCmd {
    container: MutableRc<DIContainer>,
}

impl DIObjectTrait for UninstallCmd {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self {
        Self { container }
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for UninstallCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Uninstall package").arg(
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
