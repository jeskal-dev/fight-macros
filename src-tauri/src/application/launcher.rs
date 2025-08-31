use std::sync::Arc;

use anyhow::{Context, Result};
use tauri::{
    image::Image,
    menu::{IconMenuItem, MenuBuilder},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Listener, Manager, Runtime, Window, WindowEvent,
};
use tracing::{info, instrument, warn};

use crate::{
    application::{
        handler::{get_app_handle, set_app_handle},
        resource::ResourceManager,
    },
    config::{self, watchdog::ConfigWatchdog},
    engine::executor::Executor,
    input::{self, global::init_event_channel},
};

pub const MAIN_VIEW: &str = "main";

#[derive(Debug, Default)]
pub struct AppLauncher;

impl AppLauncher {
    pub fn launch() -> Result<()> {
        let ctx = tauri::generate_context!();

        tauri::Builder::default()
            .plugin(tauri_plugin_opener::init())
            .setup(move |app| {
                set_app_handle(app.handle().clone());
                Self::enable_tray(app)?;
                Self::setup_app(app)?;
                Ok(())
            })
            .on_tray_icon_event(Self::handle_tray_icon_event)
            .on_window_event(Self::handle_window_event)
            .invoke_handler(tauri::generate_handler![
                config::commands::load_config,
                config::commands::save_config,
                config::commands::change_active_profile,
                input::commands::send_keydown_event,
            ])
            .run(ctx)
            .context("error while running tauri application")?;

        Ok(())
    }

    fn show_main_window<R: Runtime>(app_handle: &AppHandle<R>) {
        if let Some(window) = app_handle.get_webview_window(MAIN_VIEW) {
            let _ = window.show();
            let _ = window.unminimize();
            let _ = window.set_focus();
        } else {
            warn!("Main window not found");
        }
    }

    fn hide_main_window<R: Runtime>(app_handle: &AppHandle<R>) {
        if let Some(window) = app_handle.get_webview_window(MAIN_VIEW) {
            let _ = window.hide();
        } else {
            warn!("Main window not found");
        }
    }

    fn handle_tray_icon_event<R: Runtime>(app_handle: &AppHandle<R>, event: TrayIconEvent) {
        if let TrayIconEvent::DoubleClick { button, .. } = event {
            if button == MouseButton::Left {
                Self::show_main_window(app_handle);
            }
        }
    }
    fn handle_window_event<R: Runtime>(window: &Window<R>, event: &tauri::WindowEvent) {
        match event {
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let _ = window.hide();
            }
            WindowEvent::Resized(_) => {
                if window.is_minimized().unwrap_or(false) {
                    let _ = window.hide();
                }
            }
            _ => {}
        }
    }
    fn enable_tray(app: &mut App) -> Result<()> {
        let quit_i = Image::from_bytes(include_bytes!("../../icons/x.png"))
            .context("Error al cargar el ícono")?;
        let hide_i = Image::from_bytes(include_bytes!("../../icons/eye-off.png"))
            .context("Error al cargar el ícono")?;
        let open_i = Image::from_bytes(include_bytes!("../../icons/eye.png"))
            .context("Error al cargar el ícono")?;

        let quit = IconMenuItem::with_id(app, "quit", "Salir", true, Some(quit_i), None::<&str>)?;
        let hide = IconMenuItem::with_id(app, "hide", "Ocultar", true, Some(hide_i), None::<&str>)?;
        let open = IconMenuItem::with_id(app, "open", "Abrir", true, Some(open_i), None::<&str>)?;

        let menu = MenuBuilder::new(app)
            .item(&open)
            .item(&hide)
            .item(&quit)
            .build()?;

        let icon_bytes = include_bytes!("../../icons/icon.png");
        let icon = Image::from_bytes(icon_bytes).context("Error al cargar el ícono")?;

        TrayIconBuilder::with_id("tray")
            .icon(icon)
            .menu(&menu)
            .on_menu_event(|app_handle, event| match event.id().as_ref() {
                "quit" => app_handle.exit(0),
                "open" => Self::show_main_window(app_handle),
                "hide" => Self::hide_main_window(app_handle),
                _ => {}
            })
            .build(app)?;

        Ok(())
    }

    #[instrument(skip(app))]
    fn setup_app(app: &mut App) -> Result<()> {
        init_event_channel();
        let handle = get_app_handle().clone();
        let executor = Arc::new(Executor::new().context("Error al iniciar el ejecutor")?);

        let mut resource_manager = ResourceManager::new(executor.clone())?;

        resource_manager.init_queue_handler()?;

        resource_manager.init_event_processor(handle.clone())?;

        let config_watchdog = ConfigWatchdog::new().context("Failed to create config watchdog")?;

        resource_manager
            .watchdogs
            .set_config_watchdog(config_watchdog);

        app.manage(executor);
        app.manage(Arc::new(std::sync::Mutex::new(resource_manager)));

        let resource_manager = app.state::<Arc<std::sync::Mutex<ResourceManager>>>();
        let resource_manager = resource_manager.inner().clone();

        app.handle().listen("tauri://exit-requested", move |_| {
            info!("Shutting down application");
            let manager = resource_manager.lock().unwrap();
            manager.shutdown();
        });

        Ok(())
    }
}
