use eframe::egui;
use std::default::Default;

enum SharprPages {
    Map,
    LocationManager,
    Dashboard,
}
impl Default for SharprPages {
    fn default() -> Self {
        return Self::Dashboard;
    }
}

#[derive(Default)]
pub struct Sharpr {
    page: SharprPages,
}

impl Sharpr {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for Sharpr {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Hello World!");
        });
    }
}
