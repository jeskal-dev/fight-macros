use crate::{
    config::{handler, parser},
    domain::config::Config,
};

#[tauri::command]
pub async fn load_config() -> Result<Config, String> {
    let store = handler::get_config();

    let cfg = parser::flat_to_config(&store);
    Ok(cfg)
}

#[tauri::command]
pub fn save_config(config: Config) -> Result<(), String> {
    let flat = parser::config_to_flat(&config);

    handler::save_config(|cfg| {
        *cfg = flat;
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn change_active_profile(id: u64) -> Result<(), String> {
    handler::save_config(|cfg| {
        cfg.selected_profile_id = Some(id);
    })
    .map_err(|err| err.to_string())
}
