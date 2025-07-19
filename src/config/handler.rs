use std::path::Path;

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use tokio::{
    fs::{read_to_string, write},
    sync::RwLock,
};

use crate::config::types::Config;

const CONFIG_PATH: &str = "config.json";

pub static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::default()));

pub async fn load_config() -> Result<Config> {
    if !Path::new(CONFIG_PATH).exists() {
        log::warn!("Config no encontrada, creando default...");
        save_config(&Config::default()).await?;
    }

    let content = read_to_string(CONFIG_PATH)
        .await
        .context("No se pudo leer el archivo de configuración")?;
    let config: Config = serde_json::from_str(&content)
        .context("No se pudo deserializar el archivo de configuración")?;
    Ok(config)
}

pub async fn save_config(config: &Config) -> Result<()> {
    let content = serde_json::to_string_pretty(config)
        .context("No se pudo serializar el archivo de configuración")?;
    write(CONFIG_PATH, content)
        .await
        .context("No se pudo escribir el archivo de configuración")?;
    log::info!("Configuración guardada");
    Ok(())
}

pub async fn init_global_config() -> Result<()> {
    let cfg = load_config().await?;
    *CONFIG.write().await = cfg;
    Ok(())
}
