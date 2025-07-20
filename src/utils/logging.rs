use anyhow::Result;
use env_logger::Builder;

pub struct Logger;

impl Logger {
    pub fn init_logging() -> Result<()> {
        Builder::new()
            .filter_level(log::LevelFilter::Info)
            .format_timestamp_secs()
            .init();

        log::info!("Logging initialized");

        Ok(())
    }
}
