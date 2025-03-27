use eframe::NativeOptions;
use frontend::DependEye;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "DependEye",
        native_options,
        Box::new(|_cc| Ok(Box::new(DependEye))),
    )?;
    Ok(())
}
