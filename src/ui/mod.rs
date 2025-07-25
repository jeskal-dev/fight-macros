use anyhow::Result;

use crate::{config::service::ConfigService, ui::state::ui_state::UIState};

mod state;
mod view;
mod views;
mod widgets;

pub struct ViewWindow {
    view: UIState,
    options: eframe::NativeOptions,
}

impl ViewWindow {
    pub fn build() -> Result<Self> {
        let options = eframe::NativeOptions {
            renderer: eframe::Renderer::Glow,
            centered: true,
            ..Default::default()
        };
        let view = UIState::new(
            ConfigService::get_profiles()?,
            ConfigService::get_active_profile_idx()?,
        );
        Ok(Self { view, options })
    }

    pub fn run(&self) -> Result<()> {
        if let Err(e) = eframe::run_native(
            "Fight Macros",
            self.options.clone(),
            Box::new(|_| Ok(Box::new(self.view.clone()))),
        ) {
            log::error!("Error al ejecutar la aplicación: {}", e);
        }
        Ok(())
    }
}
