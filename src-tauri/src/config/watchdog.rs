use std::path::Path;

use anyhow::Result;
use crossbeam_channel::Sender;
use notify::{RecursiveMode, Watcher};
use tracing::{error, info};

use crate::config::handler::{load_from_disk, CONFIG, CONFIG_PATH};

#[derive(Debug)]
pub struct ConfigWatchdog {
    shutdown_tx: Sender<()>,
    thread: Option<std::thread::JoinHandle<()>>,
    _watcher: notify::RecommendedWatcher, // <-- agregado
}

impl ConfigWatchdog {
    pub fn new() -> Result<Self> {
        let (tx, rx) = crossbeam_channel::unbounded();
        let (shutdown_tx, shutdown_rx) = crossbeam_channel::bounded::<()>(1);

        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })?;

        watcher
            .watch(Path::new(CONFIG_PATH), RecursiveMode::NonRecursive)
            .map_err(|e| anyhow::anyhow!("Error al observar config.json: {e:?}"))?;

        let thread = std::thread::Builder::new()
            .name("config_watchdog".into())
            .spawn(move || {
                loop {
                    crossbeam::select! {
                        recv(rx) -> event => {
                            match event {
                                Ok(_) => match load_from_disk() {
                                    Ok(new) => {
                                        *CONFIG.write() = new;
                                        info!("[INFO] config.json recargado");
                                    }
                                    Err(e) => error!("[ERROR] No se pudo recargar config.json: {:?}", e),
                                },
                                Err(e) => error!("[ERROR] Error al observar config.json: {:?}", e),
                            }
                        }
                        recv(shutdown_rx) -> _ => break,
                    }
                }
            })?;

        Ok(Self {
            shutdown_tx,
            thread: Some(thread),
            _watcher: watcher,
        })
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());
    }

    pub fn wait_for_completion(&mut self) {
        if let Some(thread) = self.thread.take() {
            if let Err(e) = thread.join() {
                error!("Error waiting for config watchdog: {:?}", e);
            }
        }
    }
}
