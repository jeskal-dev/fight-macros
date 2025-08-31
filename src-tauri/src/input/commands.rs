use rdev::{Event, EventType};

use crate::keys;

#[tauri::command]
pub fn send_keydown_event(key: String) -> Result<(), String> {
    // Convertir el string a Key
    let rdev_key = keys::str_to_key(&key);

    // Crear solo el EventType necesario
    let event_type = EventType::KeyPress(rdev_key);

    // Crear el evento sin el campo name problemático
    let event = Event {
        event_type,
        time: std::time::SystemTime::now(),
        name: None, // O manejar correctamente el lifetime
    };

    let tx = super::global::get_event_sender();
    tx.send(event).map_err(|e| e.to_string())
}
