use eframe::egui;

pub fn empty_placeholder(ui: &mut egui::Ui, text: &str) {
    ui.centered_and_justified(|ui| {
        ui.label(text);
    });
}
