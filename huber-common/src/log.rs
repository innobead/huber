use std::str::FromStr;

use log::{info, LevelFilter};

use crate::model::config::Config;

pub struct Logger;

impl Logger {
    pub fn init(config: &Config) -> anyhow::Result<()> {
        match LevelFilter::from_str(&config.log_level.to_uppercase())? {
            LevelFilter::Off => {
                env_logger::builder()
                    .filter_level(LevelFilter::Info)
                    .default_format()
                    .format_target(false)
                    .format_timestamp(None)
                    .try_init()?;
            }
            value => {
                env_logger::builder()
                    .filter_level(value)
                    .default_format()
                    .try_init()?;
            }
        }

        Ok(())
    }
}

pub fn println_many(msg: &str, objs: &[String]) {
    let lines: String = objs.join("\n - ");
    info!("{}:\n - {}", msg, lines.trim_end_matches(" - "));
}
