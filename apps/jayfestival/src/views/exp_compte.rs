//! EXP-E13 — Mon compte exposant.

use super::components::ActionButton;
use dioxus::prelude::*;
use miyukini_service_ui::use_palette;

/// Page de gestion du compte exposant.
#[component]
pub fn ExpCompte() -> Element {
    let c = use_palette();
    // Charger les données exposant
    let exposant = {
        let db = crate::use_db();
        db.exposants_list(false)
            .unwrap_or_default()
            .into_iter()
            .next()
    };

    let company_name = exposant
        .as_ref()
        .and_then(|e| e.company_name.clone())
        .unwrap_or_default();
    let email = exposant
        .as_ref()
        .and_then(|e| e.contact_email.clone())
        .unwrap_or_default();
    let phone = exposant
        .as_ref()
        .and_then(|e| e.contact_phone.clone())
        .unwrap_or_default();
    let siret = exposant
        .as_ref()
        .and_then(|e| e.siret.clone())
        .unwrap_or_default();
    let address = exposant
        .as_ref()
        .and_then(|e| e.adresse.clone())
        .unwrap_or_default();

    let company_input = use_signal(|| company_name.clone());
    let email_input = use_signal(|| email.clone());
    let phone_input = use_signal(|| phone.clone());
    let siret_input = use_signal(|| siret.clone());
    let mut address_input = use_signal(|| address.clone());

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            h2 {
                style: "font-size: 24px; color: {c.text_white};",
                "Mon compte"
            }

            // Informations générales
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Informations de l'entreprise"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                    FormField {
                        label: "Nom de l'entreprise",
                        value: company_input,
                    }
                    FormField {
                        label: "SIRET",
                        value: siret_input,
                    }
                    FormField {
                        label: "Email de contact",
                        value: email_input,
                    }
                    FormField {
                        label: "Telephone",
                        value: phone_input,
                    }
                }

                div {
                    style: "margin-top: 16px;",
                    label {
                        style: "font-size: 12px; color: {c.text_muted}; display: block; margin-bottom: 4px;",
                        "Adresse"
                    }
                    textarea {
                        style: "width: 100%; padding: 10px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; font-size: 13px; resize: vertical; min-height: 80px;",
                        value: "{address_input}",
                        oninput: move |evt| { address_input.set(evt.value()); },
                    }
                }

                div {
                    style: "margin-top: 16px; display: flex; justify-content: flex-end;",
                    ActionButton {
                        label: "Enregistrer".to_string(),
                        icon: "💾".to_string(),
                        accent: true,
                        onclick: move |_| {
                            // TODO: Sauvegarder les modifications
                        },
                    }
                }
            }

            // Logo et visuels
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Logo et visuels"
                }

                div {
                    style: "display: flex; gap: 24px;",

                    // Logo actuel
                    div {
                        style: "text-align: center;",
                        div {
                            style: "width: 120px; height: 120px; background: {c.bg_main}; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 48px; margin-bottom: 12px;",
                            "🏪"
                        }
                        button {
                            style: "padding: 8px 16px; background: {c.bg_hover}; border: none; border-radius: 6px; color: {c.text_primary}; cursor: pointer; font-size: 13px;",
                            "Changer le logo"
                        }
                    }

                    // Info format
                    div {
                        style: "flex: 1; padding: 16px; background: {c.bg_main}; border-radius: 8px;",
                        p {
                            style: "font-size: 13px; color: {c.text_primary}; margin-bottom: 8px;",
                            "Formats acceptes : PNG, JPG, SVG"
                        }
                        p {
                            style: "font-size: 12px; color: {c.text_muted};",
                            "Taille recommandee : 512x512 pixels minimum"
                        }
                        p {
                            style: "font-size: 12px; color: {c.text_muted};",
                            "Poids maximum : 2 Mo"
                        }
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
                        icon: "🔑",
                        title: "Mot de passe",
                        description: "Derniere modification il y a 30 jours",
                        action: "Modifier",
                    }
                    SecurityRow {
                        icon: "📱",
                        title: "Authentification a deux facteurs",
                        description: "Non active",
                        action: "Activer",
                    }
                    SecurityRow {
                        icon: "📧",
                        title: "Notifications email",
                        description: "Activees pour les candidatures et factures",
                        action: "Configurer",
                    }
                }
            }

            // Zone danger
            section {
                style: "background: {c.bg_secondary}; border: 1px solid {c.accent_red}40; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {c.accent_red}; margin-bottom: 16px;",
                    "Zone dangereuse"
                }

                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",

                    div {
                        p {
                            style: "font-size: 14px; color: {c.text_white}; margin-bottom: 4px;",
                            "Supprimer mon compte"
                        }
                        p {
                            style: "font-size: 12px; color: {c.text_muted};",
                            "Cette action est irreversible. Toutes vos donnees seront supprimees."
                        }
                    }

                    button {
                        style: "padding: 8px 16px; background: transparent; border: 1px solid {c.accent_red}; border-radius: 6px; color: {c.accent_red}; cursor: pointer; font-size: 13px;",
                        "Supprimer"
                    }
                }
            }
        }
    }
}

#[component]
fn FormField(label: &'static str, value: Signal<String>) -> Element {
    let c = use_palette();
    let val = value.read().clone();

    rsx! {
        div {
            label {
                style: "font-size: 12px; color: {c.text_muted}; display: block; margin-bottom: 4px;",
                "{label}"
            }
            input {
                r#type: "text",
                style: "width: 100%; padding: 10px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; font-size: 13px;",
                value: "{val}",
                oninput: move |evt| { value.set(evt.value()); },
            }
        }
    }
}

#[component]
fn SecurityRow(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    action: &'static str,
) -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 12px; background: {c.bg_main}; border-radius: 6px;",

            span { style: "font-size: 20px;", "{icon}" }

            div {
                style: "flex: 1;",
                p {
                    style: "font-size: 14px; color: {c.text_white}; margin-bottom: 2px;",
                    "{title}"
                }
                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "{description}"
                }
            }

            button {
                style: "padding: 6px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                "{action}"
            }
        }
    }
}
