use eframe::NativeOptions;
use frontend::DependEyeApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let native_options: NativeOptions = eframe::NativeOptions::default();
    eframe::run_native(
        "DependEye",
        native_options,
        Box::new(|_cc| Ok(Box::new(DependEyeApp::new()))),
    )?;
    Ok(())
}
