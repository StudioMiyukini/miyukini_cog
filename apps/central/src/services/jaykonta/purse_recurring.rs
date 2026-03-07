//! Ecran transactions recurrentes (depenses et revenus).
//!
//! Liste les recurrents actifs, permet d'en ajouter, suspendre, supprimer.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{ActionButton, Badge};
use super::JayKontaState;

#[component]
pub fn PurseRecurring(state: Signal<JayKontaState>) -> Element {
    let p = use_palette();
    let conns = use_service_connections();
    let mut show_form = use_signal(|| false);
    let mut refresh_tick = use_signal(|| 0u32);

    // Charger les recurrents
    let recurring_list = {
        let _tick = refresh_tick();
        let conns_r = conns.read();
        let db = &conns_r.jaykonta;
        let aid = get_account_id(db);
        db.list_recurring(&aid, false).unwrap_or_default()
    };

    let expenses: Vec<jaykonta::domain::purse::RecurringTransaction> = recurring_list
        .iter()
        .filter(|r| r.direction == jaykonta::domain::purse::RecurringDirection::Expense)
        .cloned()
        .collect();
    let incomes: Vec<jaykonta::domain::purse::RecurringTransaction> = recurring_list
        .iter()
        .filter(|r| r.direction == jaykonta::domain::purse::RecurringDirection::Income)
        .cloned()
        .collect();

    // Totaux mensuels
    let monthly_expense: f64 = expenses
        .iter()
        .filter(|r| r.is_active)
        .map(monthly_amount)
        .sum();
    let monthly_income: f64 = incomes
        .iter()
        .filter(|r| r.is_active)
        .map(monthly_amount)
        .sum();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tete
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                div {
                    h1 {
                        style: "font-size: 24px; color: {p.text_high}; margin-bottom: 4px;",
                        "Transactions recurrentes"
                    }
                    p {
                        style: "font-size: 13px; color: {p.text_muted};",
                        "Gerez vos depenses et revenus periodiques"
                    }
                }
                ActionButton {
                    label: "Ajouter un recurrent".to_string(),
                    icon: "+".to_string(),
                    accent: true,
                    onclick: move |_| { show_form.set(!show_form()); },
                }
            }

            // Resume mensuel
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                SummaryCard {
                    label: "Revenus recurrents / mois",
                    amount: monthly_income,
                    positive: true,
                }
                SummaryCard {
                    label: "Depenses recurrentes / mois",
                    amount: monthly_expense,
                    positive: false,
                }
                SummaryCard {
                    label: "Solde net recurrent / mois",
                    amount: monthly_income - monthly_expense,
                    positive: monthly_income >= monthly_expense,
                }
            }

            // Formulaire d'ajout
            if show_form() {
                RecurringForm {
                    on_save: move |_| {
                        show_form.set(false);
                        refresh_tick.set(refresh_tick() + 1);
                    },
                    on_cancel: move |_| { show_form.set(false); },
                }
            }

            // Section revenus
            div {
                style: "display: flex; flex-direction: column; gap: 12px;",

                h2 {
                    style: "font-size: 18px; color: {p.success}; display: flex; align-items: center; gap: 8px;",
                    span { "💵" }
                    span { "Revenus recurrents ({incomes.len()})" }
                }

                if incomes.is_empty() {
                    p {
                        style: "font-size: 13px; color: {p.text_muted}; padding: 16px;",
                        "Aucun revenu recurrent. Ajoutez votre salaire, allocations, etc."
                    }
                }

                for item in incomes.into_iter() {
                    { render_recurring_row(&item, &c, conns, refresh_tick) }
                }
            }

            // Section depenses
            div {
                style: "display: flex; flex-direction: column; gap: 12px;",

                h2 {
                    style: "font-size: 18px; color: {p.error}; display: flex; align-items: center; gap: 8px;",
                    span { "💸" }
                    span { "Depenses recurrentes ({expenses.len()})" }
                }

                if expenses.is_empty() {
                    p {
                        style: "font-size: 13px; color: {p.text_muted}; padding: 16px;",
                        "Aucune depense recurrente. Ajoutez loyer, abonnements, etc."
                    }
                }

                for item in expenses.into_iter() {
                    { render_recurring_row(&item, &c, conns, refresh_tick) }
                }
            }
        }
    }
}

