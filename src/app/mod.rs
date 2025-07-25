use anyhow::Result;

use crate::{
    app::lifecycle::LifecycleManager, config::handler::ConfigHandler,
    engine::processor::EventProcessor, ui::ViewWindow,
};

mod lifecycle;

pub struct FightMacrosApp {
    event_processor: EventProcessor,
    lifecycle_manager: LifecycleManager,
    window: ViewWindow,
}

impl FightMacrosApp {
    pub async fn new() -> Result<Self> {
        ConfigHandler::init_global_config().await?;

        let event_processor = EventProcessor::new()?;
        let lifecycle_manager = LifecycleManager::new();
        let window = ViewWindow::build()?;
        Ok(Self {
            event_processor,
            lifecycle_manager,
            window,
        })
    }

    pub async fn run(self) -> Result<()> {
        let lifecycle_handle =
            tokio::spawn(async move { self.lifecycle_manager.run(self.event_processor).await });
        self.window.run()?;

        lifecycle_handle.abort();
        Ok(())
    }
}
