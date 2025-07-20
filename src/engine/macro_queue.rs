use tokio::{sync::mpsc::UnboundedReceiver, task::JoinHandle};

use crate::{config::types::Macro, engine::executor::Executor};

#[derive(Debug)]
pub enum QueueCommand {
    Push(Macro),
}

pub struct MacroQueue {
    rx: UnboundedReceiver<QueueCommand>,
    current_task: Option<JoinHandle<()>>,
}

impl MacroQueue {
    pub fn new(rx: UnboundedReceiver<QueueCommand>) -> Self {
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
                        if let Err(e) = Executor::run_sequence(&macro_def.sequence).await {
                            log::error!("Error al ejecutar macro {}: {}", macro_def.name, e);
                        }
                    });
                    self.current_task = Some(task);
                }
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
