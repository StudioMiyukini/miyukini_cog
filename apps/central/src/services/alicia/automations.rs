//! Ecran Automatisations Alicia Home Assistante.
//!
//! Affiche les automatisations configurees (trigger -> conditions -> actions).
//! Placeholder : les donnees reelles seront fournies par miyualicia-automations.

use dioxus::prelude::*;

use super::state::AliciaSnapshot;
use crate::state::use_app_state;

// ─────────────────────────────────────────────────────────────────────────────
// Types locaux (placeholders en attendant miyualicia-automations)
// ─────────────────────────────────────────────────────────────────────────────

/// Exemple d'automatisation pre-definie (placeholder visuel).
struct AutomationExample {
    name: &'static str,
    trigger_icon: &'static str,
    trigger_desc: &'static str,
    conditions: &'static str,
    actions: &'static str,
    enabled: bool,
}

/// Exemples pour peupler l'interface (demonstration).
const EXAMPLE_AUTOMATIONS: &[AutomationExample] = &[
    AutomationExample {
        name: "Lumiere salon au coucher de soleil",
        trigger_icon: "[sun]",
        trigger_desc: "Declencheur : coucher de soleil",
        conditions: "Jour de semaine uniquement",
        actions: "Allumer Lampe Salon a 60%",
        enabled: true,
    },
    AutomationExample {
        name: "Alerte intrusion",
        trigger_icon: "[door]",
        trigger_desc: "Declencheur : capteur porte ouverte",
        conditions: "Mode absence actif",
        actions: "Notification push + allumer toutes les lumieres",
        enabled: true,
    },
    AutomationExample {
        name: "Chauffage matin",
        trigger_icon: "[clock]",
        trigger_desc: "Declencheur : 06:30 chaque jour",
        conditions: "Temperature < 19 C",
        actions: "Thermostat salon a 21 C",
        enabled: false,
    },
];

