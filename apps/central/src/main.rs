//! Miyukini Central — Application Native
//!
//! Hub de gestion des Services COG avec interface Steam-like.
//! Rendu natif via Dioxus Desktop (WGPU).

mod app;
mod audio;
mod components;
mod data;
mod screens;
mod services;
mod state;
mod theme;

fn main() {
    // Initialiser le logging
    tracing_subscriber::fmt()
        .with_env_filter("miyukini_central=debug,dioxus=info")
        .init();

    tracing::info!("Démarrage de Miyukini Central...");

    // Configuration de la fenêtre
    let config = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("Miyukini Central")
                .with_inner_size(dioxus::desktop::LogicalSize::new(1400.0, 900.0))
                .with_min_inner_size(dioxus::desktop::LogicalSize::new(1024.0, 700.0))
                .with_decorations(true)
                .with_resizable(true),
        )
        .with_disable_context_menu(true);

    // Lancer l'application
    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(app::App);
}
