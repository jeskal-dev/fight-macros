use eframe::egui;

use crate::ui::{
    state::{commands::sync_config, ui_state::UIState},
    views::{
        editor::editor_panel,
        macros::{macros_panel, modal_layer},
        profile::profiles_panel,
    },
};

impl eframe::App for UIState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            profiles_panel(ui, self);
            macros_panel(ui, self);
            editor_panel(ui, self);
            modal_layer(ctx, self);
        });

        sync_config(self);
    }
}