// ─────────────────────────────────────────────────────────────────────────────
// Composant principal
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn AutomationsScreen() -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();
    let _snapshot = use_context::<Signal<Option<AliciaSnapshot>>>();

    // Signal local pour afficher/masquer le formulaire
    let mut show_add_form = use_signal(|| false);
    let mut form_name = use_signal(String::new);
    let mut form_trigger = use_signal(String::new);
    let mut form_condition = use_signal(String::new);
    let mut form_action = use_signal(String::new);

    let is_form_visible = *show_add_form.read();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px; max-width: 1000px;",

            // Header
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    h2 {
                        style: "font-size: 16px; font-weight: 600; color: {c.text_white};",
                        "Automatisations"
                    }
                    p {
                        style: "font-size: 13px; color: {c.text_secondary};",
                        "Regles automatiques : quand un evenement se produit, Alicia execute des actions."
                    }
                }
                button {
                    style: "padding: 8px 16px; background: {c.accent_blue}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer; font-size: 13px; font-weight: 500;",
                    onclick: move |_| {
                        let current = *show_add_form.read();
                        show_add_form.set(!current);
                    },
                    if is_form_visible { "Annuler" } else { "Nouvelle automatisation" }
                }
            }

            // Formulaire d'ajout (visible si show_add_form)
            if is_form_visible {
                div {
                    style: "padding: 20px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.accent_blue}40; display: flex; flex-direction: column; gap: 16px;",
                    h3 {
                        style: "font-size: 14px; font-weight: 600; color: {c.text_white};",
                        "Nouvelle automatisation"
                    }

                    // Nom
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        label {
                            style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                            "Nom"
                        }
                        input {
                            r#type: "text",
                            placeholder: "Ex: Lumiere salon au coucher de soleil",
                            style: "padding: 8px 12px; background: {c.bg_secondary}; border-radius: 4px; border: 1px solid {c.border}; color: {c.text_primary}; font-size: 13px;",
                            value: "{form_name}",
                            oninput: move |evt: Event<FormData>| {
                                form_name.set(evt.value());
                            },
                        }
                    }

                    // Trigger
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        label {
                            style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                            "Declencheur (trigger)"
                        }
                        input {
                            r#type: "text",
                            placeholder: "Ex: coucher de soleil, capteur porte, 06:30 chaque jour",
                            style: "padding: 8px 12px; background: {c.bg_secondary}; border-radius: 4px; border: 1px solid {c.border}; color: {c.text_primary}; font-size: 13px;",
                            value: "{form_trigger}",
                            oninput: move |evt: Event<FormData>| {
                                form_trigger.set(evt.value());
                            },
                        }
                    }

                    // Ligne Condition + Action
                    div {
                        style: "display: flex; gap: 16px;",

                        // Condition
                        div {
                            style: "flex: 1; display: flex; flex-direction: column; gap: 4px;",
                            label {
                                style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                                "Condition (optionnelle)"
                            }
                            input {
                                r#type: "text",
                                placeholder: "Ex: temperature < 19 C",
                                style: "padding: 8px 12px; background: {c.bg_secondary}; border-radius: 4px; border: 1px solid {c.border}; color: {c.text_primary}; font-size: 13px;",
                                value: "{form_condition}",
                                oninput: move |evt: Event<FormData>| {
                                    form_condition.set(evt.value());
                                },
                            }
                        }

                        // Action
                        div {
                            style: "flex: 1; display: flex; flex-direction: column; gap: 4px;",
                            label {
                                style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                                "Action"
                            }
                            input {
                                r#type: "text",
                                placeholder: "Ex: allumer lampe salon a 60%",
                                style: "padding: 8px 12px; background: {c.bg_secondary}; border-radius: 4px; border: 1px solid {c.border}; color: {c.text_primary}; font-size: 13px;",
                                value: "{form_action}",
                                oninput: move |evt: Event<FormData>| {
                                    form_action.set(evt.value());
                                },
                            }
                        }
                    }

                    // Bouton enregistrer (placeholder — pas de backend)
                    div {
                        style: "display: flex; gap: 12px; margin-top: 4px;",
                        button {
                            style: "padding: 8px 20px; background: {c.accent_green}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer; font-size: 13px; font-weight: 500; opacity: 0.5;",
                            disabled: true,
                            "Enregistrer (backend non integre)"
                        }
                    }
                }
            }

            // Banner placeholder
            div {
                style: "padding: 16px 20px; background: {c.accent_orange}15; border: 1px solid {c.accent_orange}30; border-radius: 8px;",
                p {
                    style: "font-size: 13px; color: {c.accent_orange}; font-weight: 500; margin-bottom: 4px;",
                    "Fonctionnalite en preparation"
                }
                p {
                    style: "font-size: 12px; color: {c.text_secondary};",
                    "Les automatisations seront disponibles avec le crate miyualicia-automations. "
                    "Les cartes ci-dessous sont des exemples de presentation."
                }
            }

            // Cartes d'automatisations (exemples visuels)
            div {
                style: "display: flex; flex-direction: column; gap: 12px;",
                for auto_example in EXAMPLE_AUTOMATIONS.iter() {
                    {
                        let name = auto_example.name;
                        let trigger_icon = auto_example.trigger_icon;
                        let trigger_desc = auto_example.trigger_desc;
                        let conditions = auto_example.conditions;
                        let actions = auto_example.actions;
                        let enabled = auto_example.enabled;
                        let toggle_bg = if enabled { c.accent_green } else { c.text_muted };
                        let toggle_label = if enabled { "Actif" } else { "Inactif" };
                        let card_opacity = if enabled { "1.0" } else { "0.6" };

                        rsx! {
                            div {
                                style: "padding: 16px 20px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; display: flex; align-items: center; gap: 16px; opacity: {card_opacity};",

                                // Icone trigger
                                div {
                                    style: "width: 40px; height: 40px; border-radius: 8px; background: {c.bg_secondary}; display: flex; align-items: center; justify-content: center; font-size: 16px; color: {c.text_muted}; flex-shrink: 0;",
                                    "{trigger_icon}"
                                }

                                // Contenu
                                div {
                                    style: "flex: 1; display: flex; flex-direction: column; gap: 4px; min-width: 0;",
                                    p {
                                        style: "font-size: 14px; font-weight: 500; color: {c.text_white};",
                                        "{name}"
                                    }
                                    div {
                                        style: "display: flex; gap: 16px; flex-wrap: wrap;",
                                        span {
                                            style: "font-size: 12px; color: {c.accent_blue};",
                                            "{trigger_desc}"
                                        }
                                        span {
                                            style: "font-size: 12px; color: {c.text_secondary};",
                                            "Si : {conditions}"
                                        }
                                        span {
                                            style: "font-size: 12px; color: {c.accent_green};",
                                            "Alors : {actions}"
                                        }
                                    }
                                }

                                // Toggle actif / inactif
                                div {
                                    style: "display: flex; align-items: center; gap: 6px; padding: 4px 10px; border-radius: 12px; background: {toggle_bg}20; flex-shrink: 0;",
                                    div {
                                        style: "width: 6px; height: 6px; border-radius: 50%; background: {toggle_bg};",
                                    }
                                    span {
                                        style: "font-size: 11px; font-weight: 500; color: {toggle_bg};",
                                        "{toggle_label}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Note informative
            div {
                style: "padding: 16px; background: {c.bg_secondary}; border-radius: 8px; display: flex; gap: 12px; align-items: flex-start;",
                span {
                    style: "font-size: 18px; color: {c.accent_blue};",
                    "[i]"
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    p {
                        style: "font-size: 13px; color: {c.text_primary}; font-weight: 500;",
                        "Moteur de regles"
                    }
                    p {
                        style: "font-size: 12px; color: {c.text_secondary};",
                        "Le moteur d'automatisations utilisera un systeme de regles declaratif : "
                        "declencheur (horaire, capteur, commande vocale) + conditions optionnelles + actions. "
                        "Tout sera execute localement via le COG, sans cloud."
                    }
                }
            }
        }
    }
}
