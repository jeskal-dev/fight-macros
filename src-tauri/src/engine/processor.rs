use std::sync::Arc;

use anyhow::{Context, Result};
use crossbeam_channel::Receiver;
use parking_lot::Mutex;
use tauri::{AppHandle, Emitter};
use tracing::{error, info, warn};

use crate::{
    config::handler::{get_config, save_config},
    domain::enums::ListenableChannel,
    engine::handler::QueueHandler,
    input::handler::{HotkeyEvent, KeyboardHandler},
};

#[derive(Debug)]
pub struct EventProcessor {
    queue_handler: QueueHandler,
    keyboard_handler: Mutex<KeyboardHandler>, // ← Mutex aquí
    app_handle: AppHandle,
}

impl EventProcessor {
    pub fn new(queue_handler: QueueHandler, app_handle: AppHandle) -> Result<Arc<Self>> {
        info!("Iniciando procesador de eventos");
        Ok(Arc::new(Self {
            queue_handler,
            keyboard_handler: Mutex::new(KeyboardHandler::new()),
            app_handle,
        }))
    }

    pub fn process_events(self: Arc<Self>, shutdown_rx: Receiver<()>) -> Result<()> {
        info!("Procesando eventos");

        let event_rx = {
            let mut kbd = self.keyboard_handler.lock();
            kbd.start_listener(shutdown_rx.clone())?
        };

        loop {
            crossbeam_channel::select! {
                recv(event_rx) -> ev => match ev {
                    Ok(ev) => {
                        if let Err(e) = self.handle_event(ev) {
                            error!("Error al procesar evento: {e:?}");
                        }
                    }
                    Err(err) => {
                        warn!("Señal de shutdown recibida del KeyboardHandler, terminando worker, {err}");
                         break;
                    }
                },
                recv(shutdown_rx) -> _ => {
                    warn!("Señal de shutdown recibida del general");
                    break;
                }
            }
        }
        Ok(())
    }

    fn handle_event(&self, ev: HotkeyEvent) -> Result<()> {
        info!("Evento recibido: {:?}", ev);
        match ev {
            HotkeyEvent::ComboTriggered(id) => {
                self.handle_macro_trigger(&id)?;
            }
            HotkeyEvent::ProfileSwitch(id) => {
                self.handle_profile_switch(&id)?;
            }
        }

        Ok(())
    }

    fn handle_macro_trigger(&self, id: &u64) -> Result<()> {
        info!("Buscando macro con ID: {}", id);
        let cfg = get_config();
        if let Some(macro_def) = cfg.find_macro(id) {
            info!("Macro encontrada: {}", macro_def.name); // ← Agregar este log
            self.queue_handler.push(macro_def.clone())?;
        } else {
            warn!("No se encontró macro con ID: {}", id); // ← Agregar este log
        }
        Ok(())
    }

    fn handle_profile_switch(&self, id: &u64) -> Result<()> {
        info!("Cambiando perfil a: {}", id);
        let cfg = get_config();

        save_config(|config| {
            if let Some(profile) = cfg.find_profile(id).cloned() {
                config.selected_profile_id = Some(profile.id);
                info!("Perfil cambiado a: {}", id);
            } else {
                config.selected_profile_id = None;
                warn!("Perfil {} no encontrado", id);
            }
        })
        .context("Error al guardar config")?;

        info!("Cambiado perfil a: {}", id);

        {
            let channel = &ListenableChannel::SelectedProfileChanged.to_string();
            if let Err(e) = self.app_handle.emit(channel, id) {
                error!("Error al emitir evento: {e}");
            }
        }

        Ok(())
    }
}

impl Drop for EventProcessor {
    fn drop(&mut self) {
        self.keyboard_handler.lock().wait_for_completion();
    }
}
