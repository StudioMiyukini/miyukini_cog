//! Dashboard Purse — P1.
//!
//! Composants : SoldeCard, DepensesBudgetCards, CategorieChart,
//! MouvementsList (5 derniers), ObjectifsResume, BudgetsResume, QuickAddButton.
//! Contrats : CK-OP-01, CK-TK-51.
//! Performance : < 3s (NFR-PUR-04).

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::{JayKontaState, PurseSection};
use super::components::{KpiCard, MovementRow, ProgressBar, ActionButton};

#[component]
pub fn PurseDashboard(state: Signal<JayKontaState>) -> Element {
    let p = use_palette();
    let conns = use_service_connections();

    // Charger les donnees reelles
    let db = &conns.read().jaykonta;
    let solde = db.solde_purse().unwrap_or(0.0);
    let stats = db.purse_stats().unwrap_or_default();
    let movements = db.movements_recent(5).unwrap_or_default();

    // Formatage
    let solde_str = format!("{solde:.2} EUR");
    let expenses_str = format!("{:.2} EUR", stats.monthly_expenses);
    let budget_pct = format!("{:.0}%", stats.budget_usage_pct);
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Titre
            h2 {
                style: "font-size: 22px; color: {p.text_high};",
                "Tableau de bord — Purse"
            }

            // Solde principal
            div {
                style: "background: {p.bg_secondary}; border-radius: 12px; padding: 24px;",

                p {
                    style: "font-size: 13px; color: {p.text_secondary}; text-transform: uppercase; letter-spacing: 1px; margin-bottom: 8px;",
                    "SOLDE ACTUEL"
                }
                p {
                    style: "font-size: 36px; font-weight: 700; color: {p.text_high};",
                    "{solde_str}"
                }
            }

            // KPI Grid
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                KpiCard {
                    label: "Depenses ce mois".to_string(),
                    value: expenses_str,
                    detail: format!("{budget_pct} du budget"),
                    icon: "📉".to_string(),
                    positive: false,
                }
                KpiCard {
                    label: "Objectifs actifs".to_string(),
                    value: stats.active_goals.to_string(),
                    detail: "en cours".to_string(),
                    icon: "🏆".to_string(),
                    positive: true,
                }
                KpiCard {
                    label: "Mouvements classes".to_string(),
                    value: format!("{:.0}%", stats.categorized_pct),
                    detail: "avec categorie".to_string(),
                    icon: "📂".to_string(),
                    positive: stats.categorized_pct > 80.0,
                }
            }

            // Repartition par categorie (placeholder simplifie)
            div {
                style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                    "Repartition par categorie"
                }
                p {
                    style: "font-size: 13px; color: {p.text_muted}; text-align: center; padding: 24px;",
                    "Budget utilise : {budget_pct}"
                }
                ProgressBar {
                    value: stats.budget_usage_pct,
                    max: 100.0,
                }
            }

            // Derniers mouvements
            div {
                style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px;",

                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;",

                    h3 {
                        style: "font-size: 16px; color: {p.text_high};",
                        "Derniers mouvements"
                    }
                }

                if movements.is_empty() {
                    p {
                        style: "font-size: 13px; color: {p.text_muted}; text-align: center; padding: 32px;",
                        "Aucun mouvement enregistre. Commencez par ajouter une depense ou un revenu."
                    }
                } else {
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        for m in movements.iter() {
                            MovementRow {
                                description: if m.category.is_empty() { m.context_ref.clone() } else { m.category.clone() },
                                date: m.movement_date.clone(),
                                category: m.source_service.clone(),
                                amount: m.amount,
                                currency: m.currency.clone(),
                            }
                        }
                    }
                }

                button {
                    style: "margin-top: 16px; padding: 8px 16px; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_secondary}; cursor: pointer; font-size: 12px; width: 100%;",
                    onclick: move |_| {
                        state.write().purse_section = PurseSection::Mouvements;
                    },
                    "Voir tous les mouvements →"
                }
            }

            // Bouton ajout rapide
            div {
                style: "display: flex; justify-content: center;",

                ActionButton {
                    label: "+ Nouveau mouvement".to_string(),
                    icon: String::new(),
                    accent: true,
                    onclick: move |_| {
                        state.write().purse_section = PurseSection::NouveauMouvement;
                    },
                }
            }
        }
    }
}
