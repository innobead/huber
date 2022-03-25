use std::fs::{remove_file, File};
use std::sync::Arc;

use log::{debug, info};
use simpledi_rs::di::{DIContainer, DIContainerExtTrait, DependencyInjectTrait};

use huber_common::model::config::{Config, ConfigPath};
use huber_common::result::Result;

use crate::service::ServiceTrait;

lazy_static! {
    static ref DEFAULT_CONFIG: Config = Config::new();
}

#[derive(Debug)]
pub(crate) struct ConfigService {
    pub(crate) container: Option<Arc<DIContainer>>,
}

unsafe impl Send for ConfigService {}

unsafe impl Sync for ConfigService {}

pub(crate) trait ConfigTrait {
    fn get(&self) -> Result<Config>;
    fn update(&self, config: &Config) -> Result<()>;
}

impl ConfigService {
    pub(crate) fn new() -> Self {
        Self { container: None }
    }
}

impl ServiceTrait for ConfigService {}

impl DependencyInjectTrait for ConfigService {
    fn inject(&mut self, container: Arc<DIContainer>) {
        self.container = Some(container);
    }
}

impl ConfigTrait for ConfigService {
    fn get(&self) -> Result<Config> {
        let path = DEFAULT_CONFIG.config_file()?;
        let config = self.container.get::<Config>().unwrap();

        if path.exists() {
            debug!("Getting the config from {:?}", path);

            let f = File::open(&path)?;
            Ok(serde_yaml::from_reader::<File, Config>(f)?)
        } else {
            debug!(
                "Getting the config from runtime, because {:?} does not exists",
                path
            );

            let c = Config {
                log_level: config.log_level.clone(),
                output_format: config.output_format.clone(),
                home_dir: config.home_dir.clone(),
                github_token: config.github_token.clone(),
                github_key: config.github_key.clone(),
            };
            Ok(c)
        }
    }

    fn update(&self, config: &Config) -> Result<()> {
        let path = DEFAULT_CONFIG.config_file()?;

        info!("Updating the config {:?}", path);

        if path.exists() {
            let _ = remove_file(&path);
        }
        let f = File::create(&path)?;

        Ok(serde_yaml::to_writer(f, &config)?)
    }
}
