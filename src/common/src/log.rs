use crate::config::Config;
use crate::result::Result;

pub struct Logger;

impl Logger {
    pub fn init(config: &Config) -> Result<()> {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .level(config.log_level.to_level_filter())
            .chain(std::io::stdout())
            .apply()?;

        Ok(())
    }
}
