use eframe::egui;

use crate::{
    config::types::MacroAction,
    ui::{
        state::{commands, ui_state::UIState},
        widgets::{empty_placeholder::empty_placeholder, icon_button::icon_button},
    },
};

pub fn editor_panel(ui: &mut egui::Ui, state: &mut UIState) {
    let Some(p_idx) = state.current_profile else {
        return;
    };
    let Some(m_idx) = state.current_macro else {
        empty_placeholder(ui, "Selecciona una macro para editarla.");
        return;
    };

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.vertical(|ui| {
            /* ---------- Título ---------- */
            let macro_name = &state.profiles[p_idx].macros[m_idx].name.clone();
            ui.horizontal(|ui| {
                ui.heading(format!("Secuencia de {}", macro_name));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("➕ Añadir acción").clicked() {
                        commands::add_action(state);
                    }
                    // ui.horizontal(|ui| {
                    //     if let Some(handle) = &state.recorder {
                    //         if ui.button("⏹ Parar").clicked() {
                    //             handle.stop();
                    //         }
                    //     } else if ui.button("⏺ Grabar").clicked() {
                    //         commands::start_recording(state, Some(rdev::Key::Escape));
                    //     }
                    // });
                });
            });
            ui.separator();

            /* ---------- Lista de acciones ---------- */
            egui::ScrollArea::both()
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    let mut to_remove = None;
                    for (idx, action) in state.profiles[p_idx].macros[m_idx]
                        .sequence
                        .iter_mut()
                        .enumerate()
                    {
                        // Frame que delimita la fila
                        let frame = egui::Frame::group(ui.style())
                            .inner_margin(egui::vec2(8.0, 6.0))
                            .stroke(ui.visuals().widgets.noninteractive.bg_stroke);

                        frame.show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.horizontal(|ui| {
                                action_combo(ui, action, &idx);
                                ui.add_space(8.0);
                                ui.horizontal(|ui| {
                                    ui.set_max_width(200.0);
                                    match action {
                                        MacroAction::KeyDown { key }
                                        | MacroAction::KeyUp { key } => {
                                            ui.add(
                                                egui::TextEdit::singleline(key)
                                                    .desired_width(ui.available_width()),
                                            );
                                        }
                                        MacroAction::Delay { ms } => {
                                            ui.add(
                                                egui::DragValue::new(ms).suffix(" ms").speed(10),
                                            );
                                        }
                                    };
                                });
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.horizontal(|ui| {
                                            ui.set_max_width(50.0);
                                            if icon_button(ui, "🗑", "Eliminar acción").clicked()
                                            {
                                                to_remove = Some(idx);
                                            }
                                        });
                                    },
                                );
                            });
                        });
                        ui.add_space(4.0); // separación entre filas
                    }
                    if let Some(idx) = to_remove {
                        commands::remove_action(state, idx);
                    }
                });
        });
    });
}

fn action_combo(ui: &mut egui::Ui, action: &mut MacroAction, idx: &usize) {
    let mut current = MacroAction::action_discriminant(action);
    egui::ComboBox::from_id_salt(format!("action_combo_{}", idx))
        .selected_text(current)
        .width(150.0)
        .show_ui(ui, |ui| {
            for variant in ["KeyDown", "KeyUp", "Delay"] {
                ui.selectable_value(&mut current, variant, variant);
            }
        });
    if current != MacroAction::action_discriminant(action) {
        *action = MacroAction::variant_to_action(current);
    }
    ui.add_space(6.0);
}
