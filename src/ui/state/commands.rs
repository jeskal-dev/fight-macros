use crate::{
    config::{
        handler::{CONFIG, ConfigHandler},
        types::{Macro, MacroAction, Profile},
    },
    ui::state::ui_state::UIState,
};

pub fn sync_config(state: &UIState) {
    let mut cfg = CONFIG.write().unwrap();
    cfg.profiles = state.profiles.clone();
    cfg.active_profile = state.active_profile;
}

// ---------- Profiles ----------
pub fn add_profile(state: &mut UIState) {
    state.profiles.push(Profile::default());
    state.current_profile = Some(state.profiles.len() - 1);
    state.current_macro = None;
}

pub fn remove_profile(state: &mut UIState) {
    let Some(idx) = state.current_profile else {
        return;
    };
    state.active_profile = None;
    // 1.  Eliminar el perfil

    // 2.  Ajustar active_profile sin dejar un índice inválido
    match state.current_profile {
        Some(active) if active == idx => {
            // El perfil activo era el que se borró
            state.current_profile = None;
            state.current_macro = None;
        }
        Some(active) if active > idx => {
            // El perfil activo estaba después: retroceder 1
            state.current_profile = Some(active - 1);
        }
        _ => { /* otro caso: no cambia */ }
    }

    // 3.  Asegurar que current_macro aún sea válido
    if let Some(p_idx) = state.current_profile {
        if state
            .current_macro
            .is_some_and(|m_idx| m_idx >= state.profiles[p_idx].macros.len())
        {
            state.current_macro = None;
        }
    } else {
        state.current_macro = None;
    }

    state.profiles.remove(idx);
}

pub fn duplicate_profile(state: &mut UIState) {
    let Some(idx) = state.current_profile else {
        return;
    };
    let mut clone = state.profiles[idx].clone();
    clone.name.push_str(" (copia)");
    state.profiles.insert(idx + 1, clone);
}

pub fn set_active_profile(state: &mut UIState, idx: usize) {
    state.current_profile = Some(idx);
    state.current_macro = None;
}

pub fn save_changes() {
    let cfg = CONFIG.read().unwrap();
    ConfigHandler::save_config(&cfg).unwrap();
}

// ---------- Macros ----------
pub fn add_macro(state: &mut UIState) {
    let Some(p_idx) = state.current_profile else {
        return;
    };
    state.profiles[p_idx].macros.push(Macro::default());
}

pub fn remove_macro(state: &mut UIState, idx: usize) {
    let Some(p_idx) = state.current_profile else {
        return;
    };
    state.profiles[p_idx].macros.remove(idx);
    if state.current_macro == Some(idx) {
        state.current_macro = None;
    } else if let Some(current) = state.current_macro {
        if current > idx {
            state.current_macro = Some(current - 1);
        }
    }
}

pub fn set_current_macro(state: &mut UIState, idx: usize) {
    state.current_macro = Some(idx);
}

// ---------- Macro Actions ----------
pub fn add_action(state: &mut UIState) {
    let Some(p_idx) = state.current_profile else {
        return;
    };
    let Some(m_idx) = state.current_macro else {
        return;
    };
    state.profiles[p_idx].macros[m_idx]
        .sequence
        .push(MacroAction::default());
}

pub fn remove_action(state: &mut UIState, idx: usize) {
    let Some(p_idx) = state.current_profile else {
        return;
    };
    let Some(m_idx) = state.current_macro else {
        return;
    };
    state.profiles[p_idx].macros[m_idx].sequence.remove(idx);
}

// pub fn start_recording(state: &mut UIState, stop_key: Option<Key>) {
//     let (handle, rx) = record_sequence(stop_key);
//     state.recorder = Some(handle);
//     *state.recording_future.lock().unwrap() = Some(rx);
// }

// pub fn poll_recording_result(ctx: &egui::Context, state: &mut UIState) {
//     let mut guard = state.recording_future.lock().unwrap();
//     let Some(mut rx) = guard.take() else { return };

//     match rx.try_recv() {
//         Ok(seq) => {
//             if let (Some(p_idx), Some(m_idx)) = (state.active_profile, state.current_macro) {
//                 state.profiles[p_idx].macros[m_idx].sequence = seq;
//                  sync_config(state);
//             }
//             state.recorder = None;
//         }
//         Err(oneshot::error::TryRecvError::Empty) => {
//             // Aún no terminó → devolver al estado
//             *guard = Some(rx);
//             ctx.request_repaint(); // seguir comprobando
//         }
//         Err(oneshot::error::TryRecvError::Closed) => {
//             state.recorder = None;
//         }
//     }
// }
