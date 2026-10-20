use super::helper::auto_counter;
use crate::components::button;
use eframe::egui;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

pub struct MainApp {
    progress: u32,
    runtime: Runtime,
    counter_task: Option<JoinHandle<()>>,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            progress: 0,
            runtime: Runtime::new().expect("Failed to create Tokio runtime"),
            counter_task: None,
        }
    }
}

impl eframe::App for MainApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                button(ui, "Start", true, || {
                    self.progress += 1;

                    // Start
                    self.counter_task = Some(self.runtime.spawn(async {
                        auto_counter().await;
                    }));
                });

                button(ui, "stop", true, || {
                    // Stop
                    if let Some(task) = self.counter_task.take() {
                        task.abort();
                    }
                });

                ui.label(self.progress.to_string());
            });
        });
    }
}
