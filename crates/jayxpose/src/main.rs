//! Point d'entrée binaire JayXpose (eframe).
//!
//! @id: jayxpose_main_entry
//! @do: run_native_eframe_jayxpose
//! @layer: app
//! Lance l'application native avec options viewport et délègue à JayXposeApp.

use eframe::egui;
use jayxpose::app::JayXposeApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("JayXpose"),
        ..Default::default()
    };
    eframe::run_native(
        "JayXpose",
        options,
        Box::new(|cc| Ok(Box::new(JayXposeApp::new(cc)))),
    )
}
