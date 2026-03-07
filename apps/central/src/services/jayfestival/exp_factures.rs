//! EXP-E11/E12 — Factures et paiements exposant.
//!
//! @id: jf_exp_factures @do: render_exp_factures
//! @role: ui @layer: service
//! @human: Ecrans EXP-E11/E12 JayFestival: factures et suivi des paiements exposant.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;
use super::components::{Badge, ActionButton, StatCard};

/// Vue des factures et paiements.
#[component]
pub fn ExpFactures() -> Element {
    let p = use_palette();
    let mut selected_facture = use_signal(|| None::<String>);

    // Si une facture est sélectionnée, afficher le détail
    if let Some(facture_id) = selected_facture.read().clone() {
        return rsx! {
            FactureDetail {
                facture_id: facture_id,
                on_back: move |()| { selected_facture.set(None); },
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            h2 {
                style: "font-size: 24px; color: {p.text_high};",
                "Factures et paiements"
            }

            // Stats
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                StatCard {
                    label: "Total facture".to_string(),
                    value: "1 250.00 €".to_string(),
                    icon: "💰".to_string(),
                    color: p.text_primary.to_string(),
                }
                StatCard {
                    label: "Paye".to_string(),
                    value: "850.00 €".to_string(),
                    icon: "✅".to_string(),
                    color: p.success.to_string(),
                }
                StatCard {
                    label: "En attente".to_string(),
                    value: "400.00 €".to_string(),
                    icon: "⏳".to_string(),
                    color: p.warning.to_string(),
                }
            }

            // Factures en attente
            section {
                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 12px;",
                    "Factures en attente de paiement"
                }

                FactureRow {
                    numero: "FA-2026-0058",
                    evenement: "Salon Printemps 2026",
                    montant: "400.00 €",
                    echeance: "28/02/2026",
                    status: "en_attente",
                    onclick: move |_| { selected_facture.set(Some("FA-2026-0058".to_string())); },
                }
            }

            // Historique des factures
            section {
                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 12px;",
                    "Historique"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    FactureRow {
                        numero: "FA-2026-0042",
                        evenement: "Salon Hiver 2026",
                        montant: "450.00 €",
                        echeance: "15/01/2026",
                        status: "payee",
                        onclick: move |_| { selected_facture.set(Some("FA-2026-0042".to_string())); },
                    }
                    FactureRow {
                        numero: "FA-2025-0198",
                        evenement: "Salon Automne 2025",
                        montant: "400.00 €",
                        echeance: "30/10/2025",
                        status: "payee",
                        onclick: move |_| { selected_facture.set(Some("FA-2025-0198".to_string())); },
                    }
                }
            }

            // Moyens de paiement
            PaymentMethods {}
        }
    }
}

#[component]
fn FactureRow(
    numero: &'static str,
    evenement: &'static str,
    montant: &'static str,
    echeance: &'static str,
    status: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();

    let (status_label, status_color) = match status {
        "payee" => ("Payee", p.success),
        "en_attente" => ("En attente", p.warning),
        "en_retard" => ("En retard", p.error),
        _ => ("Inconnu", p.text_muted),
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; background: {p.bg_secondary}; border-radius: 8px; padding: 16px; cursor: pointer;",
            onclick: move |evt| onclick.call(evt),

            div {
                style: "flex: 1;",
                p {
                    style: "font-size: 14px; color: {p.text_high}; margin-bottom: 4px;",
                    "{numero}"
                }
                p {
                    style: "font-size: 12px; color: {p.text_muted};",
                    "{evenement} • Echeance: {echeance}"
                }
            }

            span {
                style: "font-size: 16px; color: {p.text_high}; font-weight: 600;",
                "{montant}"
            }

            Badge {
                text: status_label.to_string(),
                color: status_color.to_string(),
            }

            if status == "en_attente" {
                ActionButton {
                    label: "Payer".to_string(),
                    icon: "💳".to_string(),
                    accent: true,
                    onclick: move |_| {},
                }
            }
        }
    }
}

