//! Composants partages JayKonta — KpiCard, AmountDisplay, ProgressBar, Badge, PlaceholderSection.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;

/// Carte KPI (valeur, delta, tendance).
#[component]
pub fn KpiCard(label: String, value: String, detail: String, icon: String, positive: bool) -> Element {
    let p = use_palette();
    let detail_color = if positive { p.success } else { p.error };

    rsx! {
        div {
            style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px;",

            div {
                style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 12px;",

                span {
                    style: "font-size: 13px; color: {p.text_secondary};",
                    "{label}"
                }
                span {
                    style: "font-size: 20px;",
                    "{icon}"
                }
            }

            p {
                style: "font-size: 28px; font-weight: 600; color: {p.text_high}; margin-bottom: 4px;",
                "{value}"
            }
            span {
                style: "font-size: 12px; color: {detail_color};",
                "{detail}"
            }
        }
    }
}

/// Affichage de montant colore (+vert / -rouge).
#[component]
pub fn AmountDisplay(amount: f64, currency: String) -> Element {
    let p = use_palette();
    let color = if amount >= 0.0 { p.success } else { p.error };
    let formatted = format!("{amount:+.2} {currency}");

    rsx! {
        span {
            style: "font-weight: 500; color: {color};",
            "{formatted}"
        }
    }
}

/// Barre de progression.
#[component]
pub fn ProgressBar(value: f64, max: f64, #[props(default)] color: String) -> Element {
    let p = use_palette();
    let pct = if max > 0.0 { (value / max * 100.0).min(100.0) } else { 0.0 };
    let bar_color = if color.is_empty() { p.accent_primary.to_string() } else { color };
    let bg_color = p.bg_overlay;

    rsx! {
        div {
            style: "width: 100%; height: 8px; background: {bg_color}; border-radius: 4px; overflow: hidden;",

            div {
                style: "width: {pct}%; height: 100%; background: {bar_color}; border-radius: 4px; transition: width 0.3s;",
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

/// Ligne de mouvement.
#[component]
pub fn MovementRow(
    description: String,
    date: String,
    category: String,
    amount: f64,
    currency: String,
) -> Element {
    let p = use_palette();
    let amount_color = if amount >= 0.0 { p.success } else { p.error };
    let amount_str = format!("{amount:+.2} {currency}");

    rsx! {
        div {
            style: "display: flex; justify-content: space-between; align-items: center; padding: 12px 16px; background: {p.bg_overlay}; border-radius: 4px;",

            div {
                style: "flex: 1;",

                div {
                    style: "display: flex; align-items: center; gap: 8px;",

                    p {
                        style: "font-size: 13px; color: {p.text_primary};",
                        "{description}"
                    }
                    if !category.is_empty() {
                        span {
                            style: "padding: 2px 8px; background: {p.bg_secondary}; border-radius: 3px; font-size: 10px; color: {p.text_muted};",
                            "{category}"
                        }
                    }
                }
                p {
                    style: "font-size: 11px; color: {p.text_muted}; margin-top: 2px;",
                    "{date}"
                }
            }
            span {
                style: "font-size: 14px; font-weight: 500; color: {amount_color}; white-space: nowrap;",
                "{amount_str}"
            }
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
