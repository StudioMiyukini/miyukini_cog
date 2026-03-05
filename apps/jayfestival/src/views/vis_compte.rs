//! VIS-E12 — Compte visiteur.

use super::components::ActionButton;
use dioxus::prelude::*;
use miyukini_service_ui::use_palette;

/// Mon compte visiteur.
#[component]
pub fn VisCompte() -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px; max-width: 800px;",

            h2 {
                style: "font-size: 24px; color: {c.text_white};",
                "Mon compte"
            }

            // Informations personnelles
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Informations personnelles"
                }

                div {
                    style: "display: flex; gap: 24px;",

                    // Avatar
                    div {
                        style: "flex-shrink: 0;",

                        div {
                            style: "width: 100px; height: 100px; background: {c.bg_hover}; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 48px;",
                            "👤"
                        }
                        button {
                            style: "display: block; margin-top: 8px; padding: 6px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 12px;",
                            "Modifier"
                        }
                    }

                    // Formulaire
                    div {
                        style: "flex: 1; display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                        FormField { label: "Prenom", value: "Jean", field_type: "text" }
                        FormField { label: "Nom", value: "Dupont", field_type: "text" }
                        FormField { label: "Email", value: "jean.dupont@email.com", field_type: "email" }
                        FormField { label: "Telephone", value: "+33 6 12 34 56 78", field_type: "tel" }
                    }
                }

                div {
                    style: "display: flex; justify-content: flex-end; margin-top: 16px;",
                    ActionButton {
                        label: "Sauvegarder".to_string(),
                        icon: "💾".to_string(),
                        accent: true,
                        onclick: move |_| {},
                    }
                }
            }

            // Préférences
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Preferences"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    PreferenceToggle {
                        label: "Notifications par email",
                        description: "Recevoir les rappels d'evenements",
                        enabled: true,
                    }
                    PreferenceToggle {
                        label: "Notifications push",
                        description: "Alertes sur votre appareil",
                        enabled: true,
                    }
                    PreferenceToggle {
                        label: "Newsletter",
                        description: "Actualites et offres des organisateurs",
                        enabled: false,
                    }
                }
            }

            // Points et historique
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Points de fidelite"
                }

                div {
                    style: "display: flex; justify-content: space-between; align-items: center; padding: 16px; background: {c.accent_orange}20; border-radius: 8px; margin-bottom: 16px;",

                    div {
                        span {
                            style: "font-size: 14px; color: {c.text_muted}; display: block;",
                            "Solde actuel"
                        }
                        span {
                            style: "font-size: 32px; font-weight: bold; color: {c.accent_orange};",
                            "⭐ 350"
                        }
                    }

                    button {
                        style: "padding: 10px 20px; background: {c.accent_orange}; border: none; border-radius: 6px; color: white; cursor: pointer;",
                        "Voir les recompenses"
                    }
                }

                h4 {
                    style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 8px;",
                    "Historique recent"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    PointsHistoryRow {
                        description: "Quiz du jour complete",
                        points: 50,
                        date: "10 fev 2026",
                    }
                    PointsHistoryRow {
                        description: "Visite stand partenaire",
                        points: 15,
                        date: "9 fev 2026",
                    }
                    PointsHistoryRow {
                        description: "Echange cafe offert",
                        points: -50,
                        date: "8 fev 2026",
                    }
                }
            }

            // Sécurité
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Securite"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    SecurityRow {
                        icon: "🔒",
                        label: "Mot de passe",
                        value: "••••••••",
                        action: "Modifier",
                    }
                    SecurityRow {
                        icon: "📱",
                        label: "Authentification 2FA",
                        value: "Non activee",
                        action: "Activer",
                    }
                }
            }

            // Zone danger
            section {
                style: "background: {c.bg_secondary}; border: 1px solid {c.accent_red}40; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {c.accent_red}; margin-bottom: 12px;",
                    "Zone dangereuse"
                }

                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",

                    div {
                        p {
                            style: "font-size: 14px; color: {c.text_primary}; margin-bottom: 4px;",
                            "Supprimer mon compte"
                        }
                        p {
                            style: "font-size: 12px; color: {c.text_muted};",
                            "Cette action est irreversible"
                        }
                    }

                    button {
                        style: "padding: 8px 16px; background: transparent; border: 1px solid {c.accent_red}; border-radius: 6px; color: {c.accent_red}; cursor: pointer;",
                        "Supprimer"
                    }
                }
            }
        }
    }
}

#[component]
fn FormField(label: &'static str, value: &'static str, field_type: &'static str) -> Element {
    let c = use_palette();

    rsx! {
        div {
            label {
                style: "display: block; font-size: 12px; color: {c.text_muted}; margin-bottom: 4px;",
                "{label}"
            }
            input {
                r#type: "{field_type}",
                value: "{value}",
                style: "width: 100%; padding: 10px 12px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; font-size: 14px;",
            }
        }
    }
}

#[component]
fn PreferenceToggle(label: &'static str, description: &'static str, enabled: bool) -> Element {
    let c = use_palette();

    let toggle_bg = if enabled { c.accent_blue } else { c.bg_hover };
    let toggle_pos = if enabled { "20px" } else { "2px" };

    rsx! {
        div {
            style: "display: flex; justify-content: space-between; align-items: center; padding: 12px; background: {c.bg_hover}; border-radius: 6px;",

            div {
                p {
                    style: "font-size: 14px; color: {c.text_primary}; margin-bottom: 2px;",
                    "{label}"
                }
                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "{description}"
                }
            }

            div {
                style: "width: 40px; height: 22px; background: {toggle_bg}; border-radius: 11px; position: relative; cursor: pointer;",
                div {
                    style: "width: 18px; height: 18px; background: white; border-radius: 50%; position: absolute; top: 2px; left: {toggle_pos}; transition: left 0.2s;",
                }
            }
        }
    }
}

#[component]
fn PointsHistoryRow(description: &'static str, points: i32, date: &'static str) -> Element {
    let c = use_palette();

    let (sign, color) = if points > 0 {
        ("+", c.accent_green)
    } else {
        ("", c.accent_red)
    };

    rsx! {
        div {
            style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0; border-bottom: 1px solid {c.border};",

            div {
                p {
                    style: "font-size: 13px; color: {c.text_primary};",
                    "{description}"
                }
                p {
                    style: "font-size: 11px; color: {c.text_muted};",
                    "{date}"
                }
            }

            span {
                style: "font-size: 14px; font-weight: 500; color: {color};",
                "{sign}{points} pts"
            }
        }
    }
}

#[component]
fn SecurityRow(
    icon: &'static str,
    label: &'static str,
    value: &'static str,
    action: &'static str,
) -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "display: flex; justify-content: space-between; align-items: center; padding: 12px; background: {c.bg_hover}; border-radius: 6px;",

            div {
                style: "display: flex; align-items: center; gap: 12px;",

                span { style: "font-size: 20px;", "{icon}" }
                div {
                    p {
                        style: "font-size: 14px; color: {c.text_primary};",
                        "{label}"
                    }
                    p {
                        style: "font-size: 12px; color: {c.text_muted};",
                        "{value}"
                    }
                }
            }

            button {
                style: "padding: 6px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 12px;",
                "{action}"
            }
        }
    }
}
