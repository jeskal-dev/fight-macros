use anyhow::Result;
use rdev::{EventType, simulate};

pub fn send_event(ev: EventType) -> Result<()> {
    simulate(&ev)?;
    Ok(())
}
