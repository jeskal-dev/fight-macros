use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc::{self, UnboundedReceiver};

use crate::{
    config::handler::CONFIG,
    engine::queue_handler::QueueHandler,
    input::keyboard::{HotkeyEvent, KeyboardHandler},
};

pub struct EventProcessor {
    queue: Arc<QueueHandler>,
    event_rx: UnboundedReceiver<HotkeyEvent>,
}

impl EventProcessor {
    pub fn new() -> Result<Self> {
        let queue = Arc::new(QueueHandler::spawn_worker());
        let (tx, rx) = mpsc::unbounded_channel();

        KeyboardHandler::start_listener(tx)?;

        Ok(Self {
            queue,
            event_rx: rx,
        })
    }

    pub async fn process_events(&mut self) -> Result<()> {
        while let Some(ev) = self.event_rx.recv().await {
            self.handle_event(ev)?;
        }
        Ok(())
    }

    fn handle_event(&self, ev: HotkeyEvent) -> Result<()> {
        match ev {
            HotkeyEvent::ComboTriggered(name) => {
                self.handle_macro_trigger(name)?;
            }
            HotkeyEvent::ProfileSwitch(name) => {
                self.handle_profile_switch(name)?;
            }
        }
        Ok(())
    }

    fn handle_macro_trigger(&self, name: String) -> Result<()> {
        let cfg = CONFIG.read().expect("Error al leer configuración global");
        if let Some(macro_def) = cfg.find_macro(&name) {
            self.queue.push(macro_def.clone());
        }
        Ok(())
    }

    fn handle_profile_switch(&self, name: String) -> Result<()> {
        let mut cfg = CONFIG
            .write()
            .expect("Error al guardar configuración global");
        cfg.active_profile = cfg.profiles.iter().position(|p| p.name == name);
        log::info!("Cambiando perfil a: {}", name);
        Ok(())
    }
}
