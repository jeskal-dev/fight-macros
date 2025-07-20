use anyhow::Result;

use crate::{
    app::lifecycle::LifecycleManager, config::handler::ConfigHandler,
    engine::processor::EventProcessor,
};

mod lifecycle;

pub struct FightMacrosApp {
    event_processor: EventProcessor,
    lifecycle_manager: LifecycleManager,
}

impl FightMacrosApp {
    pub async fn new() -> Result<Self> {
        ConfigHandler::init_global_config().await?;

        let event_processor = EventProcessor::new().await?;
        let lifecycle_manager = LifecycleManager::new();

        Ok(Self {
            event_processor,
            lifecycle_manager,
        })
    }

    pub async fn run(self) -> Result<()> {
        self.lifecycle_manager.run(self.event_processor).await
    }
}
