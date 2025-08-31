use std::sync::OnceLock;

use crossbeam_channel::{unbounded, Receiver, Sender};
use rdev::Event;

static HOTKEY_SENDER: OnceLock<Sender<Event>> = OnceLock::new();
static HOTKEY_RECEIVER: OnceLock<Receiver<Event>> = OnceLock::new();

pub fn init_event_channel() {
    let (tx, rx) = unbounded::<Event>();
    HOTKEY_SENDER.set(tx).expect("El canal ya fue inicializado");
    HOTKEY_RECEIVER
        .set(rx)
        .expect("El canal ya fue inicializado");
}

pub fn get_event_sender() -> &'static Sender<Event> {
    HOTKEY_SENDER.get().expect("Canal no inicializado")
}

pub fn get_event_receiver() -> &'static Receiver<Event> {
    HOTKEY_RECEIVER.get().expect("Canal no inicializado")
}
