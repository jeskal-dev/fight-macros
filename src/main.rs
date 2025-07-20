mod app;
mod config;
mod engine;
mod input;
mod utils;

use anyhow::Result;

use crate::{app::FightMacrosApp, utils::logging::Logger};

#[tokio::main]
async fn main() -> Result<()> {
    Logger::init_logging()?;

    let app = FightMacrosApp::new().await?;

    app.run().await
}
