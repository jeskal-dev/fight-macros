mod config;
mod engine;
mod input;
mod utils;

use anyhow::{Ok, Result};
use tokio::sync::mpsc;

use crate::{
    config::handler::{CONFIG, init_global_config},
    engine::queue::spawn_worker,
    input::keyboard::{self, HotkeyEvent},
    utils::logging,
};

#[tokio::main]
async fn main() -> Result<()> {
    logging::init_logging()?;

    init_global_config().await?;
    let queue = spawn_worker();
    let (tx, mut rx) = mpsc::unbounded_channel();

    keyboard::start_listener(tx)?;

    log::info!("Iniciando Fight Macro App");

    // window::build_ui(CONFIG.read().await.clone())?;

    while let Some(ev) = rx.recv().await {
        match ev {
            HotkeyEvent::ComboTriggered(name) => {
                let cfg = CONFIG.read().await;
                if let Some(macro_def) = cfg.find_macro(&name) {
                    queue.push(macro_def.clone());
                }
            }
            HotkeyEvent::ProfileSwitch(name) => {
                let mut cfg = CONFIG.write().await;
                cfg.active_profile = name.clone();
                log::info!("Cambiando perfil a: {}", name);
            }
        }
    }

    Ok(())
}
