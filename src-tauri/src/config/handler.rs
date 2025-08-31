use crate::config::model::FlatStorage;
use crate::config::parser;
use crate::domain::config::Config;
use anyhow::{Context, Result};
use parking_lot::RwLock;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, LazyLock};

pub(crate) const CONFIG_PATH: &str = "config.json";

pub(crate) static CONFIG: LazyLock<Arc<RwLock<FlatStorage>>> =
    LazyLock::new(|| Arc::new(RwLock::new(load_from_disk().unwrap_or_default())));

#[inline]
pub(crate) fn load_from_disk() -> Result<FlatStorage> {
    // Cargar o crear la configuración
    let cfg = match fs::read(CONFIG_PATH) {
        Ok(bytes) if !bytes.is_empty() => serde_json::from_slice(&bytes)
            .context("No se pudo deserializar el archivo de configuración"),
        _ => Ok(Config::default()),
    }?;

    // Convertir y devolver
    Ok(parser::config_to_flat(&cfg))
}

#[inline]
pub(crate) fn save_to_disk(store: &FlatStorage) -> Result<()> {
    // Serializar la configuración
    let json = serde_json::to_vec_pretty(&parser::flat_to_config(store))
        .context("No se pudo serializar la configuración")?;

    let cfg_path = Path::new(CONFIG_PATH);
    let tmp_path = cfg_path.with_extension("tmp");

    {
        let mut file =
            BufWriter::new(File::create(&tmp_path).context("No se pudo crear archivo temporal")?);
        file.write_all(&json)?;
        file.flush()?;
        file.get_mut()
            .sync_all()
            .context("No se pudo sincronizar archivo temporal")?;
    }

    // Renombrar atómicamente
    fs::rename(&tmp_path, cfg_path).with_context(|| {
        format!(
            "No se pudo reemplazar {} por {}",
            tmp_path.display(),
            cfg_path.display()
        )
    })?;

    Ok(())
}

// #[inline]
// pub fn get_config_path() -> PathBuf {
//     PathBuf::from(CONFIG_PATH)
// }

#[inline]
pub fn get_config() -> FlatStorage {
    CONFIG.read().clone()
}

#[inline]
pub fn save_config<F, R>(f: F) -> Result<R>
where
    F: FnOnce(&mut FlatStorage) -> R,
{
    let mut flat = CONFIG.write();
    let result = f(&mut flat);
    save_to_disk(&flat)?;

    Ok(result)
}
