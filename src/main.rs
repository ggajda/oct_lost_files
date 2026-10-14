#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use app::MainApp;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "OCT lost files",
        options,
        Box::new(|_cc| {
            // This gives us image support:
            Ok(Box::<MainApp>::default())
        }),
    )
}
