//! Point d'entrée binaire JayXpose (standalone).
//! Lancé par Central (ServiceManager) ou directement par l'utilisateur.
//!
//! Env vars injectées par Central :
//! - MIYUKINI_SERVICE_ID
//! - MIYUKINI_DATA_DIR
//! - MIYUKINI_PROFILE_ID

use dioxus::prelude::*;
use jayxpose_app::{views::JayXposeView, DbContext};
use miyukini_service_ui::{Theme, ThemeContext};
use std::sync::Arc;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("jayxpose=debug,dioxus=info")
        .init();

    tracing::info!("Démarrage de JayXpose (standalone)...");

    let config = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("JayXpose — Profil Exposant")
                .with_inner_size(dioxus::desktop::LogicalSize::new(1200.0, 800.0))
                .with_min_inner_size(dioxus::desktop::LogicalSize::new(900.0, 600.0))
                .with_decorations(true)
                .with_resizable(true),
        )
        .with_disable_context_menu(false);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

#[component]
fn App() -> Element {
    let data_dir = std::env::var("MIYUKINI_DATA_DIR").unwrap_or_else(|_| {
        let base = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string());
        format!("{base}/Miyukini-COG/services/jayxpose/data")
    });
    let _ = std::fs::create_dir_all(&data_dir);
    let db_path = format!("{data_dir}/jayxpose.db");

    let db = match jayxpose::data::JayXposeDb::open(&db_path) {
        Ok(db) => Arc::new(db),
        Err(e) => {
            tracing::error!("Impossible d'ouvrir la DB JayXpose: {e}");
            return rsx! {
                div {
                    style: "padding: 48px; color: #c83737; text-align: center;",
                    h2 { "Erreur de démarrage" }
                    p { "Impossible d'ouvrir la base de données : {e}" }
                }
            };
        }
    };

    use_context_provider(|| {
        Signal::new(ThemeContext {
            theme: Theme::Gaming,
        })
    });
    use_context_provider(|| Signal::new(DbContext { db }));

    rsx! {
        div {
            style: "font-family: 'Segoe UI', sans-serif; background: #171a21; color: #c6d4df; min-height: 100vh; display: flex; flex-direction: column;",
            JayXposeView {}
        }
    }
}
