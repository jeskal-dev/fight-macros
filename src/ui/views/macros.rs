use eframe::egui::{self, Sense, UiBuilder, Window};

use crate::{
    config::types::{Macro, ModifierKey},
    ui::{
        state::{
            commands,
            ui_state::{Modal, UIState},
        },
        widgets::{
            checkbox::checkbox, empty_placeholder::empty_placeholder, icon_button::icon_button,
            toggle::toggle_ui,
        },
    },
};

pub fn macros_panel(ui: &mut egui::Ui, state: &mut UIState) {
    let Some(p_idx) = state.current_profile else {
        empty_placeholder(ui, "Selecciona un perfil.");
        return;
    };

    egui::TopBottomPanel::top("profile_header")
        .exact_height(36.0)
        .show_inside(ui, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                let profile = &mut state.profiles[p_idx];
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label("Perfil:");
                    ui.add(
                        egui::TextEdit::singleline(&mut profile.name)
                            .desired_width(200.0)
                            .margin(egui::vec2(4.0, 2.0)),
                    );
                });

                ui.add_space(12.0);
                ui.label("Activar con:");
                if ui.button(&profile.switch_key).clicked() {
                    // TODO: update-profile-switch-key()
                }

                ui.add_space(12.0);
                ui.label("Activar");
                let mut on = state.active_profile == state.current_profile;
                if toggle_ui(ui, &mut on).clicked() {
                    if state.active_profile == state.current_profile {
                        state.active_profile = None;
                    } else {
                        state.active_profile = state.current_profile;
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🗑").on_hover_text("Eliminar").clicked() {
                        commands::remove_profile(state);
                    }
                    if ui.button("📋").on_hover_text("Duplicar").clicked() {
                        commands::duplicate_profile(state);
                    }
                    if ui.button("💾").on_hover_text("Guardar").clicked() {
                        commands::save_changes();
                    }
                });
            });
        });

    egui::SidePanel::left("macros_list")
        .resizable(true)
        .default_width(260.0)
        .show_inside(ui, |ui| {
            ui.add_space(6.0);
            ui.vertical(|ui| {
                macros_header(ui, state);
                macros_scroll(ui, p_idx, state);
            });
        });
}

fn macros_header(ui: &mut egui::Ui, state: &mut UIState) {
    ui.horizontal(|ui| {
        ui.heading("Macros");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("➕").clicked() {
                commands::add_macro(state);
            }
        });
    });
    ui.separator();
}

fn macros_scroll(ui: &mut egui::Ui, idx: usize, state: &mut UIState) {
    let mut to_remove = None;
    let mut open_modal = None;
    let mut clicked_idx = None;

    egui::ScrollArea::vertical().show(ui, |ui| {
        if idx >= state.profiles.len() {
            state.current_profile = None;
            state.active_profile = None;
            state.current_macro = None;
            empty_placeholder(ui, "El perfil seleccionado ya no existe.");
            return;
        }

        let profile = &mut state.profiles[idx];
        for (i, m) in profile.macros.iter_mut().enumerate() {
            let active = state.current_macro == Some(i);
            let response = ui
                .scope_builder(UiBuilder::new().sense(Sense::click()), |ui| {
                    let frame = macro_frame(active, ui.style());

                    frame.show(ui, |ui| {
                        macro_row(ui, m, i, &mut to_remove, &mut open_modal)
                    });
                })
                .response;

            if response.clicked() {
                clicked_idx = Some(i);
            }
        }
    });

    // Fuera del loop y del préstamo de `profile`:
    if let Some(i) = clicked_idx {
        commands::set_current_macro(state, i);
    }
    if let Some(i) = to_remove {
        commands::remove_macro(state, i);
    }
    if let Some(i) = open_modal {
        state.modal = Modal::TriggerEditor(i);
    }
}

pub fn modal_layer(ctx: &egui::Context, state: &mut UIState) {
    let Some(p_idx) = state.current_profile else {
        return;
    };
    let Some(m_idx) = state.current_macro else {
        return;
    };

    if let Modal::TriggerEditor(open_idx) = &state.modal {
        if *open_idx != m_idx {
            return;
        }

        let macro_ = &mut state.profiles[p_idx].macros[m_idx];

        Window::new("Definir trigger")
            .open(&mut true)
            .show(ctx, |ui| {
                ui.label("Presiona la combinación deseada y luego Aceptar");
                ui.horizontal(|ui| {
                    checkbox(ui, &mut macro_.trigger.modifiers, ModifierKey::Ctrl, "Ctrl");
                    checkbox(ui, &mut macro_.trigger.modifiers, ModifierKey::Alt, "Alt");
                    checkbox(
                        ui,
                        &mut macro_.trigger.modifiers,
                        ModifierKey::Shift,
                        "Shift",
                    );
                    checkbox(ui, &mut macro_.trigger.modifiers, ModifierKey::Meta, "Meta");
                });
                ui.text_edit_singleline(&mut macro_.trigger.key);
                if ui.button("Aceptar").clicked() {
                    state.modal = Modal::None;
                }
            });
    }
}

fn macro_frame(active: bool, style: &egui::Style) -> egui::Frame {
    egui::Frame::group(style)
        .inner_margin(egui::vec2(6.0, 4.0))
        .stroke(if active {
            egui::Stroke::new(1.0, style.visuals.strong_text_color())
        } else {
            style.visuals.widgets.noninteractive.bg_stroke
        })
}

fn macro_row(
    ui: &mut egui::Ui,
    mac: &mut Macro,
    idx: usize,
    to_remove: &mut Option<usize>,
    open_modal: &mut Option<usize>,
) {
    ui.set_width(ui.available_width());
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.style_mut().spacing.item_spacing.y = 0.0;
            ui.add(
                egui::TextEdit::singleline(&mut mac.name).desired_width(ui.available_width() / 2.0),
            );
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if icon_button(ui, "🗑", "Eliminar macro").clicked() {
                *to_remove = Some(idx);
            }
            if ui
                .button(format!("{}", mac.trigger))
                .on_hover_text("Click para cambiar")
                .clicked()
            {
                *open_modal = Some(idx);
            }
        });
    });
}