/// Rend une ligne de transaction recurrente (inline, pas un composant Dioxus pour eviter PartialEq).
fn render_recurring_row(
    item: &jaykonta::domain::purse::RecurringTransaction,
    c: &crate::theme::ThemePalette,
    conns: Signal<std::sync::Arc<crate::data::ServiceConnections>>,
    mut refresh_tick: Signal<u32>,
) -> Element {
    let is_income = item.direction == jaykonta::domain::purse::RecurringDirection::Income;
    let amount_color = if is_income { p.success } else { p.error };
    let sign = if is_income { "+" } else { "-" };
    let freq_label = item.frequency.label();
    let status_color = if item.is_active { p.success } else { p.text_muted };
    let status_label = if item.is_active { "Actif" } else { "Suspendu" };
    let opacity = if item.is_active { "1" } else { "0.5" };

    let item_id = item.id.clone();
    let item_id2 = item.id.clone();
    let item_active = item.is_active;
    let cat_name = item.category.as_ref().map(|c| c.name.clone());
    let next_due = item.next_due_date.clone();
    let end_date = item.end_date.clone();
    let desc = item.description.clone();
    let amt = item.amount;
    let currency = item.currency.clone();

    rsx! {
        div {
            style: "display: flex; align-items: center; justify-content: space-between; padding: 14px 16px; background: {p.bg_secondary}; border-radius: 6px; opacity: {opacity};",

            // Info gauche
            div {
                style: "flex: 1; display: flex; flex-direction: column; gap: 4px;",

                div {
                    style: "display: flex; align-items: center; gap: 8px;",

                    p {
                        style: "font-size: 14px; color: {p.text_high}; font-weight: 500;",
                        "{desc}"
                    }
                    Badge {
                        text: freq_label.to_string(),
                        color: p.accent_primary.to_string(),
                    }
                    Badge {
                        text: status_label.to_string(),
                        color: status_color.to_string(),
                    }
                }

                div {
                    style: "display: flex; align-items: center; gap: 12px; font-size: 12px; color: {p.text_muted};",

                    if let Some(ref cn) = cat_name {
                        span { "{cn}" }
                    }
                    span { "Prochaine: {next_due}" }
                    if let Some(ref ed) = end_date {
                        span { "Fin: {ed}" }
                    }
                }
            }

            // Montant + actions
            div {
                style: "display: flex; align-items: center; gap: 16px;",

                span {
                    style: "font-size: 16px; font-weight: 600; color: {amount_color}; white-space: nowrap;",
                    "{sign}{amt:.2} {currency}"
                }

                div {
                    style: "display: flex; gap: 6px;",

                    button {
                        style: "padding: 6px 10px; background: {p.bg_overlay}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_secondary}; cursor: pointer; font-size: 11px;",
                        onclick: move |_| {
                            let conns_r = conns.read();
                            let db = &conns_r.jaykonta;
                            let _ = db.toggle_recurring(&item_id, !item_active);
                            refresh_tick.set(refresh_tick() + 1);
                        },
                        if item_active { "⏸ Suspendre" } else { "▶ Activer" }
                    }
                    button {
                        style: "padding: 6px 10px; background: {p.bg_overlay}; border: 1px solid {p.error}40; border-radius: 4px; color: {p.error}; cursor: pointer; font-size: 11px;",
                        onclick: move |_| {
                            let conns_r = conns.read();
                            let db = &conns_r.jaykonta;
                            let _ = db.delete_recurring(&item_id2);
                            refresh_tick.set(refresh_tick() + 1);
                        },
                        "🗑"
                    }
                }
            }
        }
    }
}

// ── Composants internes ────────────────────────────────────────────

#[component]
fn SummaryCard(label: &'static str, amount: f64, positive: bool) -> Element {
    let p = use_palette();
    let color = if positive { p.success } else { p.error };

    rsx! {
        div {
            style: "background: {p.bg_secondary}; border-radius: 8px; padding: 16px;",

            p {
                style: "font-size: 12px; color: {p.text_muted}; margin-bottom: 8px;",
                "{label}"
            }
            p {
                style: "font-size: 22px; font-weight: 600; color: {color};",
                "{amount:.2} EUR"
            }
        }
    }
}

