use anyhow::Result;
use rdev::{simulate, EventType};

pub fn send_event(ev: EventType) -> Result<()> {
    simulate(&ev)?;

    Ok(())
}
