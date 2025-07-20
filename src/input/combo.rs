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
}
