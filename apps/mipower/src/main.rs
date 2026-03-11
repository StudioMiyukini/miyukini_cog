use dioxus::prelude::*;

mod db;
mod init;
mod models;
mod render;
mod state;
mod views;

use models::SequenceMeta;
use state::{load_sequences, MipowerCtx, Notification, View};
use views::{dashboard::Dashboard, report::Report, settings::Settings, starter::Starter};

fn main() {
    let css = include_str!("../assets/app.css");
    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(format!("<style>{css}</style>"))
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("MIPOWER — Workflow Editor")
                .with_inner_size(dioxus::desktop::LogicalSize::new(1280.0, 820.0))
                .with_min_inner_size(dioxus::desktop::LogicalSize::new(900.0, 600.0)),
        )
        .with_data_directory({
            let mut p = std::path::PathBuf::new();
            if let Some(base) = std::env::var_os("APPDATA") {
                p = std::path::PathBuf::from(base).join("mipower").join("webview");
            }
            p
        });

    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let mip_root = load_persisted_root();

    use_context_provider(|| MipowerCtx {
        mip_root: Signal::new(mip_root.clone()),
    });

    let mut view: Signal<View> = use_signal(|| View::Dashboard);
    let mut sequences: Signal<Vec<SequenceMeta>> = use_signal(|| load_sequences(&mip_root));
    let mut notification: Signal<Option<Notification>> = use_signal(|| None);

    // Auto-dismiss notification after 4s
    use_effect(move || {
        if notification.read().is_some() {
            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(4)).await;
                notification.set(None);
            });
        }
    });

    let current_view = view.read().clone();

    rsx! {
        div { class: "app-shell",

            // ── Sidebar nav ───────────────────────────────────────────
            nav { class: "sidebar",
                div { class: "sidebar-logo", "MIPOWER" }
                {nav_item("Dashboard",   "◫", matches!(current_view, View::Dashboard),   move |_| view.set(View::Dashboard))}
                {nav_item("Starter",     "✦", matches!(current_view, View::Starter { .. }), move |_| view.set(View::Starter { prefill_title: None, prefill_desc: None }))}
                {nav_item("Parametres", "⚙", matches!(current_view, View::Settings),    move |_| view.set(View::Settings))}
            }

            // ── Main content ──────────────────────────────────────────
            main { class: "main-content",

                // Notification banner
                if let Some(notif) = notification.read().clone() {
                    div {
                        class: if notif.is_error { "notification notification-error" } else { "notification notification-ok" },
                        "{notif.message}"
                    }
                }

                match current_view {
                    View::Dashboard => rsx! {
                        Dashboard { view, sequences }
                    },
                    View::Report { slug, file_path, tab } => rsx! {
                        Report {
                            slug,
                            initial_file: file_path,
                            initial_tab: tab,
                            view,
                            sequences,
                            notification,
                        }
                    },
                    View::Starter { prefill_title, prefill_desc } => rsx! {
                        Starter {
                            prefill_title,
                            prefill_desc,
                            view,
                            notification,
                        }
                    },
                    View::Settings => rsx! {
                        Settings { view, notification }
                    },
                }
            }
        }
    }
}

fn nav_item(
    label: &'static str,
    icon:  &'static str,
    active: bool,
    onclick: impl FnMut(MouseEvent) + 'static,
) -> Element {
    let cls = if active { "nav-item nav-item-active" } else { "nav-item" };
    rsx! {
        button {
            class: "{cls}",
            onclick,
            span { class: "nav-icon", "{icon}" }
            span { class: "nav-label", "{label}" }
        }
    }
}

fn load_persisted_root() -> String {
    // 1. Env var override
    if let Ok(v) = std::env::var("MIP_ROOT") {
        if !v.is_empty() {
            return v;
        }
    }
    // 2. Persisted APPDATA file
    if let Some(base) = std::env::var_os("APPDATA") {
        let file = std::path::PathBuf::from(base).join("mipower").join("mip_root.txt");
        if let Ok(content) = std::fs::read_to_string(&file) {
            let trimmed = content.trim().to_string();
            if !trimmed.is_empty() && std::path::Path::new(&trimmed).exists() {
                return trimmed;
            }
        }
    }
    // 3. Fallback: current dir
    std::env::current_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
