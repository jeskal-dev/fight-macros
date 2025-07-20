use anyhow::Result;
use tokio_util::sync::CancellationToken;

use crate::engine::processor::EventProcessor;

#[derive(Debug, Clone)]
pub struct LifecycleManager {
    shutdown_token: CancellationToken,
}

impl LifecycleManager {
    pub fn new() -> Self {
        Self {
            shutdown_token: CancellationToken::new(),
        }
    }

    pub async fn run(&self, mut event_processor: EventProcessor) -> Result<()> {
        log::info!("Iniciando Fight Macro App");

        let shutdown = self.shutdown_token.clone();
        tokio::spawn(async move {
            shutdown.cancelled().await;
            // TODO: Cleanup logic
        });

        // Main event loop
        tokio::select! {
            _ = self.shutdown_token.cancelled() => {
                log::info!("Shutdown signal received");
            }
            result = event_processor.process_events() => {
                if let Err(e) = result {
                    log::error!("Error processing events: {}", e);
                }
            }
        }

        self.shutdown().await;
        Ok(())
    }

    async fn shutdown(&self) {
        log::info!("Shutting down gracefully...");
        // TODO: Additional cleanup logic
    }
}
