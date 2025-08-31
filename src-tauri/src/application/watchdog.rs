use crossbeam_channel::{bounded, Sender};

use crate::config::watchdog::ConfigWatchdog;

#[derive(Debug)]
pub struct WatchdogManager {
    config_watchdog: Option<ConfigWatchdog>,
    shutdown_tx: Sender<()>,
}

impl WatchdogManager {
    pub fn new() -> Self {
        let (shutdown_tx, _) = bounded(1);

        Self {
            config_watchdog: None,
            shutdown_tx,
        }
    }

    pub fn set_config_watchdog(&mut self, watchdog: ConfigWatchdog) {
        self.config_watchdog = Some(watchdog);
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());

        if let Some(watchdog) = &self.config_watchdog {
            watchdog.shutdown();
        }
    }

    pub fn wait_for_completion(&mut self) {
        if let Some(ref mut watchdog) = self.config_watchdog {
            watchdog.wait_for_completion();
        }
    }
}
