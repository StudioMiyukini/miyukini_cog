//! Ecran previsionnel budget — projection sur 6 ou 12 mois
//! basee sur les transactions recurrentes et le solde actuel.

use chrono::Datelike;
use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::KpiCard;
use super::JayKontaState;

#[component]
pub fn PurseForecast(state: Signal<JayKontaState>) -> Element {
    let p = use_palette();
    let conns = use_service_connections();
    let mut months_ahead = use_signal(|| 6u32);

    // Charger le previsionnel directement depuis la DB
    let forecast_view = {
        let _m = months_ahead();
        let conns_r = conns.read();
        let db = &conns_r.jaykonta;
        build_forecast_direct(db, months_ahead())
    };

    let Some(fv) = forecast_view else {
        return rsx! {
            div {
                style: "display: flex; align-items: center; justify-content: center; height: 100%; color: {p.text_muted};",
                p { "Impossible de charger le previsionnel. Verifiez la connexion JayKonta." }
            }
        };
    };

    // Trouver le min/max du projected_balance pour la barre visuelle
    let max_balance = fv.forecast.iter().map(|e| e.projected_balance).fold(f64::MIN, f64::max);
    let min_balance = fv.forecast.iter().map(|e| e.projected_balance).fold(f64::MAX, f64::min);
    let balance_range = (max_balance - min_balance).max(1.0);

    // Detecter premier solde negatif
    let first_negative = fv.forecast.iter().find(|e| e.projected_balance < 0.0).cloned();

    // Formatter la detail du net mensuel
    let net_detail = if fv.monthly_net >= 0.0 { "Excedent".to_string() } else { "Deficit".to_string() };
    let net_icon = if fv.monthly_net >= 0.0 { "📈".to_string() } else { "📉".to_string() };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tete
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                div {
                    h1 {
                        style: "font-size: 24px; color: {p.text_high}; margin-bottom: 4px;",
                        "Previsionnel Budget"
                    }
                    p {
                        style: "font-size: 13px; color: {p.text_muted};",
                        "Projection basee sur vos {fv.active_recurring_count} transactions recurrentes actives"
                    }
                }

                // Selecteur horizon
                div {
                    style: "display: flex; gap: 8px;",

                    HorizonBtn {
                        label: "6 mois",
                        is_active: months_ahead() == 6,
                        onclick: move |_| months_ahead.set(6),
                    }
                    HorizonBtn {
                        label: "12 mois",
                        is_active: months_ahead() == 12,
                        onclick: move |_| months_ahead.set(12),
                    }
                }
            }

            // KPIs resume
            div {
                style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px;",

                KpiCard {
                    label: "Solde actuel".to_string(),
                    value: format!("{:.2} EUR", fv.current_balance),
                    detail: "Base de la projection".to_string(),
                    icon: "💰".to_string(),
                    positive: fv.current_balance >= 0.0,
                }
                KpiCard {
                    label: "Revenus recurrents / mois".to_string(),
                    value: format!("+{:.2} EUR", fv.monthly_recurring_income),
                    detail: "Estimation mensuelle".to_string(),
                    icon: "💵".to_string(),
                    positive: true,
                }
                KpiCard {
                    label: "Depenses recurrentes / mois".to_string(),
                    value: format!("-{:.2} EUR", fv.monthly_recurring_expense),
                    detail: "Estimation mensuelle".to_string(),
                    icon: "💸".to_string(),
                    positive: false,
                }
                KpiCard {
                    label: "Solde net / mois".to_string(),
                    value: format!("{:+.2} EUR", fv.monthly_net),
                    detail: net_detail,
                    icon: net_icon,
                    positive: fv.monthly_net >= 0.0,
                }
            }

            // Tableau previsionnel
            div {
                style: "background: {p.bg_secondary}; border-radius: 8px; padding: 24px;",

                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                    "Projection mois par mois"
                }

                // En-tete tableau
                div {
                    style: "display: grid; grid-template-columns: 100px 1fr 100px 100px 100px 120px; gap: 8px; padding: 8px 0; border-bottom: 1px solid {p.border_default}; font-size: 11px; color: {p.text_muted}; font-weight: 600; text-transform: uppercase;",

                    span { "Mois" }
                    span { "Evolution" }
                    span { style: "text-align: right;", "Revenus" }
                    span { style: "text-align: right;", "Depenses" }
                    span { style: "text-align: right;", "Net" }
                    span { style: "text-align: right;", "Solde projete" }
                }

                // Lignes
                for entry in fv.forecast.iter() {
                    { render_forecast_row(entry, min_balance, balance_range, &c) }
                }
            }

            // Alerte si solde negatif prevu
            if let Some(ref neg) = first_negative {
                div {
                    style: "background: {p.error}15; border: 1px solid {p.error}40; border-radius: 8px; padding: 16px; display: flex; align-items: center; gap: 12px;",

                    span { style: "font-size: 24px;", "⚠️" }
                    div {
                        p {
                            style: "font-size: 14px; color: {p.error}; font-weight: 500;",
                            "Attention : solde negatif prevu en {neg.month_label}"
                        }
                        p {
                            style: "font-size: 12px; color: {p.text_muted}; margin-top: 4px;",
                            "Solde projete : {neg.projected_balance:.2} EUR. Pensez a ajuster vos depenses recurrentes ou a augmenter vos revenus."
                        }
                    }
                }
            }
            if first_negative.is_none() {
                div {
                    style: "background: {p.success}15; border: 1px solid {p.success}40; border-radius: 8px; padding: 16px; display: flex; align-items: center; gap: 12px;",

                    span { style: "font-size: 24px;", "✅" }
                    div {
                        p {
                            style: "font-size: 14px; color: {p.success}; font-weight: 500;",
                            "Previsions positives"
                        }
                        p {
                            style: "font-size: 12px; color: {p.text_muted}; margin-top: 4px;",
                            "Votre solde reste positif sur toute la periode projetee."
                        }
                    }
                }
            }
        }
    }
}

