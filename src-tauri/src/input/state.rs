use ahash::AHashSet;
use rdev::EventType;

use crate::{domain::macros::KeyCombination, input::state, keys};

#[derive(Debug, Default)]
pub struct KeyState {
    pressed: AHashSet<rdev::Key>,
}

impl KeyState {
    pub fn update(&mut self, ev: &rdev::Event) {
        match ev.event_type {
            EventType::KeyPress(k) => {
                self.pressed.insert(k);
            }
            EventType::KeyRelease(k) => {
                self.pressed.remove(&k);
            }
            _ => {}
        }
    }
}

pub(crate) fn is_combo_completed(state: &state::KeyState, trigger: &KeyCombination) -> bool {
    let base = keys::str_to_key(&trigger.key);
    if !state.pressed.contains(&base) {
        return false;
    }

    trigger.modifiers.iter().all(|m| {
        let k = keys::mod_to_key(m);
        state.pressed.contains(&k)
    })
}
