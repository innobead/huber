use crate::config::Config;
use crate::result::Result;

pub struct Logger;

impl Logger {
    pub fn init(config: &Config) -> Result<()> {
        pretty_env_logger::formatted_timed_builder()
            .filter_level(config.log_level.to_level_filter())
            .try_init()?;
        Ok(())
    }
}
