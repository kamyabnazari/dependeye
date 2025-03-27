use frontend::DependencyWatcherApp;

#[tokio::main]
async fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "DependEye",
        native_options,
        Box::new(|_cc| Box::new(DependencyWatcherApp)),
    )
}
