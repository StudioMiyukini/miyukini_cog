//! Composants partages JayXpose — StatCard, Badge, ActionButton, EmptyState, PlaceholderSection,
//! FormField, FormSection, DocumentRow.
//!
//! @id: jx_components @do: export_shared_ui_components
//! @role: ui @layer: service
//! @human: Composants UI reutilisables JayXpose: StatCard, Badge, ActionButton, EmptyState, FormField.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;

/// Carte statistique avec bordure coloree (XP-E01 dashboard).
#[component]
pub fn StatCard(label: String, value: String, icon: String, color: String) -> Element {
    let p = use_palette();
    rsx! {
        div {
            style: "background: {p.bg_secondary}; border-radius: 8px; padding: 16px; border-left: 3px solid {color};",

            div {
                style: "display: flex; justify-content: space-between; align-items: flex-start;",

                div {
                    p {
                        style: "font-size: 12px; color: {p.text_secondary}; margin-bottom: 4px;",
                        "{label}"
                    }
                    p {
                        style: "font-size: 24px; font-weight: 600; color: {p.text_high};",
                        "{value}"
                    }
                }
                span {
                    style: "font-size: 20px; opacity: 0.6;",
                    "{icon}"
                }
            }
        }
    }
}

/// Badge colore (statut, categorie).
#[component]
pub fn Badge(text: String, color: String) -> Element {
    rsx! {
        span {
            style: "padding: 4px 10px; background: {color}20; color: {color}; border-radius: 4px; font-size: 11px; font-weight: 500;",
            "{text}"
        }
    }
}

/// Bouton d'action.
#[component]
pub fn ActionButton(
    label: String,
    icon: String,
    #[props(default)] accent: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();
    let bg = if accent { p.accent_primary.to_string() } else { p.bg_overlay.to_string() };
    let color = if accent { "white".to_string() } else { p.text_primary.to_string() };
    let border = if accent { "none".to_string() } else { format!("1px solid {}", p.border_default) };

    rsx! {
        button {
            style: "display: flex; align-items: center; gap: 8px; padding: 10px 16px; background: {bg}; border: {border}; border-radius: 4px; color: {color}; cursor: pointer; font-size: 13px; transition: all 0.2s;",
            onclick: move |evt| onclick.call(evt),
            span { "{icon}" }
            span { "{label}" }
        }
    }
}

/// Section placeholder (fonctionnalite en cours).
#[component]
pub fn PlaceholderSection(title: &'static str, icon: &'static str) -> Element {
    let p = use_palette();
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; color: {p.text_muted};",

            span {
                style: "font-size: 64px; margin-bottom: 16px; opacity: 0.3;",
                "{icon}"
            }
            h2 {
                style: "font-size: 20px; color: {p.text_secondary};",
                "{title}"
            }
            p {
                style: "font-size: 14px; margin-top: 8px;",
                "Cette section sera bientot disponible"
            }
        }
    }
}

/// Etat vide avec bouton d'action.
#[component]
pub fn EmptyState(
    title: String,
    message: String,
    icon: String,
    #[props(default)] action_label: String,
    #[props(default)] on_action: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 300px; color: {p.text_muted};",

            span {
                style: "font-size: 64px; margin-bottom: 16px; opacity: 0.3;",
                "{icon}"
            }
            h2 {
                style: "font-size: 20px; color: {p.text_secondary};",
                "{title}"
            }
            p {
                style: "font-size: 14px; margin-top: 8px;",
                "{message}"
            }
            if !action_label.is_empty() {
                button {
                    style: "margin-top: 16px; padding: 10px 20px; background: {p.accent_primary}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    onclick: move |evt| on_action.call(evt),
                    "{action_label}"
                }
            }
        }
    }
}

/// Section titre (en-tete de page avec compteur optionnel).
#[component]
pub fn PageHeader(
    title: String,
    #[props(default)] subtitle: String,
    #[props(default)] count: Option<usize>,
    #[props(default)] action_label: String,
    #[props(default)] action_icon: String,
    #[props(default)] on_action: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();
    rsx! {
        div {
            style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 24px;",

            div {
                div {
                    style: "display: flex; align-items: center; gap: 12px;",
                    h2 {
                        style: "font-size: 24px; color: {p.text_high};",
                        "{title}"
                    }
                    if let Some(n) = count {
                        span {
                            style: "padding: 4px 10px; background: {p.bg_overlay}; border-radius: 12px; font-size: 12px; color: {p.text_secondary};",
                            "{n}"
                        }
                    }
                }
                if !subtitle.is_empty() {
                    p {
                        style: "font-size: 13px; color: {p.text_secondary}; margin-top: 4px;",
                        "{subtitle}"
                    }
                }
            }

            if !action_label.is_empty() {
                ActionButton {
                    label: action_label,
                    icon: action_icon,
                    accent: true,
                    onclick: move |evt| on_action.call(evt),
                }
            }
        }
    }
}

