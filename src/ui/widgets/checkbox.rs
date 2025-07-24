use eframe::egui;

use crate::config::types::ModifierKey;

pub fn checkbox(ui: &mut egui::Ui, set: &mut Vec<ModifierKey>, item: ModifierKey, label: &str) {
    let mut checked = set.contains(&item);
    if ui.checkbox(&mut checked, label).changed() {
        if checked {
            set.push(item);
        } else {
            set.retain(|k| item.eq(k));
        }
    }
}
