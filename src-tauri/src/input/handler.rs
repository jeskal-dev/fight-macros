use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use crate::{
    config::handler::get_config,
    input::{
        global::{get_event_receiver, get_event_sender},
        state::{is_combo_completed, KeyState},
    },
    keys,
};
use anyhow::Result;
use crossbeam_channel::Sender;
use parking_lot::Mutex;

use rdev::{Event, EventType};
use tracing::{debug, error, info, warn};
#[cfg(windows)]
use winapi::um::processthreadsapi::{GetCurrentThread, SetThreadPriority};

type SharedState = Arc<Mutex<KeyState>>;

#[derive(Debug)]
pub enum HotkeyEvent {
    ComboTriggered(u64),
    ProfileSwitch(u64),
}

#[derive(Debug)]
pub struct KeyboardHandler {
    state: SharedState,
    tx_event: Option<Sender<HotkeyEvent>>,
    threads: Vec<JoinHandle<()>>,
}

impl KeyboardHandler {
    pub fn new() -> Self {
        info!("Creando handler de teclado");
        let state: SharedState = Arc::new(Mutex::new(KeyState::default()));
        Self {
            state,
            tx_event: None,
            threads: Vec::new(),
        }
    }

    pub fn start_listener(
        &mut self,
        shutdown_rx: crossbeam_channel::Receiver<()>,
    ) -> Result<crossbeam_channel::Receiver<HotkeyEvent>> {
        info!("Iniciando listener de teclado con scoped threads...");

        let (tx_event, rx_event) = crossbeam_channel::unbounded::<HotkeyEvent>();
        self.tx_event = Some(tx_event.clone());
        let state = Arc::clone(&self.state);

        let handle = thread::Builder::new()
            .name("keyboard_listener".to_string())
            .spawn(move || {
                #[cfg(windows)]
                unsafe {
                    let thread_handle = GetCurrentThread();
                    SetThreadPriority(thread_handle, 2);
                }
                let tx_raw = tx_event.clone();

                crossbeam::scope(|s| {
                    // Canal interno para eventos del teclado
                    let internal_tx = get_event_sender();
                    let internal_rx = get_event_receiver();

                    // Hilo para el listener de teclado (scoped)
                    s.spawn(move |_| {
                        if let Err(e) = rdev::grab(move |ev: Event| {
                            if let EventType::KeyPress(_) = ev.event_type {
                                if let Err(e) = internal_tx.send(ev.clone()) {
                                    error!(error = ?e, "Error enviando evento interno");
                                    return Some(ev);
                                }
                            }

                            Some(ev)
                        }) {
                            error!(error = ?e, "Error en keyhook");
                        }
                    });

                    // Hilo principal de procesamiento (scoped)
                    s.spawn(move |_| loop {
                        crossbeam::select! {
                            recv(internal_rx) -> msg => {
                                match msg {
                                    Ok(ev) => {
                                        let mut st = state.lock();
                                        st.update(&ev);

                                        if let Err(e) = Self::process_event(&tx_raw, &ev, &st) {
                                            error!(error = %e, "Error procesando evento");
                                        }
                                    }
                                    Err(_) => {
                                        info!("Canal interno cerrado, saliendo del procesador");
                                        break;
                                    }
                                }
                            }
                            recv(shutdown_rx) -> _ => {
                                info!("Shutdown recibido, deteniendo listener de teclado");
                                break;
                            }
                        }
                    });
                })
                .unwrap();

                info!("Listener de teclado terminado.");
            })?;

        self.threads.push(handle);
        Ok(rx_event)
    }

    fn process_event(
        tx: &crossbeam_channel::Sender<HotkeyEvent>,
        ev: &Event,
        st: &KeyState,
    ) -> anyhow::Result<()> {
        let span = tracing::debug_span!("process_event", ?ev);
        let _enter = span.enter();

        let EventType::KeyPress(key) = ev.event_type else {
            return Ok(());
        };

        let cfg = get_config();

        for (profile_id, profile) in &cfg.profiles {
            if let Some(function_key) = &profile.function_key {
                let switch_key = keys::str_to_key(function_key);
                // trace!(
                //     ?profile_id,
                //     ?function_key,
                //     ?switch_key,
                //     "Verificando cambio de perfil"
                // );
                if key == switch_key {
                    info!(?profile_id, "Cambio de perfil detectado");
                    tx.send(HotkeyEvent::ProfileSwitch(profile.id))?;
                    return Ok(());
                }
            }
        }

        if let Some(active) = cfg.get_active_profile() {
            debug!(profile_id = active.id, "Perfil activo detectado");

            let mut macros = active
                .macro_ids
                .iter()
                .filter_map(|id| cfg.macros.get(id))
                .collect::<Vec<_>>();
            macros.sort_by_key(|m| -(m.trigger.modifiers.len() as i32));

            for mac in macros {
                let span = tracing::debug_span!("check_macro", macro_id = mac.id);
                let _enter = span.enter();

                // trace!("Verificando macro: {:?}", mac.trigger);

                if keys::key_matches(&mac.trigger.key, key) && is_combo_completed(st, &mac.trigger)
                {
                    info!(macro_id = mac.id, "Macro activada");
                    if let Err(e) = tx.send(HotkeyEvent::ComboTriggered(mac.id)) {
                        warn!("Error enviando evento de macro: {:?}", e);
                    }
                    return Ok(());
                }
            }
        }

        // trace!("Ningún macro o acción coincide con la tecla presionada");
        Ok(())
    }

    pub fn wait_for_completion(&mut self) {
        for thread in self.threads.drain(..) {
            if let Err(e) = thread.join() {
                error!("Error esperando por thread: {:?}", e);
            }
        }
    }
}