/// Détail d'une facture.
#[component]
fn FactureDetail(facture_id: String, on_back: EventHandler<()>) -> Element {
    let p = use_palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Retour
            button {
                style: "display: flex; align-items: center; gap: 8px; padding: 8px 0; background: none; border: none; color: {p.accent_primary}; cursor: pointer; font-size: 13px;",
                onclick: move |_| on_back.call(()),
                "← Retour aux factures"
            }

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: flex-start;",

                div {
                    h2 {
                        style: "font-size: 24px; color: {p.text_high}; margin-bottom: 8px;",
                        "Facture {facture_id}"
                    }
                    p {
                        style: "font-size: 13px; color: {p.text_muted};",
                        "Emise le 10/01/2026 • Echeance le 28/02/2026"
                    }
                }

                div {
                    style: "display: flex; gap: 8px;",
                    ActionButton {
                        label: "Telecharger PDF".to_string(),
                        icon: "📄".to_string(),
                        accent: false,
                        onclick: move |_| {},
                    }
                    ActionButton {
                        label: "Payer maintenant".to_string(),
                        icon: "💳".to_string(),
                        accent: true,
                        onclick: move |_| {},
                    }
                }
            }

            // Détails
            section {
                style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                    "Details"
                }

                // Lignes de facture
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    InvoiceLine { description: "Stand standard 9m² - Salon Printemps 2026", quantity: "1", unit_price: "350.00 €", total: "350.00 €" }
                    InvoiceLine { description: "Option electricite renforcee", quantity: "1", unit_price: "50.00 €", total: "50.00 €" }

                    div {
                        style: "border-top: 1px solid {p.border_default}; padding-top: 12px; margin-top: 8px;",

                        div {
                            style: "display: flex; justify-content: space-between; margin-bottom: 8px;",
                            span { style: "font-size: 13px; color: {p.text_muted};", "Sous-total HT" }
                            span { style: "font-size: 13px; color: {p.text_primary};", "333.33 €" }
                        }
                        div {
                            style: "display: flex; justify-content: space-between; margin-bottom: 8px;",
                            span { style: "font-size: 13px; color: {p.text_muted};", "TVA 20%" }
                            span { style: "font-size: 13px; color: {p.text_primary};", "66.67 €" }
                        }
                        div {
                            style: "display: flex; justify-content: space-between;",
                            span { style: "font-size: 16px; color: {p.text_high}; font-weight: 600;", "Total TTC" }
                            span { style: "font-size: 16px; color: {p.text_high}; font-weight: 600;", "400.00 €" }
                        }
                    }
                }
            }

            // Infos organisateur
            section {
                style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                    "Organisateur"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                    div {
                        p { style: "font-size: 12px; color: {p.text_muted}; margin-bottom: 4px;", "Societe" }
                        p { style: "font-size: 14px; color: {p.text_primary};", "Events Pro SAS" }
                    }
                    div {
                        p { style: "font-size: 12px; color: {p.text_muted}; margin-bottom: 4px;", "SIRET" }
                        p { style: "font-size: 14px; color: {p.text_primary};", "123 456 789 00012" }
                    }
                    div {
                        p { style: "font-size: 12px; color: {p.text_muted}; margin-bottom: 4px;", "Adresse" }
                        p { style: "font-size: 14px; color: {p.text_primary};", "12 rue des Salons, 75001 Paris" }
                    }
                    div {
                        p { style: "font-size: 12px; color: {p.text_muted}; margin-bottom: 4px;", "Contact" }
                        p { style: "font-size: 14px; color: {p.text_primary};", "facturation@eventspro.fr" }
                    }
                }
            }
        }
    }
}

#[component]
fn InvoiceLine(description: &'static str, quantity: &'static str, unit_price: &'static str, total: &'static str) -> Element {
    let p = use_palette();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px;",

            span { style: "flex: 1; font-size: 13px; color: {p.text_primary};", "{description}" }
            span { style: "width: 60px; font-size: 13px; color: {p.text_muted}; text-align: center;", "x{quantity}" }
            span { style: "width: 80px; font-size: 13px; color: {p.text_muted}; text-align: right;", "{unit_price}" }
            span { style: "width: 80px; font-size: 13px; color: {p.text_primary}; text-align: right; font-weight: 500;", "{total}" }
        }
    }
}

#[component]
fn PaymentMethods() -> Element {
    let p = use_palette();

    rsx! {
        section {
            style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px;",

            h3 {
                style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                "Moyens de paiement acceptes"
            }

            div {
                style: "display: flex; gap: 16px;",

                PaymentMethod { icon: "💳", name: "Carte bancaire" }
                PaymentMethod { icon: "🏦", name: "Virement" }
                PaymentMethod { icon: "📧", name: "PayPal" }
            }
        }
    }
}

#[component]
fn PaymentMethod(icon: &'static str, name: &'static str) -> Element {
    let p = use_palette();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 8px; padding: 12px 16px; background: {p.bg_base}; border-radius: 6px;",

            span { style: "font-size: 20px;", "{icon}" }
            span { style: "font-size: 13px; color: {p.text_primary};", "{name}" }
        }
    }
}
