use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use anyhow::Result;
use crossbeam_channel::{bounded, Receiver, Sender};
use parking_lot::Mutex;
use rdev::EventType;
use tracing::info;

use crate::{domain::sequence_step::SequenceStep, engine::event::send_event, keys};

#[derive(Debug)]
pub struct Executor {
    active: AtomicBool,
    shutdown_tx: Sender<()>,
    shutdown_rx: Receiver<()>,
    current_sequence: Mutex<Option<Arc<Vec<SequenceStep>>>>,
}

impl Executor {
    pub fn new() -> Result<Self> {
        let (shutdown_tx, shutdown_rx) = bounded(1);
        Ok(Self {
            active: AtomicBool::new(true),
            shutdown_rx,
            shutdown_tx,
            current_sequence: Mutex::new(None),
        })
    }
    #[inline]
    pub fn execute(&self, action: &SequenceStep) -> Result<()> {
        match action {
            SequenceStep::KeyDown { key, .. } => {
                info!("Pulsando [{}]", key);
                let k = keys::str_to_key(key);
                send_event(EventType::KeyPress(k))?;
            }
            SequenceStep::KeyUp { key, .. } => {
                info!("Soltando [{}]", key);
                let k = keys::str_to_key(key);
                send_event(EventType::KeyRelease(k))?;
            }
            SequenceStep::Delay { ms, .. } => {
                info!("Pausando [{}] ms", ms);

                let (tx, rx) = bounded(1);

                let shutdown_rx = self.shutdown_rx.clone();

                let wait = *ms;
                std::thread::spawn(move || {
                    sleep(Duration::from_millis(wait));
                    let _ = tx.send(());
                });

                crossbeam::select! {
                    recv(rx) -> _ => (),
                    recv(shutdown_rx) -> _ => {
                        info!("Delay interrupted by shutdown");
                        return Ok(());
                    }
                }
            }
        }

        Ok(())
    }

    #[inline]
    pub fn run_sequence(&self, sequence: &[SequenceStep]) -> Result<()> {
        info!("Starting sequence execution - {} steps", sequence.len());
        if !self.active.load(Ordering::SeqCst) {
            return Err(anyhow::anyhow!("Executor is shutting down"));
        }

        let sequence_arc = Arc::new(sequence.to_vec());
        *self.current_sequence.lock() = Some(sequence_arc.clone());

        for (i, step) in sequence.iter().enumerate() {
            info!("Executing step {}: {:?}", i, step);
            if !self.active.load(Ordering::SeqCst) {
                break;
            }
            self.execute(step)?;
        }

        *self.current_sequence.lock() = None;

        info!("Sequence execution completed");
        Ok(())
    }

    pub fn shutdown(&self) {
        self.active.store(false, Ordering::SeqCst);
        let _ = self.shutdown_tx.send(());

        if let Some(sequence) = self.current_sequence.lock().take() {
            info!("Interrupting ongoing sequence of {} steps", sequence.len());
        }
    }
}
