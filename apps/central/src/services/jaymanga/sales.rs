//! Ventes JayManga — tableau de bord ventes complet : résumé revenus,
//! graphique 30 jours, transactions filtrables, licences, export.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use jaymanga::export::csv::{CsvTransactionRow, csv_generate};
use jaymanga::export::pdf::{SalesReportData, SalesReportSummary};
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{PageHeader, Badge, StatCard, EmptyState, ActionButton};
use super::JayMangaState;

/// Onglet actif dans les ventes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum SalesTab {
    #[default]
    Overview,
    Transactions,
    Licenses,
}

#[component]
pub fn Sales(state: Signal<JayMangaState>) -> Element {
    let p = use_palette();
    let conns = use_service_connections();

    let db = &conns.read().jaymanga;
    let licenses = db.license_list_all().unwrap_or_default();
    let transactions = db.transaction_list(&jaymanga::data::TransactionFilters::default()).unwrap_or_default();

    let mut active_tab = use_signal(|| SalesTab::Overview);

    // ── Statistiques globales ───────────────────────────────────────
    let license_count = licenses.len();
    let active_count = licenses.iter().filter(|l| l.status.as_deref() == Some("active")).count();
    let _refunded_count = licenses.iter().filter(|l| l.status.as_deref() == Some("refunded")).count();

    let total_revenue_cents: i64 = licenses.iter()
        .filter(|l| l.status.as_deref() == Some("active"))
        .filter_map(|l| l.amount_paid)
        .sum();
    let total_revenue = format_amount(total_revenue_cents);

    // Revenus du mois (approximation par licences récentes)
    let now_prefix = chrono::Utc::now().format("%Y-%m").to_string();
    let month_revenue_cents: i64 = licenses.iter()
        .filter(|l| l.status.as_deref() == Some("active"))
        .filter(|l| l.purchased_at.as_deref().unwrap_or("").starts_with(&now_prefix))
        .filter_map(|l| l.amount_paid)
        .sum();
    let month_revenue = format_amount(month_revenue_cents);

    let month_sales = licenses.iter()
        .filter(|l| l.purchased_at.as_deref().unwrap_or("").starts_with(&now_prefix))
        .count();

    let avg_amount = if active_count > 0 {
        format_amount(total_revenue_cents / active_count as i64)
    } else {
        "0,00 €".to_string()
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            PageHeader {
                title: "💰 Ventes".to_string(),
                subtitle: "Tableau de bord des ventes et revenus".to_string(),
                count: Some(license_count),
            }

            // Stats principales
            div {
                style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 16px;",

                StatCard { label: "Revenus totaux".to_string(), value: total_revenue, icon: "💰".to_string(), color: "#10b981".to_string() }
                StatCard { label: "Ce mois".to_string(), value: month_revenue, icon: "📅".to_string(), color: p.accent_primary.to_string() }
                StatCard { label: "Ventes ce mois".to_string(), value: month_sales.to_string(), icon: "🛒".to_string(), color: p.warning.to_string() }
                StatCard { label: "Licences actives".to_string(), value: active_count.to_string(), icon: "✅".to_string(), color: p.success.to_string() }
                StatCard { label: "Panier moyen".to_string(), value: avg_amount, icon: "📊".to_string(), color: "#8b5cf6".to_string() }
            }

            // Onglets
            div {
                style: "display: flex; gap: 4px; border-bottom: 1px solid {p.border_default};",

                SalesTabButton { label: "Vue d'ensemble", is_active: *active_tab.read() == SalesTab::Overview, onclick: move |_| { active_tab.set(SalesTab::Overview); } }
                SalesTabButton { label: "Transactions", is_active: *active_tab.read() == SalesTab::Transactions, onclick: move |_| { active_tab.set(SalesTab::Transactions); } }
                SalesTabButton { label: "Licences", is_active: *active_tab.read() == SalesTab::Licenses, onclick: move |_| { active_tab.set(SalesTab::Licenses); } }
            }

            // Contenu onglet
            match *active_tab.read() {
                SalesTab::Overview => rsx! {
                    SalesOverview { licenses: licenses.clone() }
                },
                SalesTab::Transactions => rsx! {
                    TransactionsView { transactions: transactions.clone() }
                },
                SalesTab::Licenses => rsx! {
                    LicensesView { licenses: licenses.clone() }
                },
            }
        }
    }
}

