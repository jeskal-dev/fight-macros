use std::sync::Arc;

use anyhow::{Context, Result};

use crossbeam_channel::bounded;
use tracing::{error, info};

use crate::{
    config::model::StoredMacro,
    engine::{
        executor::Executor,
        queue::{MacroQueue, QueueCommand},
    },
};

#[derive(Debug, Clone)]
pub struct QueueHandler {
    tx: crossbeam_channel::Sender<QueueCommand>,
    shutdown_tx: crossbeam_channel::Sender<()>,
}

impl QueueHandler {
    pub fn new(executor: Arc<Executor>) -> (Self, std::thread::JoinHandle<()>) {
        info!("Iniciando QueueHandler");

        let (tx, rx) = bounded(1024);
        let (shutdown_tx, shutdown_rx) = bounded(1);

        let worker = MacroQueue::new(rx, shutdown_rx, executor);
        let handle = std::thread::Builder::new()
            .name("macro_queue_worker".into())
            .spawn(move || {
                if let Err(e) = worker.run() {
                    error!("Worker de macro queue murió: {e}");
                }
            })
            .expect("No se pudo spawnear el thread del QueueHandler");

        (QueueHandler { tx, shutdown_tx }, handle)
    }

    pub fn push(&self, macro_def: StoredMacro) -> Result<()> {
        info!("Intentando enviar macro: {}", macro_def.name); // ← Agregar este log
        self.tx
            .try_send(QueueCommand::Push(macro_def))
            .context("El canal está lleno o el worker se detuvo")
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());
    }
}
