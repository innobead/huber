use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::anyhow;
use clap::{CommandFactory, Parser, ValueHint};
use clap_complete::Generator;
use huber::cmd::config::ConfigCommands;
use huber::cmd::lock::LockCommands;
use huber::cmd::repo::RepoCommands;
use huber::cmd::CommandTrait;
use huber::cmd::Commands;
use huber::error::HuberError;
use huber::service::config::{ConfigService, DEFAULT_CONFIG};
use huber::service::init_services;
use huber_common::fs::dir;
use huber_common::log::Logger;
use huber_common::model::config::Config;
use libcli_rs::output::OutputFormat;
use log::{debug, error, LevelFilter};
use scopeguard::defer;
use simpledi_rs::di::{DIContainer, DIContainerTrait, DependencyInjectTrait};
use simpledi_rs::inject_dep;

#[derive(Parser)]
#[command(version, bin_name = env!("CARGO_PKG_NAME"), about, long_about = None)]
struct Cli {
    #[arg(
        help = "Log level",
        short = 'l',
        long,
        global = true,
        num_args = 1,
        default_value_t = get_default_log_level(),
        value_parser = parse_log_level
    )]
    log_level: LevelFilter,

    #[arg(
        help = "GitHub token; Optional until reaching the rate limit of GitHub API",
        long,
        global = true,
        num_args = 1,
        value_hint = ValueHint::Unknown,
        env = "GITHUB_TOKEN",
    )]
    github_token: Option<String>,

    #[arg(
        help = "Github SSH key path; Optional, if you want to use SSH to clone the Huber repository",
        long,
        global = true,
        num_args = 1,
        value_hint = ValueHint::FilePath,
        env = "GITHUB_KEY",
    )]
    github_key: Option<String>,

    #[arg(
        help = "Output format",
        short,
        long,
        global = true,
        num_args = 1,
        value_hint = ValueHint::Unknown,
        default_value_t = get_default_output_format(),
        value_parser = parse_output_format,
        hide = true,
    )]
    output_format: String,

    #[arg(
        help = "Huber directory",
        long,
        global = true,
        num_args = 1,
        value_hint = ValueHint::DirPath,
        default_value_t = get_default_huber_dir(),
        value_parser = parse_huber_dir
    )]
    huber_dir: String,

    #[arg(
        help = "GitHub base URI",
        long,
        global = true,
        num_args = 1,
        value_hint = ValueHint::Url,
        env = "GITHUB_BASE_URI",
        default_value = "https://api.github.com"
    )]
    github_base_uri: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let (config, container) = init(&cli);

    let result = match &cli.command {
        Commands::Install(args) => args.run(&config, &container).await,
        Commands::Config(args) => match args.command {
            ConfigCommands::Show(ref args) => args.run(&config, &container).await,
            ConfigCommands::Save(ref args) => args.run(&config, &container).await,
        },
        Commands::Repo(args) => match args.command {
            RepoCommands::Add(ref args) => args.run(&config, &container).await,
            RepoCommands::Remove(ref args) => args.run(&config, &container).await,
            RepoCommands::Show(ref args) => args.run(&config, &container).await,
        },
        Commands::Current(args) => args.run(&config, &container).await,
        Commands::Flush(args) => args.run(&config, &container).await,
        Commands::Info(args) => args.run(&config, &container).await,
        Commands::Reset(args) => args.run(&config, &container).await,
        Commands::Search(args) => args.run(&config, &container).await,
        Commands::SelfUpdate(args) => args.run(&config, &container).await,
        Commands::Show(args) => args.run(&config, &container).await,
        Commands::Uninstall(args) => args.run(&config, &container).await,
        Commands::Update(args) => args.run(&config, &container).await,
        Commands::Save(args) => args.run(&config, &container).await,
        Commands::Load(args) => args.run(&config, &container).await,
        Commands::Lock(args) => {
            if let Some(cmd) = &args.command {
                match cmd {
                    LockCommands::Show(args) => args.run(&config, &container).await,
                }
            } else {
                args.run(&config, &container).await
            }
        }
        Commands::Unlock(args) => args.run(&config, &container).await,
        Commands::Completions { shell } => {
            shell.generate(&Cli::command(), &mut io::stdout());
            Ok(())
        }
    };

    if let Err(e) = result {
        defer! {
            debug!("{:?}", e);
            exit(1);
        }

        if let Some(e) = e.downcast_ref::<HuberError>() {
            let source_err = e.source().map(|e| format!(": {}", e)).unwrap_or_default();
            error!("{}{}", e, source_err);
        } else {
            error!("Unknown: {}", e);
        }
    }
}

fn init(cli: &Cli) -> (Config, Arc<DIContainer>) {
    better_panic::install();

    let config = Config::new(
        cli.log_level.to_string(),
        OutputFormat::from_str(&cli.output_format).unwrap(),
        dir(PathBuf::from(&cli.huber_dir)).unwrap(),
        cli.github_token.clone(),
        cli.github_key.clone(),
        cli.github_base_uri.clone(),
        Default::default(),
    );

    Logger::init(&config).expect("Failed to init logger");

    let container = init_services(&config);
    inject_dep!(ConfigService, container.clone());

    (config, container)
}

fn parse_log_level(log_level: &str) -> anyhow::Result<LevelFilter> {
    Ok(LevelFilter::from_str(&log_level.to_uppercase())?)
}

fn parse_output_format(format: &str) -> anyhow::Result<String> {
    OutputFormat::from_str(format)
        .map_err(|_| anyhow!("Invalid output format: {}", format))
        .map(|t| match t {
            OutputFormat::Console => "console".to_string(),
            OutputFormat::Yaml => "yaml".to_string(),
            OutputFormat::Json => "json".to_string(),
        })
}

fn parse_huber_dir(dir: &str) -> anyhow::Result<String> {
    let p = PathBuf::from(dir);
    if p.exists() && !p.is_dir() {
        return Err(anyhow!("Huber dir ({}) is not a directory", dir));
    }

    Ok(p.into_os_string()
        .into_string()
        .expect("Failed to parse huber dir"))
}

fn get_default_huber_dir() -> String {
    DEFAULT_CONFIG
        .huber_dir
        .clone()
        .into_os_string()
        .into_string()
        .expect("Failed to get default huber dir")
}

fn get_default_log_level() -> LevelFilter {
    LevelFilter::from_str(DEFAULT_CONFIG.log_level.as_str())
        .expect("Failed to get default log level")
}

fn get_default_output_format() -> String {
    //TODO: fix this in libcli-rs
    match DEFAULT_CONFIG.output_format {
        OutputFormat::Console => "console".to_string(),
        OutputFormat::Yaml => "yaml".to_string(),
        OutputFormat::Json => "json".to_string(),
    }
}
