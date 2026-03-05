//! ORG-E24 — Activation des services visiteur (jeux, concours, reservations).

use dioxus::prelude::*;
use miyukini_service_ui::use_palette;

/// Configuration des services visiteur.
#[component]
pub fn OrgServices(edition_id: String) -> Element {
    let c = use_palette();

    // États des services
    let billetterie = use_signal(|| true);
    let reservations = use_signal(|| true);
    let jeux = use_signal(|| false);
    let concours = use_signal(|| false);
    let pass_vip = use_signal(|| false);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tête
            h3 {
                style: "font-size: 18px; color: {c.text_white};",
                "Services visiteur"
            }

            p {
                style: "font-size: 14px; color: {c.text_secondary};",
                "Activez les services disponibles pour les visiteurs de votre evenement."
            }

            // Liste des services
            div {
                style: "display: flex; flex-direction: column; gap: 16px;",

                // Billetterie
                ServiceToggle {
                    icon: "🎟️",
                    title: "Billetterie",
                    description: "Vente de billets en ligne et gestion des entrees",
                    enabled: billetterie,
                }

                // Réservations
                ServiceToggle {
                    icon: "📅",
                    title: "Reservations",
                    description: "Permettre aux visiteurs de reserver des creneaux (ateliers, conferences...)",
                    enabled: reservations,
                }

                // Jeux
                ServiceToggle {
                    icon: "🎮",
                    title: "Jeux interactifs",
                    description: "Chasses au tresor, quiz, defis pour engager les visiteurs",
                    enabled: jeux,
                }

                // Concours
                ServiceToggle {
                    icon: "🏆",
                    title: "Concours et tirages",
                    description: "Organiser des concours avec lots a gagner",
                    enabled: concours,
                }

                // Pass VIP
                ServiceToggle {
                    icon: "⭐",
                    title: "Pass VIP",
                    description: "Offres speciales et acces privilegies pour visiteurs premium",
                    enabled: pass_vip,
                }
            }

            // Section Configuration avancée
            ServiceAdvancedConfig {}
        }
    }
}

/// Toggle pour activer/désactiver un service.
#[component]
fn ServiceToggle(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    enabled: Signal<bool>,
) -> Element {
    let c = use_palette();
    let is_enabled = *enabled.read();
    let bg = if is_enabled {
        c.bg_secondary
    } else {
        c.bg_main
    };
    let border = if is_enabled { c.accent_blue } else { c.border };
    let toggle_bg = if is_enabled {
        c.accent_blue
    } else {
        c.bg_hover
    };
    let toggle_pos = if is_enabled {
        "right: 2px"
    } else {
        "left: 2px"
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; background: {bg}; border: 1px solid {border}; border-radius: 8px; padding: 16px; cursor: pointer;",
            onclick: move |_| {
                let v = *enabled.read();
                enabled.set(!v);
            },

            // Icône
            span {
                style: "font-size: 32px;",
                "{icon}"
            }

            // Infos
            div {
                style: "flex: 1;",

                div {
                    style: "display: flex; align-items: center; gap: 8px;",

                    h4 {
                        style: "font-size: 14px; color: {c.text_white};",
                        "{title}"
                    }
                    if is_enabled {
                        span {
                            style: "display: inline-block; padding: 2px 8px; background: {c.accent_green}; color: white; border-radius: 4px; font-size: 11px; font-weight: 600;",
                            "Actif"
                        }
                    }
                }
                p {
                    style: "font-size: 12px; color: {c.text_muted}; margin-top: 4px;",
                    "{description}"
                }
            }

            // Toggle visuel
            div {
                style: "width: 48px; height: 28px; background: {toggle_bg}; border-radius: 14px; position: relative; transition: background 0.2s;",

                div {
                    style: "width: 24px; height: 24px; background: white; border-radius: 50%; position: absolute; top: 2px; {toggle_pos}; transition: all 0.2s;",
                }
            }
        }
    }
}

#[component]
fn ServiceAdvancedConfig() -> Element {
    let c = use_palette();

    rsx! {
        section {
            style: "margin-top: 24px; padding-top: 24px; border-top: 1px solid {c.border};",

            h3 {
                style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                "Configuration avancee"
            }

            div {
                style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                // Tarification
                PricingConfig {}

                // Capacité
                CapacityConfig {}
            }
        }
    }
}

#[component]
fn PricingConfig() -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

            h4 {
                style: "font-size: 14px; color: {c.text_white}; margin-bottom: 12px;",
                "💰 Tarification billets"
            }

            div {
                style: "display: flex; flex-direction: column; gap: 12px;",

                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    span {
                        style: "font-size: 13px; color: {c.text_primary};",
                        "Entree standard"
                    }
                    input {
                        r#type: "number",
                        style: "width: 80px; padding: 6px 8px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px; text-align: right;",
                        value: "10",
                    }
                }
                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    span {
                        style: "font-size: 13px; color: {c.text_primary};",
                        "Tarif reduit"
                    }
                    input {
                        r#type: "number",
                        style: "width: 80px; padding: 6px 8px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px; text-align: right;",
                        value: "5",
                    }
                }
                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    span {
                        style: "font-size: 13px; color: {c.text_primary};",
                        "Pass VIP"
                    }
                    input {
                        r#type: "number",
                        style: "width: 80px; padding: 6px 8px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px; text-align: right;",
                        value: "25",
                    }
                }
            }
        }
    }
}

#[component]
fn CapacityConfig() -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

            h4 {
                style: "font-size: 14px; color: {c.text_white}; margin-bottom: 12px;",
                "📊 Capacite et jauges"
            }

            div {
                style: "display: flex; flex-direction: column; gap: 12px;",

                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    span {
                        style: "font-size: 13px; color: {c.text_primary};",
                        "Capacite totale"
                    }
                    input {
                        r#type: "number",
                        style: "width: 80px; padding: 6px 8px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px; text-align: right;",
                        value: "500",
                    }
                }
                label {
                    style: "display: flex; align-items: center; gap: 8px; font-size: 13px; color: {c.text_primary}; cursor: pointer;",
                    input {
                        r#type: "checkbox",
                    }
                    "Limiter les ventes par creneau"
                }
                label {
                    style: "display: flex; align-items: center; gap: 8px; font-size: 13px; color: {c.text_primary}; cursor: pointer;",
                    input {
                        r#type: "checkbox",
                    }
                    "Envoyer alerte a 80% capacite"
                }
            }
        }
    }
}
