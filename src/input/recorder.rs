use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

use rdev::Key;
use tokio::sync::{mpsc, oneshot};

use crate::config::types::MacroAction;

#[derive(Debug, Clone)]
pub struct RecorderHandler {
    is_recording: Arc<AtomicBool>,
    stop_sender: mpsc::UnboundedSender<()>,
}

impl RecorderHandler {
    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::Relaxed)
    }
    pub fn stop(&self) {
        let _ = self.stop_sender.send(());
    }
}

pub fn record_sequence(
    stop_key: Option<Key>,
) -> (RecorderHandler, oneshot::Receiver<Vec<MacroAction>>) {
    let stop_key = stop_key.unwrap_or(Key::Escape);

    let (tx_result, rx_result) = oneshot::channel();
    let (tx_stop, mut rx_stop) = mpsc::unbounded_channel();

    let is_recording = Arc::new(AtomicBool::new(true));

    let handle = RecorderHandler {
        is_recording: is_recording.clone(),
        stop_sender: tx_stop,
    };

    thread::spawn(move || {
        use rdev::{Event, EventType, listen};
        use std::collections::HashSet;
        use std::time::{Duration, Instant};
        let mut tx_result = Some(tx_result);
        let mut seq = Vec::new();
        let mut last_time = Instant::now();
        let mut pressed = HashSet::new();
        let mut last_stop_press = None;

        // Envoltorio de `listen` que termina cuando recibe señal
        let _ = listen(move |ev: Event| {
            // 1) Comprobar señal externa (botón, etc.)
            if let Ok(()) = rx_stop.try_recv() {
                is_recording.store(false, Ordering::Relaxed);
                if let Some(tx) = tx_result.take() {
                    let _ = tx.send(seq.clone());
                }
                return;
            }

            let now = Instant::now();
            let delta_ms = now.duration_since(last_time).as_secs_f64() * 1000.0;
            if delta_ms > 0.0 && !seq.is_empty() {
                seq.push(MacroAction::Delay { ms: delta_ms });
            }
            last_time = now;

            match ev.event_type {
                EventType::KeyPress(k) => {
                    if k == stop_key {
                        match last_stop_press {
                            Some(prev) if now.duration_since(prev) < Duration::from_millis(500) => {
                                is_recording.store(false, Ordering::Relaxed);
                                if let Some(tx) = tx_result.take() {
                                    let _ = tx.send(seq.clone());
                                }
                                return;
                            }
                            _ => last_stop_press = Some(now),
                        }
                    }

                    if pressed.insert(k) {
                        seq.push(MacroAction::KeyDown {
                            key: format!("{:?}", k),
                        });
                    }
                }
                EventType::KeyRelease(k) => {
                    pressed.remove(&k);
                    seq.push(MacroAction::KeyUp {
                        key: format!("{:?}", k),
                    });
                }
                _ => {}
            }
        });

        if let Some(tx) = tx_result.take() {
            let _ = tx.send(seq);
        }
    });

    (handle, rx_result)
}
