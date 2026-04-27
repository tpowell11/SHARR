use eframe::egui;
mod sharpr;
mod location;
mod weatherdata;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "SHARPR",
        native_options,
        Box::new(|cc| Ok(Box::new(sharpr::Sharpr::new(cc)))),
    );
}

