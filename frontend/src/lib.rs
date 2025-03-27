use eframe::{egui, App};

pub struct DependencyWatcherApp;

impl App for DependencyWatcherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dependency Watcher MVP");
            ui.separator();
            ui.label("Dependencies will appear here...");
        });
    }
}
