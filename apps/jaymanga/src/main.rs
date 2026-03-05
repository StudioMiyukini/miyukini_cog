//! Point d'entree binaire JayManga (standalone).

use dioxus::prelude::*;
use jaymanga::data::JayMangaDb;
use jaymanga_app::{views, DbContext};
use miyukini_service_ui::{Theme, ThemeContext};
use std::sync::Arc;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("jaymanga=debug,dioxus=info")
        .init();

    tracing::info!("Demarrage de JayManga (standalone)...");

    let config = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("JayManga \u{2014} Manga & Lecture")
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
    // Ouvrir la DB JayManga
    let data_dir =
        std::env::var("MIYUKINI_DATA_DIR").unwrap_or_else(|_| ".miyukini/jaymanga".to_string());

    let db_path = std::path::Path::new(&data_dir).join("jaymanga.db");

    // Creer le repertoire si necessaire
    if let Some(parent) = db_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let db = match JayMangaDb::open(&db_path) {
        Ok(db) => Arc::new(db),
        Err(e) => {
            tracing::error!("Impossible d'ouvrir la DB JayManga: {e}");
            return rsx! {
                div {
                    style: "font-family: 'Segoe UI', sans-serif; background: #171a21; color: #c6d4df; min-height: 100vh; display: flex; align-items: center; justify-content: center;",
                    p { "Erreur DB: {e}" }
                }
            };
        }
    };

    // Fournir les contextes
    use_context_provider(|| Signal::new(DbContext { db }));
    use_context_provider(|| {
        Signal::new(ThemeContext {
            theme: Theme::default(),
        })
    });

    rsx! {
        views::JayMangaView {}
    }
}