/// Rend une ligne du tableau previsionnel.
fn render_forecast_row(
    entry: &jaykonta::domain::purse::ForecastEntry,
    min_balance: f64,
    balance_range: f64,
    c: &crate::theme::ThemePalette,
) -> Element {
    let net_color = if entry.recurring_net >= 0.0 { p.success } else { p.error };
    let balance_color = if entry.projected_balance >= 0.0 { p.success } else { p.error };
    let bar_pct = if balance_range > 0.0 {
        ((entry.projected_balance - min_balance) / balance_range * 100.0).clamp(2.0, 100.0)
    } else {
        50.0
    };
    let bar_color = if entry.projected_balance >= 0.0 { p.success } else { p.error };

    rsx! {
        div {
            style: "display: grid; grid-template-columns: 100px 1fr 100px 100px 100px 120px; gap: 8px; padding: 10px 0; border-bottom: 1px solid {p.border_default}20; align-items: center; font-size: 13px;",

            span {
                style: "color: {p.text_primary}; font-weight: 500;",
                "{entry.month_label}"
            }

            div {
                style: "width: 100%; height: 6px; background: {p.bg_overlay}; border-radius: 3px; overflow: hidden;",
                div {
                    style: "width: {bar_pct}%; height: 100%; background: {bar_color}; border-radius: 3px; transition: width 0.3s;",
                }
            }

            span {
                style: "text-align: right; color: {p.success};",
                "+{entry.recurring_income:.2}"
            }

            span {
                style: "text-align: right; color: {p.error};",
                "-{entry.recurring_expense:.2}"
            }

            span {
                style: "text-align: right; color: {net_color}; font-weight: 500;",
                "{entry.recurring_net:+.2}"
            }

            span {
                style: "text-align: right; color: {balance_color}; font-weight: 600;",
                "{entry.projected_balance:.2}"
            }
        }
    }
}

// ── Composants internes ────────────────────────────────────────────

