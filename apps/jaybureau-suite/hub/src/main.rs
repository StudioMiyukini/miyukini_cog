//! @id     jaybureau_suite_hub_main
//! @do     main
//! @role   ui
//! @layer  app
//! @human  jaybureau-suite — module « hub main ».

//! Jay Bureau — hub lanceur de la suite bureautique.
//!
//! Liste les apps (Docs, Sheets, Slides, Formulaire, Réunion) et les documents
//! récents du Drive. Lance les apps en tant que processus fils.

use dioxus::prelude::*;
use jaybureau_core::DocKind;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("jaybureau=debug,info")
        .init();

    let cfg = dioxus::desktop::Config::new().with_window(
        dioxus::desktop::WindowBuilder::new()
            .with_title("Jay Bureau")
            .with_inner_size(dioxus::desktop::LogicalSize::new(1200.0, 800.0)),
    );

    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        style { {include_str!("../assets/style.css")} }
        div {
            class: "jay-app",
            Header {}
            div {
                class: "jay-main",
                Sidebar {}
                Content {}
            }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header {
            class: "jay-header",
            div {
                class: "jay-brand",
                span { class: "jay-logo", "✨" }
                h1 { "Jay Bureau" }
            }
            div {
                class: "jay-user",
                span { "Connecté" }
            }
        }
    }
}

#[component]
fn Sidebar() -> Element {
    rsx! {
        aside {
            class: "jay-sidebar",
            h3 { "Applications" }
            ul {
                for kind in DocKind::all() {
                    AppTile { kind: *kind }
                }
            }
        }
    }
}

#[component]
fn AppTile(kind: DocKind) -> Element {
    let label = kind.label();
    let icon = kind.icon();

    rsx! {
        li {
            class: "jay-app-tile",
            onclick: move |_| {
                if let Err(e) = launch_app(kind) {
                    tracing::warn!("Lancement de {} impossible: {e}", kind.binary_name());
                }
            },
            span { class: "jay-app-icon", "{icon}" }
            span { class: "jay-app-label", "{label}" }
        }
    }
}

#[component]
fn Content() -> Element {
    rsx! {
        main {
            class: "jay-content",
            h2 { "Créer un document" }
            div {
                class: "jay-create-grid",
                for kind in DocKind::all() {
                    CreateCard { kind: *kind }
                }
            }
            h2 { style: "margin-top: 40px;", "Documents récents" }
            div {
                class: "jay-recent-empty",
                "Aucun document pour le moment. Créez-en un ci-dessus."
            }
        }
    }
}

#[component]
fn CreateCard(kind: DocKind) -> Element {
    let label = kind.label();
    let icon = kind.icon();

    rsx! {
        div {
            class: "jay-create-card",
            onclick: move |_| {
                if let Err(e) = launch_app(kind) {
                    tracing::warn!("Lancement de {} impossible: {e}", kind.binary_name());
                }
            },
            div { class: "jay-create-icon", "{icon}" }
            div { class: "jay-create-label", "{label}" }
            div { class: "jay-create-new", "+ Nouveau" }
        }
    }
}

/// Lance un app Jay Bureau en tant que processus fils.
///
/// Le binaire est cherché **uniquement** à côté de l'exécutable du hub ; pas
/// de repli sur `PATH` (R-011, détournement de binaire).
fn launch_app(kind: DocKind) -> std::io::Result<()> {
    tracing::info!("Lancement de {}", kind.binary_name());
    let exe = std::env::current_exe()?;
    let dir = exe.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "répertoire du lanceur introuvable")
    })?;
    let bin_path = kind
        .sibling_binary_path(dir)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e.to_string()))?;
    std::process::Command::new(bin_path).spawn().map(|_| ())
}
