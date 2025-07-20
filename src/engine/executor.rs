use std::time::Duration;

use anyhow::Result;
use tokio::time::sleep;

use crate::{config::types::MacroAction, engine::mapper::send_event, utils::helpers};

pub struct Executor;

impl Executor {
    pub async fn execute(action: &MacroAction) -> Result<()> {
        match action {
            MacroAction::KeyDown { key } => {
                log::info!("Pulsando [{}]", key);
                let k = helpers::str_to_key(key);
                send_event(rdev::EventType::KeyPress(k))?;
            }
            MacroAction::KeyUp { key } => {
                log::info!("Soltando [{}]", key);
                let k = helpers::str_to_key(key);
                send_event(rdev::EventType::KeyRelease(k))?;
            }
            MacroAction::Delay { ms } => {
                log::info!("Pausando [{}] ms", ms);
                sleep(Duration::from_secs_f64(ms / 1000.0)).await;
            }
        }

        Ok(())
    }

    pub async fn run_sequence(actions: &[MacroAction]) -> Result<()> {
        for action in actions {
            Executor::execute(action).await?;
        }
        Ok(())
    }
}
