use eframe::egui;

pub fn labeled_textbox(
    ui: &mut egui::Ui,
    label: &str,
    text: &mut String,
    width: f32,
) -> egui::Response {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::TextEdit::singleline(text).desired_width(width))
    })
    .response
}
