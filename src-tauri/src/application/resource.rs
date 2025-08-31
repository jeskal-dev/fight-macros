use std::sync::Arc;

use anyhow::{Context, Result};
use crossbeam_channel::{bounded, Receiver, Sender};
use tauri::AppHandle;
use tracing::error;

use crate::{
    application::watchdog::WatchdogManager,
    engine::{executor::Executor, handler::QueueHandler, processor::EventProcessor},
};

#[derive(Debug)]
pub struct ResourceManager {
    processor: Option<Arc<EventProcessor>>,
    executor: Arc<Executor>,
    queue_handler: Option<QueueHandler>,
    shutdown_tx: Sender<()>,
    shutdown_rx: Receiver<()>,
    pub watchdogs: WatchdogManager,
    threads: Vec<std::thread::JoinHandle<()>>,
}

impl ResourceManager {
    pub fn new(executor: Arc<Executor>) -> Result<Self> {
        let (shutdown_tx, shutdown_rx) = bounded(1);

        Ok(Self {
            processor: None,
            executor,
            queue_handler: None,
            shutdown_tx,
            shutdown_rx,
            watchdogs: WatchdogManager::new(),
            threads: Vec::new(),
        })
    }

    pub fn init_event_processor(&mut self, app_handle: AppHandle) -> Result<()> {
        let queue_handler = self
            .queue_handler
            .clone()
            .context("QueueHandler no inicializado")?;

        let processor = EventProcessor::new(queue_handler, app_handle.clone())?;

        // Spawn event loop thread
        let processor_clone = processor.clone();
        let shutdown_rx = self.shutdown_rx.clone();
        let event_loop_thread = std::thread::Builder::new()
            .name("processor_event_loop".into())
            .spawn(move || {
                if let Err(e) = processor_clone.process_events(shutdown_rx) {
                    error!("Error en event loop: {:?}", e);
                }
            })
            .context("Failed to spawn event loop thread")?;

        self.threads.push(event_loop_thread);
        self.processor = Some(processor);

        Ok(())
    }

    pub fn init_queue_handler(&mut self) -> Result<()> {
        let (queue_handler, handle) = QueueHandler::new(self.executor.clone());
        self.queue_handler = Some(queue_handler);
        self.threads.push(handle);
        Ok(())
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());
        self.executor.shutdown();
        self.watchdogs.shutdown();
        if let Some(queue_handler) = &self.queue_handler {
            queue_handler.shutdown();
        }
    }

    pub fn wait_for_completion(&mut self) {
        for thread in self.threads.drain(..) {
            if let Err(e) = thread.join() {
                error!("Error esperando por thread: {:?}", e);
            }
        }
        self.watchdogs.wait_for_completion();
    }
}

impl Drop for ResourceManager {
    fn drop(&mut self) {
        self.shutdown();
        self.wait_for_completion();
    }
}
