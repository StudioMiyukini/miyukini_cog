//! Composants partagés JayManga — StatCard, Badge, ActionButton, EmptyState, PageHeader.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;

/// Carte statistique avec bordure colorée.
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

/// Badge coloré (statut, catégorie).
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

/// État vide avec bouton d'action.
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

/// En-tête de page avec compteur et action optionnels.
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

/// Carte d'accès rapide (dashboard).
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