#[component]
fn RecurringForm(
    on_save: EventHandler<MouseEvent>,
    on_cancel: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();
    let conns = use_service_connections();

    let mut direction = use_signal(|| "expense".to_string());
    let mut amount = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut frequency = use_signal(|| "monthly".to_string());
    let mut day_of_month = use_signal(|| "1".to_string());
    let mut start_date = use_signal(|| chrono::Local::now().format("%Y-%m-%d").to_string());
    let mut end_date = use_signal(String::new);
    let mut notes = use_signal(String::new);
    let mut error_msg = use_signal(String::new);

    // Charger les categories
    let categories = {
        let conns_r = conns.read();
        let db = &conns_r.jaykonta;
        let aid = get_account_id(db);
        db.list_categories(&aid).unwrap_or_default()
    };
    let mut category_id = use_signal(String::new);

    let input_style = format!(
        "padding: 8px 12px; background: {}; border: 1px solid {}; border-radius: 4px; color: {}; font-size: 13px;",
        p.bg_base, p.border_default, p.text_primary
    );
    let select_style = input_style.clone();

    rsx! {
        div {
            style: "background: {p.bg_secondary}; border-radius: 8px; padding: 24px; border: 1px solid {p.accent_primary}40;",

            h3 {
                style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                "Nouvelle transaction recurrente"
            }

            // Direction
            div {
                style: "display: flex; gap: 8px; margin-bottom: 16px;",

                DirectionBtn {
                    label: "Depense",
                    icon: "💸",
                    is_active: direction() == "expense",
                    onclick: move |_| direction.set("expense".to_string()),
                }
                DirectionBtn {
                    label: "Revenu",
                    icon: "💵",
                    is_active: direction() == "income",
                    onclick: move |_| direction.set("income".to_string()),
                }
            }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-bottom: 16px;",

                // Description
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: 12px; color: {p.text_secondary};", "Description" }
                    input {
                        style: "{input_style}",
                        r#type: "text",
                        placeholder: "Ex: Loyer, Salaire, Netflix...",
                        value: "{description}",
                        oninput: move |evt: Event<FormData>| description.set(evt.value().clone()),
                    }
                }

                // Montant
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: 12px; color: {p.text_secondary};", "Montant (EUR)" }
                    input {
                        style: "{input_style}",
                        r#type: "text",
                        placeholder: "0.00",
                        value: "{amount}",
                        oninput: move |evt: Event<FormData>| amount.set(evt.value().clone()),
                    }
                }

                // Frequence
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: 12px; color: {p.text_secondary};", "Frequence" }
                    select {
                        style: "{select_style}",
                        value: "{frequency}",
                        onchange: move |evt: Event<FormData>| frequency.set(evt.value().clone()),
                        option { value: "weekly", "Hebdomadaire" }
                        option { value: "biweekly", "Bi-hebdomadaire" }
                        option { value: "monthly", "Mensuel" }
                        option { value: "quarterly", "Trimestriel" }
                        option { value: "yearly", "Annuel" }
                    }
                }

                // Jour du mois
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: 12px; color: {p.text_secondary};", "Jour du mois (1-31)" }
                    input {
                        style: "{input_style}",
                        r#type: "text",
                        placeholder: "1",
                        value: "{day_of_month}",
                        oninput: move |evt: Event<FormData>| day_of_month.set(evt.value().clone()),
                    }
                }

                // Categorie
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: 12px; color: {p.text_secondary};", "Categorie" }
                    select {
                        style: "{select_style}",
                        value: "{category_id}",
                        onchange: move |evt: Event<FormData>| category_id.set(evt.value().clone()),
                        option { value: "", "— Sans categorie —" }
                        for cat in categories.iter() {
                            option {
                                value: "{cat.id}",
                                "{cat.icon.as_deref().unwrap_or(\"\")} {cat.name}"
                            }
                        }
                    }
                }

                // Date de debut
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: 12px; color: {p.text_secondary};", "Date de debut" }
                    input {
                        style: "{input_style}",
                        r#type: "text",
                        placeholder: "YYYY-MM-DD",
                        value: "{start_date}",
                        oninput: move |evt: Event<FormData>| start_date.set(evt.value().clone()),
                    }
                }

                // Date de fin
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: 12px; color: {p.text_secondary};", "Date de fin (optionnel)" }
                    input {
                        style: "{input_style}",
                        r#type: "text",
                        placeholder: "YYYY-MM-DD ou vide",
                        value: "{end_date}",
                        oninput: move |evt: Event<FormData>| end_date.set(evt.value().clone()),
                    }
                }

                // Notes
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: 12px; color: {p.text_secondary};", "Notes (optionnel)" }
                    input {
                        style: "{input_style}",
                        r#type: "text",
                        placeholder: "Details supplementaires...",
                        value: "{notes}",
                        oninput: move |evt: Event<FormData>| notes.set(evt.value().clone()),
                    }
                }
            }

            if !error_msg().is_empty() {
                p {
                    style: "color: {p.error}; font-size: 13px; margin-bottom: 12px;",
                    "{error_msg}"
                }
            }

            // Boutons
            div {
                style: "display: flex; gap: 12px; justify-content: flex-end;",

                button {
                    style: "padding: 10px 20px; background: {p.bg_overlay}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_secondary}; cursor: pointer; font-size: 13px;",
                    onclick: move |evt| on_cancel.call(evt),
                    "Annuler"
                }
                button {
                    style: "padding: 10px 20px; background: {p.accent_primary}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 13px; font-weight: 500;",
                    onclick: move |evt| {
                        // Validation
                        let amt: f64 = amount().parse().unwrap_or(0.0);
                        if amt <= 0.0 {
                            error_msg.set("Montant invalide".to_string());
                            return;
                        }
                        if description().trim().is_empty() {
                            error_msg.set("Description requise".to_string());
                            return;
                        }

                        let dom: Option<i32> = day_of_month().parse().ok().filter(|d| *d >= 1 && *d <= 31);
                        let end = if end_date().trim().is_empty() { None } else { Some(end_date()) };
                        let cat = if category_id().is_empty() { None } else { Some(category_id()) };
                        let note = if notes().trim().is_empty() { None } else { Some(notes()) };

                        let conns_r = conns.read();
                        let db = &conns_r.jaykonta;
                        let account_id = get_account_id(db);
                        let id = uuid::Uuid::new_v4().to_string();
                        let next_due = start_date();
                        let result = db.create_recurring(
                            &id,
                            &account_id,
                            &direction(),
                            amt,
                            cat.as_deref(),
                            description().trim(),
                            &frequency(),
                            dom,
                            None, // day_of_week
                            &start_date(),
                            end.as_deref(),
                            &next_due,
                            false, // auto_create
                            note.as_deref(),
                        );
                        match result {
                            Ok(()) => {
                                error_msg.set(String::new());
                                on_save.call(evt);
                            }
                            Err(e) => error_msg.set(format!("Erreur: {e}")),
                        }
                    },
                    "Enregistrer"
                }
            }
        }
    }
}

