//! Binaire JayKoa — lancement standalone (mode développement / test).
//!
//! @id: jaykoa_main
//! @do: launch_standalone_jaykoa_app
//! @layer: app
//! @human: Point d'entrée binaire JayKoa (eframe window).

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("JayKoa — Calendrier universel"),
        ..Default::default()
    };
    eframe::run_native(
        "JayKoa",
        options,
        Box::new(|cc| Ok(Box::new(jaykoa::app::JayKoaApp::new(cc)))),
    )
}
