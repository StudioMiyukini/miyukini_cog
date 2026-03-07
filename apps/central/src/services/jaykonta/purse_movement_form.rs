//! Formulaire de mouvement rapide Purse — P2.
//!
//! Objectif : saisie complete en < 2s, max 3 actions (NFR-PUR-05, NFR-PUR-06).
//! Flux : montant → categorie → valider.
//! Contrats : CK-OP-02, CK-TK-11, CK-AUD-01.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::{JayKontaState, PurseSection};

/// Categories rapides pour saisie express.
const QUICK_CATEGORIES: &[(&str, &str)] = &[
    ("Alimentation", "🍕"),
    ("Transport", "🚌"),
    ("Logement", "🏠"),
    ("Loisirs", "🎮"),
    ("Sante", "🏥"),
    ("Vetements", "👕"),
    ("Technologie", "📱"),
    ("Divers", "📦"),
];

#[component]
pub fn PurseMovementForm(state: Signal<JayKontaState>) -> Element {
    let p = use_palette();
    let conns = use_service_connections();

    // State du formulaire
    let mut is_expense = use_signal(|| true);
    let mut amount_str = use_signal(String::new);
    let mut selected_category = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut date_str = use_signal(|| chrono::Local::now().format("%Y-%m-%d").to_string());
    let mut submitted = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    rsx! {
        div {
            style: "max-width: 600px; margin: 0 auto;",

            // Header
            div {
                style: "display: flex; align-items: center; gap: 16px; margin-bottom: 24px;",

                button {
                    style: "padding: 8px 12px; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_secondary}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| {
                        state.write().purse_section = PurseSection::Dashboard;
                    },
                    "← Retour"
                }
                h2 {
                    style: "font-size: 20px; color: {p.text_high};",
                    "Nouveau mouvement"
                }
            }

            // Type toggle
            div {
                style: "display: flex; gap: 8px; margin-bottom: 20px;",

                TypeButton {
                    label: "Depense",
                    is_active: *is_expense.read(),
                    color: p.error.to_string(),
                    onclick: move |_| { is_expense.set(true); },
                }
                TypeButton {
                    label: "Revenu",
                    is_active: !*is_expense.read(),
                    color: p.success.to_string(),
                    onclick: move |_| { is_expense.set(false); },
                }
            }

            // Montant
            div {
                style: "margin-bottom: 20px;",

                label {
                    style: "display: block; font-size: 13px; color: {p.text_secondary}; margin-bottom: 6px;",
                    "Montant *"
                }
                input {
                    style: "width: 100%; padding: 14px 16px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 8px; color: {p.text_high}; font-size: 24px; font-weight: 600; text-align: center;",
                    r#type: "number",
                    step: "0.01",
                    min: "0.01",
                    placeholder: "0,00",
                    value: "{amount_str}",
                    oninput: move |evt| amount_str.set(evt.value()),
                    autofocus: true,
                }
                span {
                    style: "display: block; text-align: center; font-size: 13px; color: {p.text_muted}; margin-top: 4px;",
                    "EUR"
                }
            }

            // Categories rapides
            div {
                style: "margin-bottom: 20px;",

                label {
                    style: "display: block; font-size: 13px; color: {p.text_secondary}; margin-bottom: 8px;",
                    "Categorie *"
                }
                div {
                    style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 8px;",

                    for (name, icon) in QUICK_CATEGORIES.iter() {
                        {
                            let cat_name = name.to_string();
                            let is_selected = *selected_category.read() == cat_name;
                            let bg = if is_selected { p.accent_primary.to_string() } else { p.bg_secondary.to_string() };
                            let color = if is_selected { "white".to_string() } else { p.text_secondary.to_string() };
                            let border_style = if is_selected { format!("2px solid {}", p.accent_primary) } else { format!("1px solid {}", p.border_default) };

                            rsx! {
                                button {
                                    style: "display: flex; flex-direction: column; align-items: center; gap: 4px; padding: 12px 8px; background: {bg}; border: {border_style}; border-radius: 8px; cursor: pointer; color: {color}; font-size: 11px; transition: all 0.2s;",
                                    onclick: move |_| {
                                        selected_category.set(cat_name.clone());
                                    },
                                    span { style: "font-size: 20px;", "{icon}" }
                                    span { "{name}" }
                                }
                            }
                        }
                    }
                }
            }

            // Description
            div {
                style: "margin-bottom: 20px;",

                label {
                    style: "display: block; font-size: 13px; color: {p.text_secondary}; margin-bottom: 6px;",
                    "Description (optionnel)"
                }
                input {
                    style: "width: 100%; padding: 10px 14px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_high}; font-size: 14px;",
                    placeholder: "Ex: Courses Carrefour",
                    value: "{description}",
                    oninput: move |evt| description.set(evt.value()),
                }
            }

            // Date
            div {
                style: "margin-bottom: 24px;",

                label {
                    style: "display: block; font-size: 13px; color: {p.text_secondary}; margin-bottom: 6px;",
                    "Date"
                }
                input {
                    style: "width: 100%; padding: 10px 14px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_high}; font-size: 14px;",
                    r#type: "date",
                    value: "{date_str}",
                    oninput: move |evt| date_str.set(evt.value()),
                }
            }

            // Erreur
            if let Some(err) = &*error_msg.read() {
                p {
                    style: "color: {p.error}; font-size: 13px; margin-bottom: 12px; text-align: center;",
                    "{err}"
                }
            }

            // Message succes
            if *submitted.read() {
                div {
                    style: "background: {p.success}20; border: 1px solid {p.success}; border-radius: 8px; padding: 16px; text-align: center; margin-bottom: 16px;",

                    p {
                        style: "color: {p.success}; font-size: 14px; font-weight: 500;",
                        "Mouvement enregistre avec succes !"
                    }
                }
            }

            // Bouton valider
            button {
                style: "width: 100%; padding: 14px; background: {p.accent_primary}; border: none; border-radius: 8px; color: white; font-size: 16px; font-weight: 600; cursor: pointer; transition: opacity 0.2s;",
                onclick: move |_| {
                    // Validation
                    let amount_val: f64 = amount_str.read().parse().unwrap_or(0.0);
                    if amount_val <= 0.0 {
                        error_msg.set(Some("Veuillez saisir un montant valide.".to_string()));
                        return;
                    }
                    if selected_category.read().is_empty() {
                        error_msg.set(Some("Veuillez selectionner une categorie.".to_string()));
                        return;
                    }

                    // Calcul montant signe
                    let signed_amount = if *is_expense.read() { -amount_val } else { amount_val };

                    // Insertion via DB directe (pas de PurseService ici car pas d'account_id contextuel)
                    let db = &conns.read().jaykonta;
                    let id = uuid::Uuid::new_v4().to_string();
                    let _desc = if description.read().is_empty() { None } else { Some(description.read().clone()) };
                    let result = db.insert_movement(&jaykonta::data::MovementRecord {
                        id,
                        scope: "purse".to_string(),
                        context_ref: selected_category.read().clone(),
                        category: selected_category.read().clone(),
                        amount: signed_amount,
                        currency: "EUR".to_string(),
                        movement_date: date_str.read().clone(),
                        source_service: "manual".to_string(),
                    });

                    match result {
                        Ok(()) => {
                            error_msg.set(None);
                            submitted.set(true);
                            // Reset form
                            amount_str.set(String::new());
                            selected_category.set(String::new());
                            description.set(String::new());
                        }
                        Err(e) => {
                            error_msg.set(Some(format!("Erreur: {e}")));
                        }
                    }
                },
                "Enregistrer"
            }

            // Indication performance
            p {
                style: "text-align: center; font-size: 11px; color: {p.text_muted}; margin-top: 12px;",
                "Saisie rapide : montant → categorie → valider"
            }
        }
    }
}

#[component]
fn TypeButton(label: &'static str, is_active: bool, color: String, onclick: EventHandler<MouseEvent>) -> Element {
    let p = use_palette();
    let bg = if is_active { format!("{color}20") } else { p.bg_secondary.to_string() };
    let text_color = if is_active { &color } else { p.text_secondary };
    let border = if is_active { format!("2px solid {color}") } else { format!("1px solid {}", p.border_default) };

    rsx! {
        button {
            style: "flex: 1; padding: 12px; background: {bg}; border: {border}; border-radius: 8px; color: {text_color}; cursor: pointer; font-size: 14px; font-weight: 500; transition: all 0.2s;",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}
