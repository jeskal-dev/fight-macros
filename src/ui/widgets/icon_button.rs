use eframe::egui;

pub fn icon_button(ui: &mut egui::Ui, icon: &str, tooltip: &str) -> egui::Response {
    ui.button(icon).on_hover_text(tooltip)
}
