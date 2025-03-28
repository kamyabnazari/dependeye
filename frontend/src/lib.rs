use backend::db;
use eframe::{App, Frame, egui};
use tokio::runtime::Runtime;

pub struct DependEyeApp {
    connection_status: Option<bool>,
    repo_urls: Vec<String>,
    new_repo: String,
}

impl DependEyeApp {
    pub fn new() -> Self {
        Self {
            connection_status: None,
            repo_urls: Vec::new(),
            new_repo: String::new(),
        }
    }
}

impl App for DependEyeApp {
    // Remove the 'name' method; it's no longer part of the trait.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Button to check DB connection
            if ui.button("Check DB Connection").clicked() {
                // Create a temporary Tokio runtime for this async operation.
                let rt = Runtime::new().unwrap();
                let connected = rt.block_on(db::check_connection());
                self.connection_status = Some(connected);
            }

            // Display connection result
            if let Some(status) = self.connection_status {
                if status {
                    ui.label("Database connection successful!");
                } else {
                    ui.label("Failed to connect to database!");
                }
            }

            ui.separator();

            // UI for managing repository URLs
            ui.heading("Repository URLs");

            // Text field and add button
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_repo);
                if ui.button("Add Repo").clicked() {
                    if !self.new_repo.trim().is_empty() {
                        self.repo_urls.push(self.new_repo.trim().to_string());
                        self.new_repo.clear();
                    }
                }
            });

            ui.separator();

            // List existing repository URLs with a remove button for each
            for i in (0..self.repo_urls.len()).rev() {
                ui.horizontal(|ui| {
                    ui.label(&self.repo_urls[i]);
                    if ui.button("Remove").clicked() {
                        self.repo_urls.remove(i);
                    }
                });
            }
        });
    }
}
