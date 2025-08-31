use std::sync::OnceLock;

use tauri::AppHandle;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub fn set_app_handle(handle: AppHandle) {
    APP_HANDLE.set(handle).unwrap();
}

pub fn get_app_handle() -> &'static AppHandle {
    APP_HANDLE.get().expect("AppHandle no ha sido inicializado")
}
