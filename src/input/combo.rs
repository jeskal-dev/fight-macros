use std::collections::HashSet;

use rdev::{EventType, Key};

#[derive(Debug, Default)]
pub struct KeyState {
    pressed: HashSet<Key>,
}

impl KeyState {
    pub fn pressed(&self) -> &HashSet<Key> {
        &self.pressed
    }

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

    // pub fn matches(&self, combo: &KeyCombination) -> bool {
    //     let base: Key = helpers::str_to_key(&combo.key);
    //     if !self.pressed.contains(&base) {
    //         return false;
    //     }

    //     for m in &combo.modifiers {
    //         let k = helpers::mod_to_key(m);
    //         if !self.pressed.contains(&k) {
    //             return false;
    //         }
    //     }

    //     true
    // }
}
