//! VIS-E06 — Billets visiteur.

use super::components::{ActionButton, Badge};
use dioxus::prelude::*;
use miyukini_service_ui::use_palette;

/// Gestion des billets du visiteur.
#[component]
pub fn VisBillets() -> Element {
    let c = use_palette();
    let mut active_tab = use_signal(|| "actifs".to_string());

    let tab = active_tab.read().clone();
    let tab_actifs_bg = if tab == "actifs" {
        c.accent_blue
    } else {
        c.bg_secondary
    };
    let tab_actifs_color = if tab == "actifs" {
        "white"
    } else {
        c.text_primary
    };
    let tab_passes_bg = if tab == "passes" {
        c.accent_blue
    } else {
        c.bg_secondary
    };
    let tab_passes_color = if tab == "passes" {
        "white"
    } else {
        c.text_primary
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h2 {
                    style: "font-size: 24px; color: {c.text_white};",
                    "Mes billets"
                }

                ActionButton {
                    label: "Acheter un billet".to_string(),
                    icon: "🎟️".to_string(),
                    accent: true,
                    onclick: move |_| {},
                }
            }

            // Onglets
            div {
                style: "display: flex; gap: 8px;",

                button {
                    style: "padding: 8px 16px; background: {tab_actifs_bg}; border: none; border-radius: 6px; color: {tab_actifs_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("actifs".to_string()); },
                    "Billets actifs (2)"
                }
                button {
                    style: "padding: 8px 16px; background: {tab_passes_bg}; border: none; border-radius: 6px; color: {tab_passes_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("passes".to_string()); },
                    "Historique"
                }
            }

            // Billets
            if tab == "actifs" {
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    BilletCard {
                        event_name: "Salon Printemps 2026",
                        ticket_type: "Entree standard",
                        date: "Samedi 15 février 2026",
                        location: "Paris Expo - Hall A",
                        code: "TICK-2026-0042",
                        status: "valide",
                    }
                    BilletCard {
                        event_name: "Salon Artisanat",
                        ticket_type: "Pass 2 jours",
                        date: "15-16 mars 2026",
                        location: "Centre des Congres",
                        code: "TICK-2026-0058",
                        status: "valide",
                    }
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    BilletCard {
                        event_name: "Salon Hiver 2025",
                        ticket_type: "Entree standard",
                        date: "12 décembre 2025",
                        location: "Paris Expo - Hall B",
                        code: "TICK-2025-0198",
                        status: "utilise",
                    }
                }
            }
        }
    }
}

#[component]
fn BilletCard(
    event_name: &'static str,
    ticket_type: &'static str,
    date: &'static str,
    location: &'static str,
    code: &'static str,
    status: &'static str,
) -> Element {
    let c = use_palette();

    let (status_label, status_color) = match status {
        "valide" => ("Valide", c.accent_green),
        "utilise" => ("Utilise", c.text_muted),
        "expire" => ("Expire", c.accent_red),
        _ => ("Inconnu", c.text_muted),
    };

    let border = if status == "valide" {
        format!("2px solid {}", c.accent_green)
    } else {
        format!("1px solid {}", c.border)
    };

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border: {border}; border-radius: 12px; overflow: hidden;",

            // En-tête
            div {
                style: "background: {c.bg_hover}; padding: 16px 20px; display: flex; justify-content: space-between; align-items: center;",

                div {
                    h3 {
                        style: "font-size: 16px; color: {c.text_white}; margin-bottom: 4px;",
                        "{event_name}"
                    }
                    span {
                        style: "font-size: 13px; color: {c.text_muted};",
                        "{ticket_type}"
                    }
                }

                Badge {
                    text: status_label.to_string(),
                    color: status_color.to_string(),
                }
            }

            // Contenu
            div {
                style: "padding: 20px; display: flex; gap: 24px;",

                // Infos
                div {
                    style: "flex: 1;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        InfoRow { label: "Date", value: date }
                        InfoRow { label: "Lieu", value: location }
                        InfoRow { label: "Code", value: code }
                    }
                }

                // QR Code placeholder
                if status == "valide" {
                    div {
                        style: "width: 120px; height: 120px; background: white; border-radius: 8px; display: flex; align-items: center; justify-content: center;",

                        div {
                            style: "width: 100px; height: 100px; background: repeating-linear-gradient(45deg, #000 0, #000 10px, #fff 10px, #fff 20px); opacity: 0.5;",
                        }
                    }
                }
            }

            // Actions
            if status == "valide" {
                div {
                    style: "padding: 16px 20px; border-top: 1px solid {c.border}; display: flex; gap: 12px;",

                    button {
                        style: "flex: 1; padding: 10px; background: {c.accent_blue}; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 13px;",
                        "📱 Afficher le QR Code"
                    }
                    button {
                        style: "padding: 10px 16px; background: {c.bg_hover}; border: none; border-radius: 6px; color: {c.text_primary}; cursor: pointer; font-size: 13px;",
                        "📤 Partager"
                    }
                }
            }
        }
    }
}

#[component]
fn InfoRow(label: &'static str, value: &'static str) -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "display: flex; gap: 12px;",
            span {
                style: "font-size: 12px; color: {c.text_muted}; width: 60px;",
                "{label}"
            }
            span {
                style: "font-size: 13px; color: {c.text_primary};",
                "{value}"
            }
        }
    }
}
