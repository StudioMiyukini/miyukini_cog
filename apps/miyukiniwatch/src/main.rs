//! Point d'entree binaire MiyukiniWatch (standalone).
//!
//! Initialise la DB KindMother, les providers de contexte (theme + DB + profil),
//! puis lance l'UI Dioxus Desktop.

use std::sync::Arc;
use dioxus::prelude::*;
use miyukini_service_ui::{Theme, ThemeContext};
use miyukiniwatch_app::{DbContext, ProfileContext, views::MiyukiniWatchView};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("miyukiniwatch=debug,dioxus=info")
        .init();

    tracing::info!("Demarrage de MiyukiniWatch (standalone)...");

    let config = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("MiyukiniWatch \u{2014} Habitudes & Mesures")
                .with_inner_size(dioxus::desktop::LogicalSize::new(900.0, 650.0))
                .with_min_inner_size(dioxus::desktop::LogicalSize::new(700.0, 500.0))
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
    // Repertoire de donnees (injecte par Central ou fallback local)
    let data_dir = std::env::var("MIYUKINI_DATA_DIR")
        .unwrap_or_else(|_| {
            let base = std::env::var("LOCALAPPDATA")
                .unwrap_or_else(|_| ".".to_string());
            format!("{base}/Miyukini-COG/services/miyukiniwatch/data")
        });

    // Creer le repertoire de donnees si necessaire
    let _ = std::fs::create_dir_all(&data_dir);

    // Initialiser la DB KindMother
    let db_path = format!("{data_dir}/miyukiniwatch.db");
    let db = match miyukiniwatch::MiyukiniWatchDb::open(&db_path) {
        Ok(db) => Arc::new(db),
        Err(e) => {
            tracing::error!("Impossible d'ouvrir la DB MiyukiniWatch: {e}");
            return rsx! {
                div {
                    style: "padding: 48px; color: #c83737; text-align: center;",
                    h2 { "Erreur de demarrage" }
                    p { "Impossible d'ouvrir la base de donnees : {e}" }
                    p { style: "font-size: 12px; color: #5c6873;", "Chemin : {db_path}" }
                }
            };
        }
    };

    // Profile ID depuis env var ou "default"
    let profile_id = std::env::var("MIYUKINI_PROFILE_ID")
        .unwrap_or_else(|_| "default".to_string());

    // Provider : theme
    use_context_provider(|| Signal::new(ThemeContext { theme: Theme::Gaming }));

    // Provider : DB
    use_context_provider(|| Signal::new(DbContext { db }));

    // Provider : profil
    use_context_provider(|| Signal::new(ProfileContext { profile_id }));

    rsx! {
        div {
            style: "font-family: 'Segoe UI', sans-serif; background: #171a21; color: #c6d4df; min-height: 100vh; display: flex; flex-direction: column;",
            MiyukiniWatchView {}
        }
    }
}