#[component]
fn DirectionBtn(
    label: &'static str,
    icon: &'static str,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();
    let bg = if is_active { p.accent_primary } else { p.bg_overlay };
    let color = if is_active { "white" } else { p.text_secondary };

    rsx! {
        button {
            style: "flex: 1; padding: 10px; background: {bg}; color: {color}; border: none; border-radius: 4px; cursor: pointer; font-size: 13px; font-weight: 500; display: flex; align-items: center; justify-content: center; gap: 8px;",
            onclick: move |evt| onclick.call(evt),
            span { "{icon}" }
            span { "{label}" }
        }
    }
}

/// Helper : obtenir l'account_id purse depuis la DB.
fn get_account_id(db: &jaykonta::data::JayKontaDb) -> String {
    db.ensure_account("purse", "default", "Budget personnel")
        .unwrap_or_default()
}

/// Convertit un montant de recurrent en equivalent mensuel.
fn monthly_amount(r: &jaykonta::domain::purse::RecurringTransaction) -> f64 {
    match r.frequency {
        jaykonta::domain::purse::RecurringFrequency::Weekly => r.amount * 52.0 / 12.0,
        jaykonta::domain::purse::RecurringFrequency::Biweekly => r.amount * 26.0 / 12.0,
        jaykonta::domain::purse::RecurringFrequency::Monthly => r.amount,
        jaykonta::domain::purse::RecurringFrequency::Quarterly => r.amount / 3.0,
        jaykonta::domain::purse::RecurringFrequency::Yearly => r.amount / 12.0,
    }
}
