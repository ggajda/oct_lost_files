#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod helper;

use anyhow::Result;
use app::MainApp;
use eframe::egui;

fn main() -> eframe::Result {
    let icon = get_icon().ok().unwrap_or_default();

    let app_name = concat!(
        env!("CARGO_PKG_DESCRIPTION"),
        " - ver. ",
        env!("CARGO_PKG_VERSION")
    );

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_icon(icon),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        app_name,
        options,
        Box::new(|_cc| Ok(Box::<MainApp>::default())),
    )
}

pub fn get_icon() -> Result<egui::IconData> {
    let icon_src = include_bytes!("../assets/icon.ico");
    let img = image::load_from_memory(icon_src)?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let icon = egui::IconData {
        rgba: rgba.into_raw(),
        width,
        height,
    };

    Ok(icon)
}
