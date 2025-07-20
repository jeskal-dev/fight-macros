use crate::{
    config::types::Macro,
    engine::macro_queue::{MacroQueue, QueueCommand},
};

#[derive(Debug, Clone)]
pub struct QueueHandler {
    tx: tokio::sync::mpsc::UnboundedSender<QueueCommand>,
}

impl QueueHandler {
    pub fn spawn_worker() -> QueueHandler {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let worker = MacroQueue::new(rx);
        tokio::spawn(worker.run());
        QueueHandler { tx }
    }
    pub fn push(&self, macro_def: Macro) {
        let _ = self.tx.send(QueueCommand::Push(macro_def));
    }
}