/// Champ de formulaire avec label.
#[component]
pub fn FormField(
    label: String,
    value: String,
    #[props(default)] placeholder: String,
    #[props(default)] field_type: String,
    #[props(default)] optional: bool,
    oninput: EventHandler<FormEvent>,
) -> Element {
    let p = use_palette();
    let ft = if field_type.is_empty() { "text" } else { &field_type };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 4px;",

            label {
                style: "font-size: 12px; color: {p.text_secondary}; font-weight: 500;",
                "{label}"
                if optional {
                    span { style: "color: {p.text_muted}; font-weight: 400;", " (optionnel)" }
                }
            }
            input {
                style: "padding: 10px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_primary}; font-size: 13px; box-sizing: border-box;",
                r#type: "{ft}",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |evt| oninput.call(evt),
            }
        }
    }
}

/// Champ textarea avec label.
#[component]
pub fn FormTextarea(
    label: String,
    value: String,
    #[props(default)] placeholder: String,
    #[props(default)] rows: u32,
    #[props(default)] optional: bool,
    oninput: EventHandler<FormEvent>,
) -> Element {
    let p = use_palette();
    let r = if rows == 0 { 3 } else { rows };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 4px;",

            label {
                style: "font-size: 12px; color: {p.text_secondary}; font-weight: 500;",
                "{label}"
                if optional {
                    span { style: "color: {p.text_muted}; font-weight: 400;", " (optionnel)" }
                }
            }
            textarea {
                style: "padding: 10px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_primary}; font-size: 13px; resize: vertical; font-family: inherit; box-sizing: border-box;",
                rows: "{r}",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |evt| oninput.call(evt),
            }
        }
    }
}

/// Section de formulaire avec titre.
#[component]
pub fn FormSection(title: String, children: Element) -> Element {
    let p = use_palette();
    rsx! {
        div {
            style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px; display: flex; flex-direction: column; gap: 16px;",

            h3 {
                style: "font-size: 14px; color: {p.text_high}; font-weight: 600; padding-bottom: 8px; border-bottom: 1px solid {p.border_default};",
                "{title}"
            }
            {children}
        }
    }
}

/// Ligne document dans le coffre-fort (XP-E09).
#[component]
pub fn DocumentRow(
    doc_type: String,
    label: String,
    file_name: String,
    status: String,
    expires_at: String,
    version: i32,
    on_view: EventHandler<MouseEvent>,
    on_delete: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();
    let (status_color, status_label) = match status.as_str() {
        "valide" => (p.success, "Valide"),
        "en_attente" => (p.warning, "En attente"),
        "expire" => (p.error, "Expire"),
        "rejete" => (p.error, "Rejete"),
        _ => (p.text_muted, "Inconnu"),
    };

    let type_icon = match doc_type.as_str() {
        "rib" => "🏦",
        "assurance" => "🛡️",
        "kbis" => "📄",
        "immatriculation" => "📋",
        "licence" => "📜",
        "urssaf" => "📑",
        "carte_pro" => "🪪",
        "diplome" => "🎓",
        "sanitaire" => "🏥",
        _ => "📎",
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 12px 16px; background: {p.bg_overlay}; border-radius: 4px;",

            // Icone type
            span { style: "font-size: 20px;", "{type_icon}" }

            // Infos
            div {
                style: "flex: 1;",

                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    p {
                        style: "font-size: 13px; color: {p.text_primary}; font-weight: 500;",
                        "{label}"
                    }
                    Badge { text: status_label.to_string(), color: status_color.to_string() }
                }
                div {
                    style: "display: flex; gap: 16px; font-size: 11px; color: {p.text_muted}; margin-top: 2px;",
                    span { "{file_name}" }
                    if !expires_at.is_empty() {
                        span { "Expire: {expires_at}" }
                    }
                    span { "v{version}" }
                }
            }

            // Actions
            div {
                style: "display: flex; gap: 4px;",
                button {
                    style: "padding: 6px 10px; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_secondary}; cursor: pointer; font-size: 11px;",
                    onclick: move |evt| on_view.call(evt),
                    "Voir"
                }
                button {
                    style: "padding: 6px 10px; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.error}; cursor: pointer; font-size: 11px;",
                    onclick: move |evt| on_delete.call(evt),
                    "Suppr."
                }
            }
        }
    }
}

/// Carte d'acces rapide (dashboard).
#[component]
pub fn QuickAccessCard(
    icon: String,
    title: String,
    count: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();
    rsx! {
        button {
            style: "display: flex; flex-direction: column; align-items: center; gap: 8px; padding: 20px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 8px; cursor: pointer; transition: all 0.2s; color: {p.text_primary}; text-align: center; min-width: 120px;",
            onclick: move |evt| onclick.call(evt),

            span { style: "font-size: 28px;", "{icon}" }
            span { style: "font-size: 13px; font-weight: 500; color: {p.text_high};", "{title}" }
            if !count.is_empty() {
                span { style: "font-size: 11px; color: {p.text_muted};", "{count}" }
            }
        }
    }
}
