//! Point d'entree binaire Lord of the Click / MiyuClicker (standalone).

use std::path::PathBuf;
use dioxus::prelude::*;
use miyukini_service_ui::{Theme, ThemeContext};
use miyuclicker_app::{DataDirContext, views::MiyuClickerView};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("miyuclicker=debug,dioxus=info")
        .init();

    tracing::info!("Demarrage de Lord of the Click (standalone)...");

    let config = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("Lord of the Click \u{2014} Idle Strategy")
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
    let data_dir = std::env::var("MIYUKINI_DATA_DIR")
        .unwrap_or_else(|_| {
            let base = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string());
            format!("{base}/Miyukini-COG/services/miyuclicker/data")
        });
    let _ = std::fs::create_dir_all(&data_dir);

    use_context_provider(|| Signal::new(ThemeContext { theme: Theme::Gaming }));
    use_context_provider(|| Signal::new(DataDirContext { data_dir: PathBuf::from(&data_dir) }));

    rsx! {
        div {
            style: "font-family: 'Segoe UI', sans-serif; background: #171a21; color: #c6d4df; min-height: 100vh; display: flex; flex-direction: column;",
            MiyuClickerView {}
        }
    }
}
