use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::anyhow;
use clap::Parser;
use huber::cmd::config::ConfigCommands;
use huber::cmd::repo::RepoCommands;
use huber::cmd::CommandTrait;
use huber::cmd::Commands;
use huber::service::cache::CacheService;
use huber::service::config::ConfigService;
use huber::service::config::DEFAULT_CONFIG;
use huber::service::package::PackageService;
use huber::service::release::ReleaseService;
use huber::service::repo::RepoService;
use huber::service::update::HuberUpdateService;
use huber_common::log::Logger;
use huber_common::model::config::Config;
use libcli_rs::output::OutputFormat;
use log::LevelFilter;
use simpledi_rs::di::{DIContainer, DIContainerTrait, DependencyInjectTrait};
use simpledi_rs::{create_dep, inject_dep};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(
        help = "Log level",
        short = 'l',
        long,
        global = true,
        default_value_t = get_default_log_level(),
        value_parser = parse_log_level
    )]
    log_level: LevelFilter,

    #[arg(
        help = "GitHub token",
        long,
        global = true,
        env = "GITHUB_TOKEN",
        group = "github_auth"
    )]
    github_token: Option<String>,

    #[arg(
        help = "Github SSH key path",
        long,
        global = true,
        env = "GITHUB_KEY",
        group = "github_auth"
    )]
    github_key: Option<String>,

    #[arg(
        help = "Output format",
        short,
        long,
        global = true,
        default_value = "console",
        value_parser = parse_output_format,
        hide = true,
    )]
    output_format: OutputFormat,

    #[arg(
        help = "Huber directory",
        long,
        global = true,
        default_value_t = get_default_huber_dir(),
        value_parser = parse_huber_dir
    )]
    huber_dir: String,

    #[arg(
        help = "GitHub base URI",
        long,
        global = true,
        env = "GITHUB_BASE_URI",
        default_value = "https://api.github.com"
    )]
    github_base_uri: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let (config, container) = init(&cli);

    match &cli.command {
        Commands::Install(args) => args.run(&config, &container).await?,
        Commands::Config(args) => match args.command {
            ConfigCommands::Show(ref args) => args.run(&config, &container).await?,
            ConfigCommands::Save(ref args) => args.run(&config, &container).await?,
        },
        Commands::Repo(args) => match args.command {
            RepoCommands::Add(ref args) => args.run(&config, &container).await?,
            RepoCommands::Remove(ref args) => args.run(&config, &container).await?,
            RepoCommands::List(ref args) => args.run(&config, &container).await?,
        },
        Commands::Current(args) => args.run(&config, &container).await?,
        Commands::Flush(args) => args.run(&config, &container).await?,
        Commands::Info(args) => args.run(&config, &container).await?,
        Commands::Reset(args) => args.run(&config, &container).await?,
        Commands::Search(args) => args.run(&config, &container).await?,
        Commands::SelfUpdate(args) => args.run(&config, &container).await?,
        Commands::Show(args) => args.run(&config, &container).await?,
        Commands::Uninstall(args) => args.run(&config, &container).await?,
        Commands::Update(args) => args.run(&config, &container).await?,
        Commands::Save(args) => args.run(&config, &container).await?,
        Commands::Load(args) => args.run(&config, &container).await?,
        Commands::Lock(args) => args.run(&config, &container).await?,
        Commands::Unlock(args) => args.run(&config, &container).await?,
    }

    Ok(())
}

fn init(cli: &Cli) -> (Config, Arc<DIContainer>) {
    let config = Config {
        log_level: cli.log_level.to_string(),
        output_format: cli.output_format.clone(),
        huber_dir: PathBuf::from(&cli.huber_dir),
        github_token: cli.github_token.clone(),
        github_key: cli.github_key.clone(),
        github_base_uri: cli.github_base_uri.clone(),
        lock_pkg_versions: Default::default(),
    };

    Logger::init(&config).expect("Failed to init logger");

    let mut container = DIContainer::new();

    create_dep!(config.clone(), container);
    create_dep!(CacheService::new(), container);
    create_dep!(ConfigService::new(), container);
    create_dep!(PackageService::new(), container);
    create_dep!(ReleaseService::new(), container);
    create_dep!(RepoService::new(), container);
    create_dep!(HuberUpdateService::new(), container);

    let container = container.init().unwrap();

    inject_dep!(PackageService, container.clone());
    inject_dep!(ReleaseService, container.clone());
    inject_dep!(CacheService, container.clone());
    inject_dep!(HuberUpdateService, container.clone());
    inject_dep!(RepoService, container.clone());
    inject_dep!(ConfigService, container.clone());

    (config, container)
}

fn parse_log_level(log_level: &str) -> anyhow::Result<LevelFilter> {
    Ok(LevelFilter::from_str(&log_level.to_uppercase())?)
}

fn parse_output_format(format: &str) -> anyhow::Result<OutputFormat> {
    OutputFormat::from_str(format).map_err(|_| anyhow!("Invalid output format: {}", format))
}

fn parse_huber_dir(dir: &str) -> anyhow::Result<String> {
    let p = PathBuf::from(dir);
    if !p.is_dir() {
        return Err(anyhow!("Invalid huber dir: {}", dir));
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
