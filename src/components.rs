use eframe::egui;
pub fn button(ui: &mut egui::Ui, label: &str, enabled: bool, on_click: impl FnOnce()) {
    button_sized(ui, label, [80.0, 30.0], enabled, on_click);
}

pub fn button_sized(
    ui: &mut egui::Ui,
    label: &str,
    size: [f32; 2],
    enabled: bool,
    on_click: impl FnOnce(),
) {
    if ui
        .add_enabled(
            enabled,
            egui::Button::new(label).min_size(egui::vec2(size[0], size[1])),
        )
        .clicked()
    {
        on_click();
    }
}
