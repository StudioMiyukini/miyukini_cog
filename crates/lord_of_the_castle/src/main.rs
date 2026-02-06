//! Point d'entrée Lord of the Castle (mode autonome — développement / tests).
//!
//! Point d'accès utilisateur canonique : Miyukini Central (CANON-CENTRAL).
//! Ce binaire permet de lancer Lord of the Castle en fenêtre standalone.
//!
//! @id: lord_of_the_castle_main_entry
//! @do: run_native_lord_of_the_castle_app
//! @role: bootstrap
//! @layer: infra
//! @human: Binaire standalone (dev/test). Point d'accès canonique : Miyukini Central.

use eframe::egui;
use lord_of_the_castle::LordOfTheCastleApp;
use std::io;

const SAVE_PATH: &str = "lord_of_the_castle_save.bin";

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Lord of the Castle — Miyukini Survivor"),
        ..Default::default()
    };
    eframe::run_native(
        "Lord of the Castle",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            let mut app = LordOfTheCastleApp::new(cc);
            app.set_save_load_callbacks(
                |bytes| {
                    std::fs::write(SAVE_PATH, bytes).map_err(|e: io::Error| e.to_string())
                },
                || {
                    std::fs::read(SAVE_PATH).map_err(|e: io::Error| e.to_string())
                },
            );
            Ok(Box::new(app))
        }),
    )
}
