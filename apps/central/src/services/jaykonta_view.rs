//! Vue du service JayKonta — donnees reelles via JayKontaDb.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;

#[component]
pub fn JayKontaView() -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
    let conns = use_service_connections();

    // Charger les stats reelles
    let db = &conns.read().jaykonta;
    let solde = db.solde_purse().unwrap_or(0.0);
    let account = db.account_stats().unwrap_or_default();
    let purse = db.purse_stats().unwrap_or_default();
    let movements = db.movements_recent(10).unwrap_or_default();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Header
            h2 {
                style: "font-size: 24px; color: {c.text_white};",
                "🧮 JayKonta — Comptabilite COG"
            }

            // Resume financier
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                FinanceCard {
                    label: "Solde Purse".to_string(),
                    value: format!("{solde:.2} EUR"),
                    detail: format!("Budget: {:.0}%", purse.budget_usage_pct),
                    positive: solde >= 0.0,
                    icon: "💰".to_string(),
                }
                FinanceCard {
                    label: "CA mensuel (Account)".to_string(),
                    value: format!("{:.2} EUR", account.ca_mensuel),
                    detail: format!("{} factures", account.invoices_count),
                    positive: true,
                    icon: "📈".to_string(),
                }
                FinanceCard {
                    label: "Depenses (mois)".to_string(),
                    value: format!("{:.2} EUR", purse.monthly_expenses),
                    detail: format!("Impayes: {:.2} EUR", account.impayes),
                    positive: false,
                    icon: "📉".to_string(),
                }
            }

            // Sections
            div {
                style: "display: grid; grid-template-columns: 2fr 1fr; gap: 24px;",

                // Derniers mouvements
                div {
                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                    h3 {
                        style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                        "Derniers mouvements"
                    }

                    if movements.is_empty() {
                        p {
                            style: "font-size: 13px; color: {c.text_muted}; text-align: center; padding: 32px;",
                            "Aucun mouvement enregistre"
                        }
                    } else {
                        div {
                            style: "display: flex; flex-direction: column; gap: 12px;",

                            for m in movements.iter() {
                                TransactionRow {
                                    description: format!("{} — {}", m.category, m.context_ref),
                                    date: m.movement_date.clone(),
                                    amount: format!("{:+.2} {}", m.amount, m.currency),
                                    positive: m.amount >= 0.0,
                                }
                            }
                        }
                    }

                    button {
                        style: "margin-top: 16px; padding: 8px 16px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 12px; width: 100%;",
                        "Voir tous les mouvements →"
                    }
                }

                // Actions rapides
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    h3 {
                        style: "font-size: 16px; color: {c.text_white}; margin-bottom: 4px;",
                        "Actions"
                    }

                    QuickAction { icon: "➕", label: "Nouvelle depense" }
                    QuickAction { icon: "📄", label: "Creer une facture" }
                    QuickAction { icon: "📊", label: "Generer rapport" }
                    QuickAction { icon: "⬇️", label: "Importer releve" }
                }
            }
        }
    }
}

#[component]
fn FinanceCard(
    label: String,
    value: String,
    detail: String,
    positive: bool,
    icon: String,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let detail_color = if positive { c.accent_green } else { c.accent_red };

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

            div {
                style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 12px;",

                span {
                    style: "font-size: 13px; color: {c.text_secondary};",
                    "{label}"
                }
                span {
                    style: "font-size: 20px;",
                    "{icon}"
                }
            }

            p {
                style: "font-size: 28px; font-weight: 600; color: {c.text_white}; margin-bottom: 4px;",
                "{value}"
            }
            span {
                style: "font-size: 12px; color: {detail_color};",
                "{detail}"
            }
        }
    }
}

#[component]
fn TransactionRow(
    description: String,
    date: String,
    amount: String,
    positive: bool,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let amount_color = if positive { c.accent_green } else { c.accent_red };

    rsx! {
        div {
            style: "display: flex; justify-content: space-between; align-items: center; padding: 12px; background: {c.bg_hover}; border-radius: 4px;",

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
                style: "font-size: 14px; font-weight: 500; color: {amount_color};",
                "{amount}"
            }
        }
    }
}

#[component]
fn QuickAction(icon: &'static str, label: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        button {
            style: "display: flex; align-items: center; gap: 12px; padding: 14px 16px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px; color: {c.text_primary}; cursor: pointer; font-size: 13px; text-align: left; transition: background 0.2s;",

            span { "{icon}" }
            span { "{label}" }
        }
    }
}
