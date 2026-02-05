//! Point d'entrée Lord of the Click (mode autonome — développement / debug uniquement).
//!
//! **Point d'accès utilisateur canonique : Miyukini Central** (CANON-CENTRAL).
//! Tous les Services ont comme point d'accès utilisateur Miyukini Central ;
//! le jeu s'exécute dans le body de Central. Ce binaire permet de lancer Lord of the Click
//! en fenêtre standalone pour le développement et les tests ; il ne doit pas être
//! exposé comme point d'entrée utilisateur en production.
//!
//! @id: miyuclicker_main_entry
//! @do: run_native_miyuclicker_app
//! @role: bootstrap

use eframe::egui;
use miyuclicker::app::MiyuClickerApp;

/// Point d'entrée natif (eframe, fenêtre, MiyuClickerApp).
/// @id: miyuclicker_main_fn_main
/// @do: run_native_eframe_app
/// @role: bootstrap
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Lord of the Click — Premier jeu Miyukini"),
        ..Default::default()
    };
    eframe::run_native(
        "Lord of the Click",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(MiyuClickerApp::new(cc)))
        }),
    )
}
