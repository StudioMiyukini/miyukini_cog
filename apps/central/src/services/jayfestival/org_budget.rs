//! ORG-E19 — Budget d'une édition (revenus, dépenses, balance).
//!
//! @id: jf_org_budget @do: render_org_budget
//! @role: ui @layer: service
//! @human: Ecran ORG-E19 JayFestival: tableau budgetaire edition (revenus, depenses, balance).

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{Badge, StatCard};

#[component]
pub fn OrgBudget(edition_id: String) -> Element {
    let p = use_palette();
    let conns = use_service_connections();
    let mut show_form = use_signal(|| false);
    let mut new_label = use_signal(String::new);
    let mut new_category = use_signal(|| "autre".to_string());
    let mut new_amount = use_signal(|| "0".to_string());
    let mut new_entry_type = use_signal(|| "depense".to_string());
    let mut new_date = use_signal(String::new);
    let mut new_notes = use_signal(String::new);

    // Charger les entrées budgétaires
    let entries = {
        let db = &conns.read().jayfestival;
        db.budget_entries_by_edition(&edition_id).unwrap_or_default()
    };

    let summary = {
        let db = &conns.read().jayfestival;
        db.budget_summary(&edition_id).unwrap_or_default()
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            // Stats budget
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                StatCard {
                    label: "Revenus".to_string(),
                    value: format!("{:.2} EUR", summary.total_revenus),
                    icon: "💰".to_string(),
                    color: p.success.to_string(),
                }
                StatCard {
                    label: "Depenses".to_string(),
                    value: format!("{:.2} EUR", summary.total_depenses),
                    icon: "📉".to_string(),
                    color: p.warning.to_string(),
                }
                StatCard {
                    label: "Balance".to_string(),
                    value: format!("{:.2} EUR", summary.balance),
                    icon: "📊".to_string(),
                    color: if summary.balance >= 0.0 { p.success.to_string() } else { p.warning.to_string() },
                }
            }

            // En-tête liste
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h3 {
                    style: "font-size: 18px; color: {p.text_high};",
                    "Ecritures budgetaires ({entries.len()})"
                }

                button {
                    style: "padding: 8px 16px; background: {p.accent_primary}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 13px;",
                    onclick: move |_| show_form.set(!show_form()),
                    "➕ Nouvelle ecriture"
                }
            }

            // Formulaire d'ajout
            if *show_form.read() {
                div {
                    style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px; border: 1px solid {p.border_default};",

                    div {
                        style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-bottom: 16px;",

                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 4px;",
                                "Libelle *"
                            }
                            input {
                                style: "width: 100%; padding: 8px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_high}; font-size: 13px; box-sizing: border-box;",
                                r#type: "text",
                                placeholder: "Ex: Location salle",
                                value: "{new_label}",
                                oninput: move |evt| new_label.set(evt.value()),
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 4px;",
                                "Montant (EUR) *"
                            }
                            input {
                                style: "width: 100%; padding: 8px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_high}; font-size: 13px; box-sizing: border-box;",
                                r#type: "number",
                                step: "0.01",
                                value: "{new_amount}",
                                oninput: move |evt| new_amount.set(evt.value()),
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 4px;",
                                "Type"
                            }
                            select {
                                style: "width: 100%; padding: 8px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_high}; font-size: 13px; box-sizing: border-box;",
                                value: "{new_entry_type}",
                                onchange: move |evt| new_entry_type.set(evt.value()),

                                option { value: "depense", "Depense" }
                                option { value: "revenu", "Revenu" }
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 4px;",
                                "Categorie"
                            }
                            select {
                                style: "width: 100%; padding: 8px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_high}; font-size: 13px; box-sizing: border-box;",
                                value: "{new_category}",
                                onchange: move |evt| new_category.set(evt.value()),

                                option { value: "logistique", "Logistique" }
                                option { value: "communication", "Communication" }
                                option { value: "location", "Location" }
                                option { value: "personnel", "Personnel" }
                                option { value: "sponsor", "Sponsor" }
                                option { value: "billetterie", "Billetterie" }
                                option { value: "autre", "Autre" }
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 4px;",
                                "Date"
                            }
                            input {
                                style: "width: 100%; padding: 8px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_high}; font-size: 13px; box-sizing: border-box;",
                                r#type: "date",
                                value: "{new_date}",
                                oninput: move |evt| new_date.set(evt.value()),
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 4px;",
                                "Notes"
                            }
                            input {
                                style: "width: 100%; padding: 8px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_high}; font-size: 13px; box-sizing: border-box;",
                                r#type: "text",
                                placeholder: "Notes optionnelles",
                                value: "{new_notes}",
                                oninput: move |evt| new_notes.set(evt.value()),
                            }
                        }
                    }

                    div {
                        style: "display: flex; gap: 8px; justify-content: flex-end;",

                        button {
                            style: "padding: 8px 16px; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_secondary}; cursor: pointer; font-size: 13px;",
                            onclick: move |_| show_form.set(false),
                            "Annuler"
                        }
                        button {
                            style: "padding: 8px 16px; background: {p.accent_primary}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 13px;",
                            onclick: {
                                let eid = edition_id.clone();
                                move |_| {
                                    let label = new_label.read().clone();
                                    if !label.is_empty() {
                                        let amount: f64 = new_amount.read().parse().unwrap_or(0.0);
                                        let entry_type = new_entry_type.read().clone();
                                        let category = new_category.read().clone();
                                        let date = { let v = new_date.read().clone(); if v.is_empty() { None } else { Some(v) } };
                                        let notes = { let v = new_notes.read().clone(); if v.is_empty() { None } else { Some(v) } };
                                        let db = &conns.read().jayfestival;
                                        let _ = db.budget_entry_create(
                                            &eid,
                                            &label,
                                            Some(category.as_str()),
                                            amount,
                                            &entry_type,
                                            date.as_deref(),
                                            notes.as_deref(),
                                        );
                                        show_form.set(false);
                                        new_label.set(String::new());
                                        new_amount.set("0".to_string());
                                        new_notes.set(String::new());
                                    }
                                }
                            },
                            "Ajouter"
                        }
                    }
                }
            }

            // Liste des entrées
            if entries.is_empty() {
                div {
                    style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 200px; color: {p.text_muted}; background: {p.bg_secondary}; border-radius: 8px;",

                    span { style: "font-size: 48px; margin-bottom: 12px; opacity: 0.3;", "💰" }
                    p { style: "font-size: 14px;", "Aucune ecriture budgetaire" }
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 6px;",

                    for entry in entries.iter() {
                        {
                            let label = entry.label.clone().unwrap_or_else(|| "Sans libelle".to_string());
                            let amount = entry.amount.unwrap_or(0.0);
                            let entry_type = entry.entry_type.clone().unwrap_or_default();
                            let category = entry.category.clone().unwrap_or_else(|| "autre".to_string());
                            let date = entry.date.clone().unwrap_or_default();
                            let is_revenue = entry_type == "revenu";
                            let color = if is_revenue { p.success } else { p.warning };
                            let sign = if is_revenue { "+" } else { "-" };
                            rsx! {
                                div {
                                    style: "display: flex; align-items: center; gap: 12px; background: {p.bg_secondary}; border-radius: 6px; padding: 10px 16px;",

                                    div {
                                        style: "flex: 1;",
                                        span {
                                            style: "color: {p.text_high}; font-size: 13px;",
                                            "{label}"
                                        }
                                        if !date.is_empty() {
                                            span {
                                                style: "color: {p.text_muted}; font-size: 11px; margin-left: 8px;",
                                                "{date}"
                                            }
                                        }
                                    }

                                    Badge {
                                        text: category.clone(),
                                        color: p.text_secondary.to_string(),
                                    }

                                    span {
                                        style: "font-weight: 600; font-size: 14px; color: {color}; min-width: 100px; text-align: right;",
                                        "{sign}{amount:.2} EUR"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
