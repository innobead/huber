use std::fs::{remove_file, File};
use std::sync::Arc;

use anyhow::anyhow;
use huber_common::model::config::{Config, ConfigPath};
use lazy_static::lazy_static;
use log::debug;
use simpledi_rs::di::{DIContainer, DIContainerExtTrait, DependencyInjectTrait};

use crate::error::HuberError;
use crate::service::ServiceTrait;

lazy_static! {
    pub static ref DEFAULT_CONFIG: Config = Default::default();
}

#[derive(Debug)]
pub struct ConfigService {
    pub container: Option<Arc<DIContainer>>,
}

unsafe impl Send for ConfigService {}

unsafe impl Sync for ConfigService {}

pub trait ConfigTrait {
    fn get(&self) -> anyhow::Result<Config>;
    fn update(&self, config: &Config) -> anyhow::Result<()>;
}

impl Default for ConfigService {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigService {
    pub fn new() -> Self {
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
    fn get(&self) -> anyhow::Result<Config> {
        let config = self.container.get::<Config>().expect("no config found");
        let config_path = config.config_file()?;

        if config_path.exists() {
            debug!("Getting the config from {:?}", config_path);

            let f = File::open(&config_path)?;
            return Ok(serde_yaml::from_reader::<File, Config>(f)?);
        }

        Err(anyhow!(HuberError::ConfigNotFound(
            config_path.to_string_lossy().to_string()
        )))
    }

    fn update(&self, config: &Config) -> anyhow::Result<()> {
        let path = config.config_file()?;

        debug!("Updating the config {:?}: {:?}", path, config);
        if path.exists() {
            let _ = remove_file(&path);
        }
        let f = File::create(&path)?;
        serde_yaml::to_writer(f, &config)?;

        Ok(())
    }
}
