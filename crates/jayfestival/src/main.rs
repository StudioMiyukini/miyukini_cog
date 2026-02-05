//! Point d'entrée binaire JayFestival (eframe).
//!
//! @id: jayfestival_main_entry
//! @do: run_native_eframe_jayfestival
//! @layer: app
//! Lance l'application native avec options viewport et délègue à JayFestivalApp.

use eframe::egui;
use jayfestival::app::JayFestivalApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("JayFestival"),
        ..Default::default()
    };
    eframe::run_native(
        "JayFestival",
        options,
        Box::new(|cc| Ok(Box::new(JayFestivalApp::new(cc)))),
    )
}
