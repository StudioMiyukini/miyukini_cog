//! Point d'entrée du Hub Miyukini Central (MVP).

use eframe::egui;
use miyukini_central::app::MiyukiniCentralApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Miyukini Central — Hub Services"),
        ..Default::default()
    };
    eframe::run_native(
        "Miyukini Central",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(MiyukiniCentralApp::new(cc)))
        }),
    )
}
