//! Miyukini Central Mobile — point d'entrée Android.
//!
//! Client léger Dioxus Mobile pour piloter le COG Host depuis Android.
//! Zéro donnée locale — tout passe par le bridge (SSH / Whitelist / API Key).

use dioxus::prelude::*;

mod app;
mod bridge;
mod platform;
mod state;
mod theme;

// Écrans
mod screens {
    pub mod connexion;
    pub mod pairing;
}

// Composants
mod components {
    pub mod bottom_nav;
    pub mod bridge_status;
    pub mod service_card;
}

// Vues services
mod services {
    pub mod home;
    pub mod market;
    pub mod miou_chat;
    pub mod mws_status;
    pub mod remote_control;
}

fn main() {
    // Initialiser le logging
    tracing_subscriber::fmt()
        .with_env_filter("central_mobile=debug,miyukini=info")
        .init();

    tracing::info!("Central Mobile v{}", env!("CARGO_PKG_VERSION"));

    // Lancer l'application Dioxus Mobile
    dioxus::launch(app::App);
}
