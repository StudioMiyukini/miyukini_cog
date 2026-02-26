//! Point d'entrée binaire JayKonta (standalone).
//!
//! Initialise la DB KindMother, les providers de contexte (thème + DB),
//! puis lance l'UI Dioxus Desktop.

use std::sync::Arc;
use dioxus::prelude::*;
use miyukini_service_ui::{Theme, ThemeContext};
use jaykonta_app::{DbContext, views::JayKontaView};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("jaykonta=debug,dioxus=info")
        .init();

    tracing::info!("D\u{00e9}marrage de JayKonta (standalone)...");

    let config = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("JayKonta \u{2014} Comptabilit\u{00e9} COG")
                .with_inner_size(dioxus::desktop::LogicalSize::new(1100.0, 750.0))
                .with_min_inner_size(dioxus::desktop::LogicalSize::new(800.0, 600.0))
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
    // R\u{00e9}pertoire de donn\u{00e9}es (injecté par Central ou fallback local)
    let data_dir = std::env::var("MIYUKINI_DATA_DIR")
        .unwrap_or_else(|_| {
            let base = std::env::var("LOCALAPPDATA")
                .unwrap_or_else(|_| ".".to_string());
            format!("{base}/Miyukini-COG/services/jaykonta/data")
        });

    // Cr\u{00e9}er le r\u{00e9}pertoire de donn\u{00e9}es si n\u{00e9}cessaire
    let _ = std::fs::create_dir_all(&data_dir);

    // Initialiser la DB KindMother
    let db_path = format!("{data_dir}/jaykonta.db");
    let db = match jaykonta::data::JayKontaDb::open(&db_path) {
        Ok(db) => Arc::new(db),
        Err(e) => {
            tracing::error!("Impossible d'ouvrir la DB JayKonta: {e}");
            return rsx! {
                div {
                    style: "padding: 48px; color: #c83737; text-align: center;",
                    h2 { "Erreur de d\u{00e9}marrage" }
                    p { "Impossible d'ouvrir la base de donn\u{00e9}es : {e}" }
                    p { style: "font-size: 12px; color: #5c6873;", "Chemin : {db_path}" }
                }
            };
        }
    };

    // Provider : th\u{00e8}me
    use_context_provider(|| Signal::new(ThemeContext { theme: Theme::Gaming }));

    // Provider : DB
    use_context_provider(|| Signal::new(DbContext { db }));

    rsx! {
        div {
            style: "font-family: 'Segoe UI', sans-serif; background: #171a21; color: #c6d4df; min-height: 100vh; display: flex; flex-direction: column;",
            JayKontaView {}
        }
    }
}
