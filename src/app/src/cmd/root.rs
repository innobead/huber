use async_trait::async_trait;
use clap::{crate_name, crate_version, App, Arg, ArgMatches};

use huber_common::config::Config;
use huber_common::result::Result;

use crate::cmd::{CommandAsyncTrait, CommandTrait};
use huber_common::di::DIContainer;

pub(crate) const ARG_LOG_LEVEL: &str = "log-level";
pub(crate) const ARG_OUTPUT_TYPE: &str = "output";
pub(crate) const ARG_GITHUB_TOKEN: &str = "github-token";
// pub(crate) const ARG_GIT_SSH_KEY: &str = "git-key";

#[derive(Debug)]
pub(crate) struct RootCmd;
unsafe impl Send for RootCmd {}
unsafe impl Sync for RootCmd {}

impl RootCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for RootCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(crate_name!())
            .version(crate_version!())
            .long_version(crate_version!())
            .about("Huber, simplify github package management")
            .args(&[
                Arg::with_name(ARG_LOG_LEVEL)
                    .value_name("string")
                    .short("l")
                    .long(ARG_LOG_LEVEL)
                    .help("Log level")
                    .takes_value(true)
                    .global(true)
                    .default_value("off")
                    .possible_values(&["off", "error", "warn", "info", "debug", "trace"]),
                Arg::with_name(ARG_OUTPUT_TYPE)
                    .value_name("string")
                    .short("o")
                    .long(ARG_OUTPUT_TYPE)
                    .help("Output format")
                    .takes_value(true)
                    .global(true)
                    .default_value("console")
                    .possible_values(&["console", "json", "yaml"]),
                Arg::with_name(ARG_GITHUB_TOKEN)
                    .value_name("string")
                    .long(ARG_GITHUB_TOKEN)
                    .env("GITHUB_TOKEN")
                    .help("Github token, used for authored access instead of limited public access")
                    .takes_value(true)
                    .global(true),
                // Arg::with_name(ARG_GIT_SSH_KEY)
                //     .value_name("string")
                //     .long(ARG_GIT_SSH_KEY)
                //     .help("SSH key to access git repository, used for authored access of self managed package source repository")
                //     .takes_value(true)
                //     .global(true),
            ])
    }
}

#[async_trait]
impl<'a, 'b> CommandAsyncTrait<'a, 'b> for RootCmd {
    async fn run(
        &self,
        _config: &Config,
        _container: &DIContainer,
        _matches: &ArgMatches<'a>,
    ) -> Result<()> {
        unimplemented!()
    }
}
