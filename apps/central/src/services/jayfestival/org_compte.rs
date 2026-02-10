//! ORG-E20 — Mon compte organisateur.

use dioxus::prelude::*;
use crate::state::use_app_state;
use super::components::ActionButton;

/// Page compte organisateur.
#[component]
pub fn OrgCompte() -> Element {
    let c = use_app_state().read().current_theme.palette();

    let mut name = use_signal(|| "Mon Organisation".to_string());
    let mut email = use_signal(|| "contact@organisation.fr".to_string());
    let mut phone = use_signal(|| "01 23 45 67 89".to_string());
    let mut siret = use_signal(|| "123 456 789 00012".to_string());

    rsx! {
        div {
            style: "max-width: 800px;",

            h2 {
                style: "font-size: 24px; color: {c.text_white}; margin-bottom: 24px;",
                "Mon compte organisateur"
            }

            // Informations de base
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 24px; margin-bottom: 24px;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Informations generales"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                    div {
                        label {
                            style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                            "Nom de l'organisation"
                        }
                        input {
                            r#type: "text",
                            style: "width: 100%; padding: 10px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; font-size: 14px;",
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value()),
                        }
                    }

                    div {
                        label {
                            style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                            "Email de contact"
                        }
                        input {
                            r#type: "email",
                            style: "width: 100%; padding: 10px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; font-size: 14px;",
                            value: "{email}",
                            oninput: move |evt| email.set(evt.value()),
                        }
                    }

                    div {
                        label {
                            style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                            "Telephone"
                        }
                        input {
                            r#type: "tel",
                            style: "width: 100%; padding: 10px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; font-size: 14px;",
                            value: "{phone}",
                            oninput: move |evt| phone.set(evt.value()),
                        }
                    }

                    div {
                        label {
                            style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                            "SIRET"
                        }
                        input {
                            r#type: "text",
                            style: "width: 100%; padding: 10px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; font-size: 14px;",
                            value: "{siret}",
                            oninput: move |evt| siret.set(evt.value()),
                        }
                    }
                }

                div {
                    style: "margin-top: 16px;",

                    ActionButton {
                        label: "Enregistrer".to_string(),
                        icon: "💾".to_string(),
                        accent: true,
                        onclick: move |_| {},
                    }
                }
            }

            // Abonnement
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 24px; margin-bottom: 24px;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Abonnement"
                }

                div {
                    style: "display: flex; justify-content: space-between; align-items: center; padding: 16px; background: {c.bg_main}; border-radius: 8px; border: 2px solid {c.accent_blue};",

                    div {
                        div {
                            style: "display: flex; align-items: center; gap: 8px;",
                            span {
                                style: "font-size: 24px;",
                                "⭐"
                            }
                            h4 {
                                style: "font-size: 16px; color: {c.text_white};",
                                "Plan Pro"
                            }
                        }
                        p {
                            style: "font-size: 13px; color: {c.text_muted}; margin-top: 4px;",
                            "Jusqu'a 10 evenements par an • Support prioritaire"
                        }
                    }

                    div {
                        style: "text-align: right;",
                        p {
                            style: "font-size: 24px; color: {c.text_white}; font-weight: 600;",
                            "49€/mois"
                        }
                        button {
                            style: "padding: 8px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_muted}; cursor: pointer; font-size: 12px; margin-top: 4px;",
                            "Changer de plan"
                        }
                    }
                }
            }

            // Sécurité
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 24px; margin-bottom: 24px;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Securite"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    div {
                        style: "display: flex; justify-content: space-between; align-items: center; padding: 12px 0; border-bottom: 1px solid {c.border};",

                        div {
                            p {
                                style: "font-size: 14px; color: {c.text_primary};",
                                "Mot de passe"
                            }
                            p {
                                style: "font-size: 12px; color: {c.text_muted};",
                                "Derniere modification : il y a 30 jours"
                            }
                        }
                        button {
                            style: "padding: 8px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                            "Modifier"
                        }
                    }

                    div {
                        style: "display: flex; justify-content: space-between; align-items: center; padding: 12px 0; border-bottom: 1px solid {c.border};",

                        div {
                            p {
                                style: "font-size: 14px; color: {c.text_primary};",
                                "Authentification a deux facteurs"
                            }
                            p {
                                style: "font-size: 12px; color: {c.text_muted};",
                                "Non activee"
                            }
                        }
                        button {
                            style: "padding: 8px 12px; background: {c.accent_blue}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 12px;",
                            "Activer"
                        }
                    }

                    div {
                        style: "display: flex; justify-content: space-between; align-items: center; padding: 12px 0;",

                        div {
                            p {
                                style: "font-size: 14px; color: {c.text_primary};",
                                "Sessions actives"
                            }
                            p {
                                style: "font-size: 12px; color: {c.text_muted};",
                                "2 appareils connectes"
                            }
                        }
                        button {
                            style: "padding: 8px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_muted}; cursor: pointer; font-size: 12px;",
                            "Voir les sessions"
                        }
                    }
                }
            }

            // Zone danger
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 24px; border: 1px solid {c.accent_red}30;",

                h3 {
                    style: "font-size: 16px; color: {c.accent_red}; margin-bottom: 16px;",
                    "Zone de danger"
                }

                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",

                    div {
                        p {
                            style: "font-size: 14px; color: {c.text_primary};",
                            "Supprimer le compte"
                        }
                        p {
                            style: "font-size: 12px; color: {c.text_muted};",
                            "Cette action est irreversible"
                        }
                    }
                    button {
                        style: "padding: 8px 12px; background: transparent; border: 1px solid {c.accent_red}; border-radius: 4px; color: {c.accent_red}; cursor: pointer; font-size: 12px;",
                        "Supprimer"
                    }
                }
            }
        }
    }
}
