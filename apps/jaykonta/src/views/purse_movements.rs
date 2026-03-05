//! Liste des mouvements Purse — P2.
//!
//! Composants : FilterBar, MouvementsTable (paginee), TotauxSummary, Pagination.
//! Contrats : CK-TK-11.

use super::components::{ActionButton, MovementRow};
use super::{JayKontaState, PurseSection};
use dioxus::prelude::*;
use miyukini_service_ui::use_palette;

#[component]
pub fn PurseMovements(state: Signal<JayKontaState>) -> Element {
    let c = use_palette();
    let db = crate::use_db();

    // State filtres
    let mut filter_type = use_signal(|| "all".to_string()); // "all", "expense", "income"
    let mut page = use_signal(|| 0u32);
    let per_page = 10u32;

    // Charger mouvements
    let all_movements = db.movements_recent(200).unwrap_or_default();

    // Filtrer
    let filtered: Vec<_> = all_movements
        .iter()
        .filter(|m| {
            m.scope == "purse"
                && match filter_type.read().as_str() {
                    "expense" => m.amount < 0.0,
                    "income" => m.amount > 0.0,
                    _ => true,
                }
        })
        .collect();

    let total_count = filtered.len() as u32;
    let total_pages = total_count.div_ceil(per_page);
    let current_page = *page.read();
    let start = (current_page * per_page) as usize;
    let page_items: Vec<_> = filtered
        .iter()
        .skip(start)
        .take(per_page as usize)
        .copied()
        .collect();

    // Totaux
    let total_income: f64 = filtered
        .iter()
        .filter(|m| m.amount > 0.0)
        .map(|m| m.amount)
        .sum();
    let total_expense: f64 = filtered
        .iter()
        .filter(|m| m.amount < 0.0)
        .map(|m| m.amount.abs())
        .sum();
    let net = total_income - total_expense;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            // Header
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h2 {
                    style: "font-size: 22px; color: {c.text_white};",
                    "Mouvements"
                }
                ActionButton {
                    label: "+ Nouveau mouvement".to_string(),
                    icon: String::new(),
                    accent: true,
                    onclick: move |_| {
                        state.write().purse_section = PurseSection::NouveauMouvement;
                    },
                }
            }

            // Filtres
            div {
                style: "display: flex; gap: 8px; align-items: center;",

                FilterButton {
                    label: "Tous",
                    is_active: *filter_type.read() == "all",
                    onclick: move |_| { filter_type.set("all".to_string()); page.set(0); },
                }
                FilterButton {
                    label: "Depenses",
                    is_active: *filter_type.read() == "expense",
                    onclick: move |_| { filter_type.set("expense".to_string()); page.set(0); },
                }
                FilterButton {
                    label: "Revenus",
                    is_active: *filter_type.read() == "income",
                    onclick: move |_| { filter_type.set("income".to_string()); page.set(0); },
                }

                div { style: "flex: 1;" }

                span {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "{total_count} mouvements"
                }
            }

            // Tableau
            div {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px;",

                if page_items.is_empty() {
                    p {
                        style: "font-size: 13px; color: {c.text_muted}; text-align: center; padding: 40px;",
                        "Aucun mouvement pour les filtres selectionnes."
                    }
                } else {
                    // En-tete
                    div {
                        style: "display: flex; padding: 8px 16px; border-bottom: 1px solid {c.border}; margin-bottom: 8px;",

                        span { style: "flex: 1; font-size: 11px; color: {c.text_muted}; text-transform: uppercase;", "Description" }
                        span { style: "width: 100px; font-size: 11px; color: {c.text_muted}; text-transform: uppercase;", "Date" }
                        span { style: "width: 100px; font-size: 11px; color: {c.text_muted}; text-transform: uppercase; text-align: right;", "Montant" }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",

                        for m in page_items.iter() {
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
            }

            // Pagination
            if total_pages > 1 {
                div {
                    style: "display: flex; justify-content: center; align-items: center; gap: 16px;",

                    button {
                        style: "padding: 6px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 13px;",
                        disabled: current_page == 0,
                        onclick: move |_| {
                            let cur = *page.read();
                            if cur > 0 {
                                page.set(cur - 1);
                            }
                        },
                        "◀ Precedent"
                    }
                    {
                        let display_page = current_page + 1;
                        rsx! {
                            span {
                                style: "font-size: 13px; color: {c.text_secondary};",
                                "Page {display_page} / {total_pages}"
                            }
                        }
                    }
                    button {
                        style: "padding: 6px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 13px;",
                        disabled: current_page + 1 >= total_pages,
                        onclick: move |_| {
                            let cur = *page.read();
                            page.set(cur + 1);
                        },
                        "Suivant ▶"
                    }
                }
            }

            // Totaux
            div {
                style: "display: flex; justify-content: center; gap: 24px; padding: 16px; background: {c.bg_secondary}; border-radius: 8px;",

                div {
                    style: "text-align: center;",
                    p { style: "font-size: 11px; color: {c.text_muted}; text-transform: uppercase;", "Revenus" }
                    p { style: "font-size: 16px; font-weight: 600; color: {c.accent_green};", "+{total_income:.2} EUR" }
                }
                div {
                    style: "width: 1px; background: {c.border};"
                }
                div {
                    style: "text-align: center;",
                    p { style: "font-size: 11px; color: {c.text_muted}; text-transform: uppercase;", "Depenses" }
                    p { style: "font-size: 16px; font-weight: 600; color: {c.accent_red};", "-{total_expense:.2} EUR" }
                }
                div {
                    style: "width: 1px; background: {c.border};"
                }
                {
                    let net_color = if net >= 0.0 { c.accent_green } else { c.accent_red };
                    let net_str = format!("{net:+.2} EUR");
                    rsx! {
                        div {
                            style: "text-align: center;",
                            p { style: "font-size: 11px; color: {c.text_muted}; text-transform: uppercase;", "Solde net" }
                            p {
                                style: "font-size: 16px; font-weight: 600; color: {net_color};",
                                "{net_str}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FilterButton(
    label: &'static str,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_palette();
    let bg = if is_active {
        c.accent_blue
    } else {
        c.bg_secondary
    };
    let color = if is_active { "white" } else { c.text_secondary };

    rsx! {
        button {
            style: "padding: 6px 14px; background: {bg}; border: none; border-radius: 4px; color: {color}; cursor: pointer; font-size: 12px; font-weight: 500;",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}
