use anyhow::Result;
use tracing::{level_filters::LevelFilter, Level};

use crate::application::launcher::AppLauncher;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer};
mod application;
mod config;
mod domain;
mod engine;
mod input;
mod keys;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_ansi(true)
                .with_level(true)
                .with_writer(std::io::stdout)
                .pretty()
                .with_filter(LevelFilter::from_level(Level::ERROR)),
        )
        .init();

    AppLauncher::launch()
}
