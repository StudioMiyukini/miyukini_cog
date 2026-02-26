//! Point d'entree binaire JayFestival (standalone).

use std::sync::Arc;
use dioxus::prelude::*;
use jayfestival::data::JayFestivalDb;
use jayfestival_app::{DbContext, views::JayFestivalView};
use miyukini_service_ui::{Theme, ThemeContext};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("jayfestival=debug,dioxus=info")
        .init();

    tracing::info!("Demarrage de JayFestival (standalone)...");

    let config = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("JayFestival \u{2014} Festivals & \u{00c9}v\u{00e9}nements")
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
    // --- Theme context ---
    use_context_provider(|| Signal::new(ThemeContext { theme: Theme::Gaming }));

    // --- DB context ---
    let data_dir = std::env::var("MIYUKINI_DATA_DIR")
        .unwrap_or_else(|_| ".".to_string());
    let db_path = std::path::Path::new(&data_dir).join("jayfestival.db");

    let db = JayFestivalDb::open(&db_path)
        .expect("Impossible d'ouvrir la base JayFestival");

    use_context_provider(|| Signal::new(DbContext { db: Arc::new(db) }));

    rsx! {
        div {
            style: "font-family: 'Segoe UI', sans-serif; background: #171a21; color: #c6d4df; min-height: 100vh; display: flex; flex-direction: column;",

            JayFestivalView {}
        }
    }
}
