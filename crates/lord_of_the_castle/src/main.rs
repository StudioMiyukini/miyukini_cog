//! Point d'entrée binaire Lord of the Castle (Miyukini Survivor).
//! Lance l'app Dioxus Desktop (rendu Blitz/WGPU). Peut aussi être intégré au Hub Central.
//!
//! Persistance KM : base path = `MIYUKINI_DATA_DIR` (env) ou répertoire courant.
//! Fichier DB : `lord_of_the_castle.db` (table survivor_slots).

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("lord_of_the_castle=debug,dioxus=info")
        .init();

    tracing::info!("Démarrage de Lord of the Castle (Miyukini Survivor)...");

    let config = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("Lord of the Castle — Miyukini Survivor")
                .with_inner_size(dioxus::desktop::LogicalSize::new(1024.0, 768.0))
                .with_min_inner_size(dioxus::desktop::LogicalSize::new(800.0, 600.0))
                .with_decorations(true)
                .with_resizable(true),
        )
        .with_disable_context_menu(false);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(lord_of_the_castle::SurvivorAppStandalone);
}
