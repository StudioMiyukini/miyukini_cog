//! Point d'entree binaire Jay1Tribu (standalone).
//! Lance par Central (ServiceManager) ou directement par l'utilisateur.
//!
//! Env vars injectees par Central :
//! - MIYUKINI_SERVICE_ID
//! - MIYUKINI_DATA_DIR
//! - MIYUKINI_PROFILE_ID

use dioxus::prelude::*;
use jay1tribu::Jay1TribuDb;
use jay1tribu_app::views::Jay1TribuView;
use jay1tribu_app::DbContext;
use miyukini_service_ui::{Theme, ThemeContext};
use std::sync::Arc;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("jay1tribu=debug,dioxus=info")
        .init();

    tracing::info!("Demarrage de Jay1Tribu (standalone)...");

    let config = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("Jay1Tribu \u{2014} Messagerie & Tribus")
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
    let profile_id = std::env::var("MIYUKINI_PROFILE_ID").unwrap_or_default();
    let data_dir = std::env::var("MIYUKINI_DATA_DIR").unwrap_or_else(|_| {
        std::env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    });

    // Theme
    use_context_provider(|| {
        Signal::new(ThemeContext {
            theme: Theme::default(),
        })
    });

    // DB Jay1Tribu
    let db_path = std::path::PathBuf::from(&data_dir).join("jay1tribu.db");
    let db = Jay1TribuDb::open(&db_path).expect("Impossible d'ouvrir Jay1TribuDb");
    use_context_provider(|| Signal::new(DbContext { db: Arc::new(db) }));

    // Profile ID partage dans un signal pour les composants enfants
    let profile_sig = use_signal(|| profile_id);

    rsx! {
        div {
            style: "font-family: 'Segoe UI', sans-serif; background: #1a1a2e; color: #e5e7eb; min-height: 100vh; display: flex; flex-direction: column;",
            Jay1TribuView { profile_id: profile_sig }
        }
    }
}
