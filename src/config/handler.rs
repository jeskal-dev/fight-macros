use std::{
    fs::{read_to_string, write},
    path::Path,
    sync::RwLock,
};

use anyhow::{Context, Result};
use once_cell::sync::Lazy;

use crate::config::types::{Config, Profile};

const CONFIG_PATH: &str = "config.json";

pub static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::default()));

pub struct ConfigHandler;

impl ConfigHandler {
    pub fn load_config() -> Result<Config> {
        if !Path::new(CONFIG_PATH).exists() {
            log::warn!("Config no encontrada, creando default...");
            ConfigHandler::save_config(&Config::default())?;
        }

        let content =
            read_to_string(CONFIG_PATH).context("No se pudo leer el archivo de configuración")?;
        let config: Config = serde_json::from_str(&content)
            .context("No se pudo deserializar el archivo de configuración")?;
        Ok(config)
    }

    pub fn save_config(config: &Config) -> Result<()> {
        let content = serde_json::to_string_pretty(config)
            .context("No se pudo serializar el archivo de configuración")?;
        write(CONFIG_PATH, content).context("No se pudo escribir el archivo de configuración")?;
        log::info!("Configuración guardada");
        Ok(())
    }

    pub fn save_profiles(profiles: Vec<Profile>) -> Result<Config> {
        let current_config = CONFIG
            .read()
            .expect("Error al guardar configuración global");
        let new_config = Config {
            active_profile: current_config.active_profile,
            profiles,
        };

        ConfigHandler::save_config(&new_config)?;

        Ok(new_config)
    }

    pub async fn init_global_config() -> Result<()> {
        let cfg = ConfigHandler::load_config()?;
        *CONFIG
            .write()
            .expect("Error al guardar configuración global") = cfg;
        Ok(())
    }
}