fn format_amount(cents: i64) -> String {
    if cents == 0 {
        "0,00 €".to_string()
    } else {
        format!("{},{:02} €", cents / 100, (cents % 100).abs())
    }
}

// ── Bouton onglet ventes ────────────────────────────────────────────────

#[component]
fn SalesTabButton(
    label: &'static str,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();
    let border_color = if is_active { p.accent_primary } else { "transparent" };
    let text_color = if is_active { p.text_high } else { p.text_secondary };

    rsx! {
        button {
            style: "padding: 10px 16px; background: transparent; border: none; border-bottom: 2px solid {border_color}; color: {text_color}; cursor: pointer; font-size: 13px; transition: all 0.2s;",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}

// ── Vue d'ensemble ──────────────────────────────────────────────────────

#[component]
fn SalesOverview(
    licenses: Vec<jaymanga::data::PurchaseLicense>,
) -> Element {
    let p = use_palette();

    // Graphique 30 jours simplifié (barres textuelles)
    let now = chrono::Utc::now();
    let mut daily_revenue: Vec<(String, i64)> = Vec::new();
    for i in (0..30).rev() {
        let day = now - chrono::Duration::days(i);
        let day_str = day.format("%m-%d").to_string();
        let day_prefix = day.format("%Y-%m-%d").to_string();
        let rev: i64 = licenses.iter()
            .filter(|l| l.status.as_deref() == Some("active"))
            .filter(|l| l.purchased_at.as_deref().unwrap_or("").starts_with(&day_prefix))
            .filter_map(|l| l.amount_paid)
            .sum();
        daily_revenue.push((day_str, rev));
    }

    let max_rev = daily_revenue.iter().map(|(_, r)| *r).max().unwrap_or(1).max(1);

    // Ventes récentes (dernières 5)
    let mut recent_licenses: Vec<_> = licenses.iter()
        .filter(|l| l.status.as_deref() == Some("active"))
        .collect();
    recent_licenses.sort_by(|a, b| {
        let da = a.purchased_at.as_deref().unwrap_or("");
        let db_val = b.purchased_at.as_deref().unwrap_or("");
        db_val.cmp(da)
    });
    let recent_licenses: Vec<_> = recent_licenses.into_iter().take(5).collect();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            // Graphique 30 jours
            div {
                style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px; border: 1px solid {p.border_default};",

                h3 {
                    style: "font-size: 14px; color: {p.text_high}; margin-bottom: 16px;",
                    "📈 Revenus — 30 derniers jours"
                }

                div {
                    style: "display: flex; align-items: flex-end; gap: 2px; height: 120px;",

                    for (day, rev) in daily_revenue.iter() {
                        {
                            let height_pct = if *rev > 0 { (*rev as f64 / max_rev as f64 * 100.0).max(4.0) } else { 2.0 };
                            let bg = if *rev > 0 { p.success } else { p.bg_overlay };
                            let amount = format_amount(*rev);

                            rsx! {
                                div {
                                    style: "flex: 1; display: flex; flex-direction: column; align-items: center; gap: 2px;",
                                    title: "{day}: {amount}",

                                    div {
                                        style: "width: 100%; height: {height_pct}%; min-height: 2px; background: {bg}; border-radius: 2px 2px 0 0; transition: height 0.3s;",
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    style: "display: flex; justify-content: space-between; margin-top: 4px;",
                    span { style: "font-size: 10px; color: {p.text_muted};", "il y a 30j" }
                    span { style: "font-size: 10px; color: {p.text_muted};", "aujourd'hui" }
                }
            }

            // Ventes récentes
            div {
                style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px; border: 1px solid {p.border_default};",

                h3 {
                    style: "font-size: 14px; color: {p.text_high}; margin-bottom: 12px;",
                    "🛒 Ventes récentes"
                }

                if recent_licenses.is_empty() {
                    p {
                        style: "font-size: 13px; color: {p.text_muted}; text-align: center; padding: 20px;",
                        "Aucune vente récente"
                    }
                } else {
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        for license in recent_licenses.iter() {
                            {
                                let buyer = license.buyer_cog_id.clone().unwrap_or_else(|| "Inconnu".to_string());
                                let buyer_short = if buyer.len() > 12 { format!("{}...", &buyer[..12]) } else { buyer };
                                let amount = license.amount_paid.unwrap_or(0);
                                let amount_str = format_amount(amount);
                                let date = license.purchased_at.clone()
                                    .map(|d| if d.len() >= 16 { d[..16].to_string() } else { d })
                                    .unwrap_or_default();

                                rsx! {
                                    div {
                                        style: "display: flex; align-items: center; gap: 12px; padding: 10px;",

                                        span { style: "font-size: 16px;", "📖" }
                                        div {
                                            style: "flex: 1;",
                                            span { style: "font-size: 13px; color: {p.text_high};", "par {buyer_short}" }
                                        }
                                        span {
                                            style: "font-size: 14px; color: {p.success}; font-weight: 600;",
                                            "{amount_str}"
                                        }
                                        span {
                                            style: "font-size: 11px; color: {p.text_muted};",
                                            "{date}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Boutons export
            {
                let licenses_csv = licenses.clone();
                let licenses_pdf = licenses.clone();

                rsx! {
                    div {
                        style: "display: flex; gap: 12px;",

                        ActionButton {
                            label: "Export CSV".to_string(),
                            icon: "📄".to_string(),
                            onclick: move |_| {
                                // Construire les CsvTransactionRow depuis les licences
                                let rows: Vec<CsvTransactionRow> = licenses_csv.iter()
                                    .filter(|l| l.status.as_deref() == Some("active"))
                                    .map(|l| CsvTransactionRow {
                                        transaction_id: l.id.clone().unwrap_or_default(),
                                        date: l.purchased_at.clone().unwrap_or_default(),
                                        buyer_cog_id: l.buyer_cog_id.clone().unwrap_or_default(),
                                        work_title: l.work_id.clone().unwrap_or_default(),
                                        amount_cents: l.amount_paid.unwrap_or(0),
                                        currency: l.currency.clone().unwrap_or_else(|| "EUR".to_string()),
                                        method: l.payment_method.clone().unwrap_or_else(|| "—".to_string()),
                                        status: l.status.clone().unwrap_or_default(),
                                    })
                                    .collect();
                                if !rows.is_empty() {
                                    let csv_content = csv_generate(&rows);
                                    let filename = format!("jaymanga_export_{}.csv", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
                                    // Écrire le fichier via std::fs (desktop)
                                    if let Some(dir) = Some(std::env::temp_dir()) {
                                        let path = dir.join(&filename);
                                        let _ = std::fs::write(&path, &csv_content);
                                    }
                                }
                            },
                        }
                        ActionButton {
                            label: "Export PDF (JSON)".to_string(),
                            icon: "📑".to_string(),
                            onclick: move |_| {
                                // Générer le rapport structuré (JSON) — le rendu PDF est délégué
                                let active_licenses: Vec<_> = licenses_pdf.iter()
                                    .filter(|l| l.status.as_deref() == Some("active"))
                                    .collect();
                                let total_rev: i64 = active_licenses.iter().filter_map(|l| l.amount_paid).sum();
                                let refund_count = licenses_pdf.iter().filter(|l| l.status.as_deref() == Some("refunded")).count();
                                let unique_buyers: std::collections::HashSet<_> = active_licenses.iter()
                                    .filter_map(|l| l.buyer_cog_id.as_ref())
                                    .collect();

                                let report = SalesReportData {
                                    shop_name: "Ma Boutique".to_string(),
                                    period: chrono::Utc::now().format("%Y-%m").to_string(),
                                    generated_at: chrono::Utc::now().to_rfc3339(),
                                    summary: SalesReportSummary {
                                        total_revenue: total_rev,
                                        total_transactions: active_licenses.len() as i32,
                                        total_refunds: refund_count as i32,
                                        net_revenue: total_rev,
                                        currency: "EUR".to_string(),
                                        unique_buyers: unique_buyers.len() as i32,
                                    },
                                    top_works: Vec::new(),
                                    revenue_by_method: Vec::new(),
                                };
                                if let Ok(json) = serde_json::to_string_pretty(&report) {
                                    let filename = format!("jaymanga_report_{}.json", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
                                    if let Some(dir) = Some(std::env::temp_dir()) {
                                        let path = dir.join(&filename);
                                        let _ = std::fs::write(&path, &json);
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

// ── Transactions ────────────────────────────────────────────────────────

#[component]
fn TransactionsView(transactions: Vec<jaymanga::data::PaymentTransaction>) -> Element {
    let p = use_palette();

    if transactions.is_empty() {
        return rsx! {
            EmptyState {
                title: "Aucune transaction".to_string(),
                message: "Les transactions de paiement apparaîtront ici.".to_string(),
                icon: "💳".to_string(),
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 6px;",

            // En-tête
            div {
                style: "display: grid; grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr; gap: 12px; padding: 8px 16px; font-size: 11px; color: {p.text_muted}; text-transform: uppercase; letter-spacing: 0.5px;",
                span { "ID" }
                span { "Type" }
                span { "Montant" }
                span { "Statut" }
                span { "Méthode" }
                span { "Date" }
            }

            for tx in transactions.iter() {
                {
                    let tx_id = tx.id.clone().unwrap_or_default();
                    let tx_short = if tx_id.len() > 8 { format!("{}...", &tx_id[..8]) } else { tx_id };
                    let tx_type = "purchase".to_string();
                    let amount = tx.amount.unwrap_or(0);
                    let amount_str = format_amount(amount);
                    let status = tx.status.clone().unwrap_or_else(|| "pending".to_string());
                    let method = tx.method.clone().unwrap_or_else(|| "—".to_string());
                    let date = tx.created_at.clone()
                        .map(|d| if d.len() >= 10 { d[..10].to_string() } else { d })
                        .unwrap_or_default();

                    let (status_color, status_label) = match status.as_str() {
                        "completed" => (p.success, "Complété"),
                        "pending" => (p.warning, "En attente"),
                        "failed" => (p.error, "Échoué"),
                        "refunded" => ("#8b5cf6", "Remboursé"),
                        _ => (p.text_muted, "—"),
                    };

                    rsx! {
                        div {
                            style: "display: grid; grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr; gap: 12px; padding: 12px 16px; background: {p.bg_secondary}; border-radius: 4px; align-items: center;",

                            span {
                                style: "font-size: 12px; color: {p.text_secondary}; font-family: monospace;",
                                "{tx_short}"
                            }
                            Badge { text: tx_type, color: p.accent_primary.to_string() }
                            span {
                                style: "font-size: 13px; color: {p.success}; font-weight: 500;",
                                "{amount_str}"
                            }
                            Badge { text: status_label.to_string(), color: status_color.to_string() }
                            span {
                                style: "font-size: 12px; color: {p.text_secondary};",
                                "{method}"
                            }
                            span {
                                style: "font-size: 12px; color: {p.text_muted};",
                                "{date}"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Licences ────────────────────────────────────────────────────────────

#[component]
fn LicensesView(licenses: Vec<jaymanga::data::PurchaseLicense>) -> Element {
    let p = use_palette();
    let conns = use_service_connections();

    if licenses.is_empty() {
        return rsx! {
            EmptyState {
                title: "Aucune licence".to_string(),
                message: "Les licences d'achat apparaîtront ici.".to_string(),
                icon: "🎫".to_string(),
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 6px;",

            // Résumé rapide
            {
                let active = licenses.iter().filter(|l| l.status.as_deref() == Some("active")).count();
                let refunded = licenses.iter().filter(|l| l.status.as_deref() == Some("refunded")).count();
                let expired = licenses.iter().filter(|l| l.status.as_deref() == Some("expired")).count();

                rsx! {
                    div {
                        style: "display: flex; gap: 12px; margin-bottom: 8px;",
                        Badge { text: format!("{active} actives"), color: p.success.to_string() }
                        Badge { text: format!("{refunded} remboursées"), color: p.warning.to_string() }
                        Badge { text: format!("{expired} expirées"), color: p.text_muted.to_string() }
                    }
                }
            }

            // En-tête
            div {
                style: "display: grid; grid-template-columns: 2fr 1fr 1fr 1fr 1fr 80px; gap: 12px; padding: 8px 16px; font-size: 11px; color: {p.text_muted}; text-transform: uppercase; letter-spacing: 0.5px;",
                span { "Acheteur" }
                span { "Type" }
                span { "Montant" }
                span { "Statut" }
                span { "Date" }
                span { "Action" }
            }

            for license in licenses.iter() {
                {
                    let lid = license.id.clone().unwrap_or_default();
                    let buyer = license.buyer_cog_id.clone().unwrap_or_else(|| "Inconnu".to_string());
                    let buyer_short = if buyer.len() > 14 { format!("{}...", &buyer[..14]) } else { buyer };
                    let purchase_type = license.purchase_type.clone().unwrap_or_else(|| "work".to_string());
                    let amount = license.amount_paid.unwrap_or(0);
                    let amount_display = format_amount(amount);
                    let status = license.status.clone().unwrap_or_else(|| "active".to_string());
                    let date = license.purchased_at.clone()
                        .map(|d| if d.len() >= 10 { d[..10].to_string() } else { d })
                        .unwrap_or_else(|| "—".to_string());

                    let (status_color, status_label) = match status.as_str() {
                        "active" => (p.success, "Active"),
                        "refunded" => (p.warning, "Remboursée"),
                        "expired" => (p.text_muted, "Expirée"),
                        "revoked" => (p.error, "Révoquée"),
                        _ => (p.text_muted, "—"),
                    };

                    let can_revoke = status == "active";
                    let lid_revoke = lid.clone();

                    rsx! {
                        div {
                            style: "display: grid; grid-template-columns: 2fr 1fr 1fr 1fr 1fr 80px; gap: 12px; padding: 12px 16px; background: {p.bg_secondary}; border-radius: 4px; align-items: center;",

                            span {
                                style: "font-size: 13px; color: {p.text_primary};",
                                "{buyer_short}"
                            }
                            span {
                                style: "font-size: 12px; color: {p.text_secondary};",
                                "{purchase_type}"
                            }
                            span {
                                style: "font-size: 13px; color: {p.success}; font-weight: 500;",
                                "{amount_display}"
                            }
                            Badge { text: status_label.to_string(), color: status_color.to_string() }
                            span {
                                style: "font-size: 12px; color: {p.text_muted};",
                                "{date}"
                            }
                            if can_revoke {
                                button {
                                    style: "padding: 4px 8px; background: transparent; border: 1px solid {p.error}; border-radius: 4px; color: {p.error}; cursor: pointer; font-size: 11px;",
                                    onclick: move |_| {
                                        let db = &conns.read().jaymanga;
                                        let _ = db.license_update_status(&lid_revoke, "revoked");
                                    },
                                    "Révoquer"
                                }
                            } else {
                                span { "—" }
                            }
                        }
                    }
                }
            }
        }
    }
}
