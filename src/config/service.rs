use std::sync::RwLockReadGuard;

use anyhow::{Ok, Result};

use crate::config::{
    handler::CONFIG,
    types::{Config, Profile},
};

pub struct ConfigService;

impl ConfigService {
    pub fn get_config_reader<'a>() -> Result<RwLockReadGuard<'a, Config>> {
        Ok(CONFIG.read().expect("Error al leer configuración global"))
    }

    pub fn get_profiles() -> Result<Vec<Profile>> {
        let cfg = ConfigService::get_config_reader()?;
        Ok(cfg.profiles.clone())
    }

    pub fn get_active_profile_idx() -> Result<Option<usize>> {
        let cfg = ConfigService::get_config_reader()?;
        Ok(cfg.active_profile)
    }
}
