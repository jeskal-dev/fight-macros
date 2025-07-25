use eframe::egui;

use crate::ui::{
    state::{commands, ui_state::UIState},
    widgets::icon_button::icon_button,
};

pub fn profiles_panel(ui: &mut egui::Ui, state: &mut UIState) {
    egui::SidePanel::left("profiles_panel")
        .resizable(true)
        .default_width(250.0)
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Perfiles");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if icon_button(ui, "➕", "Crear perfil").clicked() {
                        commands::add_profile(state);
                    }
                });
            });

            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut clicked = None;
                for (idx, p) in state.profiles.iter_mut().enumerate() {
                    let active = state.current_profile == Some(idx);
                    if ui.selectable_label(active, &p.name).clicked() {
                        clicked = Some(idx);
                    }
                }
                if let Some(idx) = clicked {
                    commands::set_active_profile(state, idx);
                }
            });

            ui.separator();
            ui.vertical_centered(|ui| {
                if ui.button("Pausar").clicked() {
                    state.active_profile = None;
                }
            });
        });
}
