use anyhow::Result;

use crate::{config::types::MacroAction, engine::mapper::send_event, utils::helpers};

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
            tokio::time::sleep(tokio::time::Duration::from_secs_f64(ms / 1000.0)).await;
        }
    }

    Ok(())
}

pub async fn run_sequence(actions: &[MacroAction]) -> Result<()> {
    for action in actions {
        execute(action).await?;
    }
    Ok(())
}
