use crate::{config::types::Macro, engine::executor};

#[derive(Debug)]
pub enum QueueCommand {
    Push(Macro),
    // Clear,
    // Shutdown,
}

pub struct MacroQueue {
    rx: tokio::sync::mpsc::UnboundedReceiver<QueueCommand>,
    current_task: Option<tokio::task::JoinHandle<()>>,
}

impl MacroQueue {
    pub fn new(rx: tokio::sync::mpsc::UnboundedReceiver<QueueCommand>) -> Self {
        Self {
            rx,
            current_task: None,
        }
    }

    pub async fn run(mut self) {
        while let Some(cmd) = self.rx.recv().await {
            match cmd {
                QueueCommand::Push(macro_def) => {
                    self.cancel_current_task();
                    log::info!("Ejecutando macro: {}", macro_def.name);
                    let task = tokio::spawn(async move {
                        if let Err(e) = executor::run_sequence(&macro_def.sequence).await {
                            log::error!("Error al ejecutar macro {}: {}", macro_def.name, e);
                        }
                    });
                    self.current_task = Some(task);
                } // QueueCommand::Clear => {
                  //     log::debug!("Cola limpiada");
                  // }
                  // QueueCommand::Shutdown => break,
            }
        }
    }

    fn cancel_current_task(&mut self) {
        if let Some(task) = self.current_task.take() {
            task.abort();
            log::info!("Macro anterior cancelada");
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueueHandler {
    tx: tokio::sync::mpsc::UnboundedSender<QueueCommand>,
}

impl QueueHandler {
    pub fn push(&self, macro_def: Macro) {
        let _ = self.tx.send(QueueCommand::Push(macro_def));
    }
    // pub fn clear(&self) {
    //     let _ = self.tx.send(QueueCommand::Clear);
    // }
    // pub fn shutdown(&self) {
    //     let _ = self.tx.send(QueueCommand::Shutdown);
    // }
}

pub fn spawn_worker() -> QueueHandler {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let worker = MacroQueue::new(rx);
    tokio::spawn(worker.run());
    QueueHandler { tx }
}
