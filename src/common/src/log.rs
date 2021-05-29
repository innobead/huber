use std::str::FromStr;

use log::Level;

use crate::model::config::Config;
use crate::result::Result;

pub struct Logger;

impl Logger {
    pub fn init(config: &Config) -> Result<()> {
        pretty_env_logger::formatted_timed_builder()
            .filter_level(Level::from_str(&config.log_level)?.to_level_filter())
            .try_init()?;
        Ok(())
    }
}

pub fn println_many(msg: &str, objs: &[String]) {
    let lines: String = objs.join("\n -");
    println!("{}:\n - {}", msg, lines.trim_end_matches(" -"));
}
