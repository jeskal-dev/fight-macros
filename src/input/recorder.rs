use std::collections::HashSet;

use rdev::{EventType, Key, listen};
use tokio::sync::oneshot;

use crate::config::types::{KeyCombination, ModifierKey};

pub fn record_combo() -> oneshot::Receiver<KeyCombination> {
    let (tx, rx) = oneshot::channel();
    std::thread::spawn(move || {
        let mut pressed: std::collections::HashSet<Key> = Default::default();
        let _ = listen(move |ev| {
            match ev.event_type {
                EventType::KeyPress(k) => {
                    pressed.insert(k);
                }
                EventType::KeyRelease(k) => {
                    if pressed.len() > 1 {
                        // el usuario acaba de soltar la última tecla => fin
                        let combo = KeyCombination {
                            modifiers: pressed
                                .iter()
                                .filter(|k| {
                                    matches!(
                                        k,
                                        Key::ShiftLeft
                                            | Key::ShiftRight
                                            | Key::ControlLeft
                                            | Key::ControlRight
                                            | Key::Alt
                                            | Key::MetaLeft
                                    )
                                })
                                .map(|k| match k {
                                    Key::ShiftLeft | Key::ShiftRight => ModifierKey::Shift,
                                    Key::ControlLeft | Key::ControlRight => ModifierKey::Ctrl,
                                    Key::Alt => ModifierKey::Alt,
                                    Key::MetaLeft | Key::MetaRight => ModifierKey::Meta,
                                    _ => unreachable!(),
                                })
                                .collect(),
                            key: format!(
                                "{:?}",
                                pressed
                                    .iter()
                                    .find(|k| !matches!(
                                        k,
                                        Key::ShiftLeft
                                            | Key::ShiftRight
                                            | Key::ControlLeft
                                            | Key::ControlRight
                                            | Key::Alt
                                            | Key::MetaLeft
                                            | Key::MetaRight
                                    ))
                                    .unwrap_or(&Key::Unknown(0))
                            ),
                        };
                        let _ = tx.send(combo);
                        return;
                    }
                }
                _ => {}
            }
        });
    });
    rx
}
