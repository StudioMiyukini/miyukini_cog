//! Composants reutilisables MiyuCloud : Breadcrumb, FileIcon, SizeLabel.
//!
//! @id: miyucloud_ui_components
//! @do: provide_reusable_cloud_ui_atoms
//! @role: component
//! @layer: presentation
//!
//! Atoms et molecules pour l'UI MiyuCloud. Tous les styles sont extraits
//! en variables locales avant le bloc RSX (piege Dioxus 0.6).

use dioxus::prelude::*;

use crate::state::use_app_state;
use super::state::BreadcrumbItem;

// ════════════════════════════════════════════════════════════════════════════
// Breadcrumb
// ════════════════════════════════════════════════════════════════════════════

/// Fil d'Ariane : Racine > Photos > 2026.
/// Chaque segment est cliquable pour naviguer vers le dossier correspondant.
#[component]
pub fn Breadcrumb(
    items: Vec<BreadcrumbItem>,
    on_navigate: EventHandler<Option<String>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let container_style = format!(
        "display: flex; align-items: center; gap: 4px; padding: 8px 0; font-size: 13px; color: {};",
        c.text_secondary
    );
    let last_index = if items.is_empty() { 0 } else { items.len() - 1 };

    rsx! {
        nav {
            style: "{container_style}",
            for (i, item) in items.iter().enumerate() {
                {
                    let is_last = i == last_index;
                    let item_color = if is_last { c.text_white } else { c.text_link };
                    let cursor = if is_last { "default" } else { "pointer" };
                    let font_weight = if is_last { "600" } else { "400" };
                    let item_style = format!(
                        "color: {item_color}; cursor: {cursor}; font-weight: {font_weight}; background: none; border: none; padding: 2px 4px; border-radius: 4px; font-size: 13px;"
                    );
                    let folder_id = item.id.clone();
                    let name = item.name.clone();

                    rsx! {
                        if i > 0 {
                            span {
                                style: "color: {c.text_muted}; margin: 0 2px; user-select: none;",
                                ">"
                            }
                        }
                        button {
                            style: "{item_style}",
                            onclick: move |_| {
                                if !is_last {
                                    on_navigate.call(folder_id.clone());
                                }
                            },
                            "{name}"
                        }
                    }
                }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// FileIcon
// ════════════════════════════════════════════════════════════════════════════

/// Icone representant un fichier ou dossier en fonction du type MIME.
#[component]
pub fn FileIcon(
    mime_type: String,
    is_folder: bool,
    #[props(default = 32)]
    size: u32,
) -> Element {
    let icon = if is_folder {
        "\u{1F4C1}" // folder
    } else {
        mime_to_icon(&mime_type)
    };
    let font_size = size;

    rsx! {
        span {
            style: "font-size: {font_size}px; line-height: 1; display: inline-flex; align-items: center; justify-content: center; min-width: {font_size}px;",
            "{icon}"
        }
    }
}

/// Convertit un type MIME en icone emoji.
fn mime_to_icon(mime: &str) -> &'static str {
    if mime.starts_with("image/") {
        "\u{1F5BC}" // framed picture
    } else if mime.starts_with("video/") {
        "\u{1F3AC}" // clapper board
    } else if mime.starts_with("audio/") {
        "\u{1F3B5}" // musical note
    } else if mime == "application/pdf" {
        "\u{1F4C4}" // document
    } else if mime.starts_with("text/") {
        "\u{1F4DD}" // memo
    } else if mime.contains("zip")
        || mime.contains("tar")
        || mime.contains("rar")
        || mime.contains("7z")
        || mime.contains("compressed")
    {
        "\u{1F4E6}" // package
    } else if mime.contains("spreadsheet")
        || mime.contains("excel")
        || mime.contains("presentation")
        || mime.contains("powerpoint")
    {
        "\u{1F4CA}" // bar chart
    } else if mime.contains("word") || mime.contains("document") {
        "\u{1F4C3}" // page with curl
    } else {
        "\u{1F4C4}" // page facing up
    }
}

// ════════════════════════════════════════════════════════════════════════════
// SizeLabel
// ════════════════════════════════════════════════════════════════════════════

/// Affiche une taille en octets formatee (KB, MB, GB).
#[component]
pub fn SizeLabel(
    bytes: u64,
    #[props(default = false)]
    compact: bool,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let label = format_size(bytes);
    let font_size = if compact { "11px" } else { "13px" };

    rsx! {
        span {
            style: "color: {c.text_secondary}; font-size: {font_size}; white-space: nowrap;",
            "{label}"
        }
    }
}

/// Formate une taille en octets en chaine lisible.
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    const TB: u64 = 1024 * GB;

    if bytes >= TB {
        let val = bytes as f64 / TB as f64;
        format!("{val:.1} TB")
    } else if bytes >= GB {
        let val = bytes as f64 / GB as f64;
        format!("{val:.1} GB")
    } else if bytes >= MB {
        let val = bytes as f64 / MB as f64;
        format!("{val:.1} MB")
    } else if bytes >= KB {
        let val = bytes as f64 / KB as f64;
        format!("{val:.1} KB")
    } else {
        format!("{bytes} o")
    }
}

// ════════════════════════════════════════════════════════════════════════════
// StatusBadge (statut sync)
// ════════════════════════════════════════════════════════════════════════════

/// Badge de statut synchronisation P2P (placeholder Phase A).
#[component]
pub fn SyncBadge(peers_online: u32, syncing: bool) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let badge_bg = if syncing {
        c.accent_blue
    } else if peers_online > 0 {
        c.accent_green
    } else {
        c.bg_hover
    };
    let label = if syncing {
        "Sync en cours...".to_string()
    } else if peers_online > 0 {
        format!("{peers_online} pair(s) en ligne")
    } else {
        "Hors ligne".to_string()
    };
    let dot_color = if peers_online > 0 || syncing {
        "#4ade80"
    } else {
        "#6b7280"
    };

    rsx! {
        div {
            style: "display: inline-flex; align-items: center; gap: 6px; padding: 4px 12px; background: {badge_bg}20; border: 1px solid {badge_bg}40; border-radius: 16px;",
            div {
                style: "width: 8px; height: 8px; border-radius: 50%; background: {dot_color};",
            }
            span {
                style: "font-size: 12px; font-weight: 500; color: {c.text_secondary};",
                "{label}"
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// ActionButton (bouton action reutilisable)
// ════════════════════════════════════════════════════════════════════════════

/// Bouton d'action avec icone.
#[component]
pub fn ActionButton(
    label: String,
    icon: String,
    onclick: EventHandler<MouseEvent>,
    #[props(default = false)]
    primary: bool,
    #[props(default = false)]
    danger: bool,
    #[props(default = false)]
    small: bool,
    #[props(default = false)]
    disabled: bool,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let bg = if disabled {
        c.bg_hover.to_string()
    } else if danger {
        c.accent_red.to_string()
    } else if primary {
        c.accent_blue.to_string()
    } else {
        c.bg_hover.to_string()
    };
    let text_color = if disabled {
        c.text_muted
    } else if primary || danger {
        c.text_white
    } else {
        c.text_primary
    };
    let padding = if small { "4px 8px" } else { "6px 14px" };
    let font_size = if small { "11px" } else { "13px" };
    let cursor = if disabled { "not-allowed" } else { "pointer" };
    let opacity = if disabled { "0.5" } else { "1" };

    rsx! {
        button {
            style: "display: inline-flex; align-items: center; gap: 6px; padding: {padding}; background: {bg}; color: {text_color}; border: none; border-radius: 4px; cursor: {cursor}; font-size: {font_size}; opacity: {opacity}; transition: all 0.2s;",
            disabled: disabled,
            onclick: move |evt| {
                if !disabled {
                    onclick.call(evt);
                }
            },
            span { "{icon}" }
            span { "{label}" }
        }
    }
}
