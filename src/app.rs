use eframe::egui;

#[derive(Default)]
pub struct MainApp {
    pub progress: u32,
}

// impl Default for MainApp {
//     fn default() -> Self {
//         Self { progress: 0 }
//     }
// }

impl eframe::App for MainApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label("Progress: ");
            });
        });
    }
}