#[component]
fn HorizonBtn(
    label: &'static str,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();
    let bg = if is_active { p.accent_primary } else { p.bg_overlay };
    let color = if is_active { "white" } else { p.text_secondary };

    rsx! {
        button {
            style: "padding: 8px 16px; background: {bg}; color: {color}; border: none; border-radius: 4px; cursor: pointer; font-size: 13px; font-weight: 500;",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}

/// Construire le previsionnel directement depuis la DB.
fn build_forecast_direct(
    db: &jaykonta::data::JayKontaDb,
    months_ahead: u32,
) -> Option<jaykonta::domain::purse::BudgetForecastView> {
    let account_id = db.ensure_account("purse", "default", "Budget personnel").ok()?;
    let balance = db.solde_purse().ok()?;
    let recurring = db.list_recurring(&account_id, true).ok()?;
    let (monthly_recurring_income, monthly_recurring_expense) =
        db.recurring_monthly_totals(&account_id).ok()?;
    let monthly_net = monthly_recurring_income - monthly_recurring_expense;

    let today = chrono::Local::now().date_naive();
    let mut forecast = Vec::with_capacity(months_ahead as usize);
    let mut cumulative = balance;

    for i in 0..months_ahead {
        let target = add_months(today, i + 1);
        let month_str = target.format("%Y-%m").to_string();
        let month_label = french_month_label(&month_str);

        let (inc, exp) = compute_month_for_recurring(&recurring, &month_str);
        let net = inc - exp;
        cumulative += net;

        forecast.push(jaykonta::domain::purse::ForecastEntry {
            month: month_str,
            month_label,
            recurring_income: inc,
            recurring_expense: exp,
            recurring_net: net,
            projected_balance: cumulative,
        });
    }

    Some(jaykonta::domain::purse::BudgetForecastView {
        current_balance: balance,
        currency: "EUR".to_string(),
        monthly_recurring_income,
        monthly_recurring_expense,
        monthly_net,
        forecast,
        active_recurring_count: recurring.len() as u32,
    })
}

fn add_months(date: chrono::NaiveDate, months: u32) -> chrono::NaiveDate {
    use chrono::NaiveDate;
    let total = date.year() as u32 * 12 + date.month() - 1 + months;
    let y = (total / 12) as i32; // intentionnel : calcul mois/année
    let m = total % 12 + 1;
    let max_day = days_in_month(y, m);
    let d = date.day().min(max_day);
    NaiveDate::from_ymd_opt(y, m, d).unwrap_or(date)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    use chrono::NaiveDate;
    if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .map_or(28, |d| d.pred_opt().map_or(28, |p| p.day()))
}

fn french_month_label(month: &str) -> String {
    let mon: u32 = month[5..7].parse().unwrap_or(1);
    let year = &month[..4];
    let name = match mon {
        1 => "Jan", 2 => "Fev", 3 => "Mar", 4 => "Avr",
        5 => "Mai", 6 => "Juin", 7 => "Juil", 8 => "Aout",
        9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
        _ => "?",
    };
    format!("{name} {year}")
}

fn compute_month_for_recurring(
    recurring: &[jaykonta::domain::purse::RecurringTransaction],
    month: &str,
) -> (f64, f64) {
    use chrono::NaiveDate;
    let year: i32 = month[..4].parse().unwrap_or(2026);
    let mon: u32 = month[5..7].parse().unwrap_or(1);
    let m_start = NaiveDate::from_ymd_opt(year, mon, 1).unwrap_or_default();
    let m_end = if mon == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, mon + 1, 1)
    }
    .unwrap_or_default();

    let mut income = 0.0;
    let mut expense = 0.0;

    for r in recurring {
        if !r.is_active { continue; }
        let start = NaiveDate::parse_from_str(&r.start_date, "%Y-%m-%d").unwrap_or(m_start);
        if start >= m_end { continue; }
        if let Some(ref end) = r.end_date {
            let end_d = NaiveDate::parse_from_str(end, "%Y-%m-%d").unwrap_or(m_end);
            if end_d < m_start { continue; }
        }

        let occ = match r.frequency {
            jaykonta::domain::purse::RecurringFrequency::Weekly => 4,
            jaykonta::domain::purse::RecurringFrequency::Biweekly => 2,
            jaykonta::domain::purse::RecurringFrequency::Monthly => 1,
            jaykonta::domain::purse::RecurringFrequency::Quarterly => {
                let start_m = start.month();
                i32::from((m_start.month() + 12 - start_m).is_multiple_of(3))
            }
            jaykonta::domain::purse::RecurringFrequency::Yearly => {
                i32::from(m_start.month() == start.month())
            }
        };

        let total = r.amount * f64::from(occ);
        match r.direction {
            jaykonta::domain::purse::RecurringDirection::Income => income += total,
            jaykonta::domain::purse::RecurringDirection::Expense => expense += total,
        }
    }

    (income, expense)
}
