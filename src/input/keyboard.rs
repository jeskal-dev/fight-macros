use anyhow::Result;
use once_cell::sync::Lazy;
use rdev::{EventType, listen};
use std::sync::Mutex;
use tokio::sync::mpsc;

use crate::{config::types::Config, input::combo::KeyState, utils::helpers};

static KEY_STATE: Lazy<Mutex<KeyState>> = Lazy::new(|| Mutex::new(Default::default()));

#[derive(Debug)]
pub enum HotkeyEvent {
    ComboTriggered(String),
    ProfileSwitch(String),
}

pub fn start_listener(tx: mpsc::UnboundedSender<HotkeyEvent>) -> Result<()> {
    std::thread::spawn(move || {
        if let Err(e) = listen(move |ev| {
            // 1) actualizar estado
            if let Ok(mut state) = KEY_STATE.lock() {
                state.update(&ev);
            }

            if let Ok(cfg) = crate::config::handler::CONFIG.try_read() {
                if let Ok(state) = KEY_STATE.try_lock() {
                    check_triggers(&tx, &cfg, &state, &ev);
                }
            }
        }) {
            log::error!("Error en keyhook: {:?}", e);
        }
    });
    Ok(())
}

fn check_triggers(
    tx: &mpsc::UnboundedSender<HotkeyEvent>,
    cfg: &Config, // Suponiendo que tienes un tipo Config definido
    state: &KeyState,
    ev: &rdev::Event,
) {
    let EventType::KeyPress(key) = ev.event_type else {
        return;
    };

    // Verificar cambio de perfil
    for (name, profile) in &cfg.profiles {
        let switch_key = helpers::str_to_key(profile.switch_key.as_str());

        if key == switch_key {
            if let Err(e) = tx.send(HotkeyEvent::ProfileSwitch(name.clone())) {
                log::warn!("Error enviando evento de perfil: {:?}", e);
            }
            return;
        }
    }

    // Verificar macros del perfil activo
    if let Some(active) = cfg.get_active_profile() {
        let mut macros_sorted = active.macros.iter().collect::<Vec<_>>();
        macros_sorted.sort_by_key(|mac| -(mac.trigger.modifiers.len() as i32));
        for mac in macros_sorted {
            if helpers::key_matches(&mac.trigger.key, key)
                && helpers::is_combo_completed(&mac.trigger, state)
            {
                if let Err(e) = tx.send(HotkeyEvent::ComboTriggered(mac.name.clone())) {
                    log::warn!("Error enviando evento de macro: {:?}", e);
                }
                return;
            }
        }
    }
}
