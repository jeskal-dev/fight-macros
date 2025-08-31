use std::sync::Arc;

use crate::{
    config::{handler::get_config, model::StoredMacro},
    engine::executor::Executor,
};
use anyhow::Result;
use threadpool::ThreadPool;
use tracing::{error, info};

#[derive(Debug)]
pub enum QueueCommand {
    Push(StoredMacro),
}

pub struct MacroQueue {
    rx: crossbeam_channel::Receiver<QueueCommand>,
    shutdown_rx: crossbeam_channel::Receiver<()>,
    pool: ThreadPool,
    executor: Arc<Executor>,
}

impl MacroQueue {
    pub fn new(
        rx: crossbeam_channel::Receiver<QueueCommand>,
        shutdown_rx: crossbeam_channel::Receiver<()>,
        executor: Arc<Executor>,
    ) -> Self {
        info!("Creando cola de macros");
        let pool = ThreadPool::new(num_cpus::get());
        Self {
            rx,
            pool,
            shutdown_rx,
            executor,
        }
    }

    pub fn run(&self) -> Result<()> {
        info!("Iniciando cola de macros");
        loop {
            crossbeam_channel::select! {
                recv(self.rx) -> msg => {
                    match msg {
                        Ok(QueueCommand::Push(m)) => {
                            info!("Macro recibida en cola: {}", m.name);
                            let config = get_config();
                            let executor = self.executor.clone();

                            self.pool.execute(move || {
                                let sequence = m
                                    .sequence_step_ids
                                    .iter()
                                    .filter_map(|s| config.steps.get(s).cloned())
                                    .collect::<Vec<_>>();

                                if let Err(e) = executor.run_sequence(&sequence) {
                                    error!("Error al ejecutar macro {}: {}", m.name, e)
                                }
                            });
                        },
                        Err(_) => {
                            info!("Todos los senders cerrados, terminando.");
                            break;
                        },
                    }
                },
                recv(self.shutdown_rx) -> _ => {
                    info!("Señal de shutdown recibida, terminando worker");
                    break;
                },
            }
        }

        self.pool.join();
        Ok(())
    }
}
