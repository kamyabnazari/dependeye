use eframe::{egui, App};

pub struct DependEye;

impl App for DependEye {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DependEye");
            ui.separator();
            ui.label("Dependencies will appear here...");
        });
    }
}
