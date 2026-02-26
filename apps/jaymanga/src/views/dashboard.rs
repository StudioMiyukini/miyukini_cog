//! Dashboard JayManga — hub principal vendeur + lecteur.

use dioxus::prelude::*;
use jaymanga::domain::gamification::{gamification_level_for_xp, gamification_level_progress};
use miyukini_service_ui::use_palette;
use crate::use_db;
use super::components::{StatCard, QuickAccessCard, ActionButton, Badge};
use super::{JayMangaSection, JayMangaState};

#[component]
pub fn Dashboard(state: Signal<JayMangaState>) -> Element {
    let c = use_palette();
    let db = use_db();

    // Charger les donnees reelles
    let works = db.work_list(&jaymanga::data::WorkFilters::default()).unwrap_or_default();
    let work_count = works.len();
    let series_count = db.series_list().map(|v| v.len()).unwrap_or(0);
    let licenses = db.license_list_all().unwrap_or_default();
    let license_count = licenses.len();
    let favorites = db.favorite_list().unwrap_or_default();
    let progression = db.progression_get().ok().flatten();

    let chapter_count: usize = works.iter()
        .filter_map(|w| w.id.as_ref())
        .map(|wid| db.chapter_list_by_work(wid).map(|v| v.len()).unwrap_or(0))
        .sum();

    let seller_config = db.seller_config_get().ok().flatten();
    let shop_name = seller_config.as_ref()
        .and_then(|c| c.shop_name.clone())
        .unwrap_or_else(|| "Ma Boutique".to_string());

    // Revenus totaux
    let total_revenue_cents: i64 = licenses.iter()
        .filter(|l| l.status.as_deref() == Some("active"))
        .filter_map(|l| l.amount_paid)
        .sum();
    let total_revenue = format!("{},{:02} \u{20AC}", total_revenue_cents / 100, total_revenue_cents % 100);

    // Revenus ce mois
    let now_prefix = chrono::Utc::now().format("%Y-%m").to_string();
    let month_sales = licenses.iter()
        .filter(|l| l.purchased_at.as_deref().unwrap_or("").starts_with(&now_prefix))
        .count();

    // Publiees / brouillons
    let published_count = works.iter().filter(|w| w.status.as_deref() == Some("published")).count();
    let draft_count = work_count.saturating_sub(published_count);

    // Progression lecteur
    let total_xp = progression.as_ref().and_then(|p| p.total_xp).unwrap_or(0);
    let current_streak = progression.as_ref().and_then(|p| p.current_streak).unwrap_or(0);
    let (level, level_name) = gamification_level_for_xp(total_xp);
    let level_progress = gamification_level_progress(total_xp);
    let progress_pct = (level_progress * 100.0) as i32;

    // Ventes recentes (5 dernieres)
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
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tete
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",

                div {
                    h2 {
                        style: "font-size: 24px; color: {c.text_white};",
                        "\u{1F4DA} {shop_name}"
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 8px; margin-top: 4px;",
                        span {
                            style: "padding: 4px 10px; background: {c.accent_green}20; color: {c.accent_green}; border-radius: 4px; font-size: 11px; font-weight: 500;",
                            "{published_count} publiee(s)"
                        }
                        if draft_count > 0 {
                            span {
                                style: "padding: 4px 10px; background: {c.text_muted}20; color: {c.text_muted}; border-radius: 4px; font-size: 11px; font-weight: 500;",
                                "{draft_count} brouillon(s)"
                            }
                        }
                    }
                }

                // Mini progression lecteur
                div {
                    style: "display: flex; align-items: center; gap: 12px;",

                    if current_streak > 0 {
                        div {
                            style: "display: flex; align-items: center; gap: 4px; padding: 4px 10px; background: #FF450020; border-radius: 6px;",
                            span { style: "font-size: 14px;", "\u{1F525}" }
                            span { style: "font-size: 12px; color: #FF4500; font-weight: 600;", "{current_streak}j" }
                        }
                    }

                    div {
                        style: "text-align: right; cursor: pointer;",
                        onclick: move |_| { state.write().section = JayMangaSection::Profile; },

                        p {
                            style: "font-size: 12px; color: {c.text_secondary};",
                            "Nv.{level} {level_name}"
                        }
                        div {
                            style: "width: 80px; height: 6px; background: {c.bg_hover}; border-radius: 3px; overflow: hidden; margin-top: 2px;",
                            div {
                                style: "width: {progress_pct}%; height: 100%; background: #FFD700; border-radius: 3px;",
                            }
                        }
                    }
                }
            }

            // Statistiques vendeur
            div {
                style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px;",

                StatCard { label: "Oeuvres".to_string(), value: work_count.to_string(), icon: "\u{1F4D6}".to_string(), color: c.accent_blue.to_string() }
                StatCard { label: "Chapitres".to_string(), value: chapter_count.to_string(), icon: "\u{1F4D1}".to_string(), color: c.accent_orange.to_string() }
                StatCard { label: "Revenus".to_string(), value: total_revenue, icon: "\u{1F4B0}".to_string(), color: c.accent_green.to_string() }
                StatCard { label: "Ventes ce mois".to_string(), value: month_sales.to_string(), icon: "\u{1F4C5}".to_string(), color: "#8b5cf6".to_string() }
            }

            // Acces rapides (3 cartes principales)
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                QuickAccessCard {
                    icon: "\u{1F4D6}".to_string(),
                    title: "Mon Catalogue".to_string(),
                    count: format!("{work_count} oeuvres \u{2014} {draft_count} brouillons"),
                    onclick: move |_| { state.write().section = JayMangaSection::Catalogue; },
                }
                QuickAccessCard {
                    icon: "\u{1F4B0}".to_string(),
                    title: "Mes Ventes".to_string(),
                    count: format!("{license_count} ventes \u{2014} ce mois: {month_sales}"),
                    onclick: move |_| { state.write().section = JayMangaSection::Sales; },
                }
                QuickAccessCard {
                    icon: "\u{1F4DA}".to_string(),
                    title: "Ma Bibliotheque".to_string(),
                    count: format!("{} favoris \u{2014} \u{1F525} {} jours", favorites.len(), current_streak),
                    onclick: move |_| { state.write().section = JayMangaSection::Library; },
                }
            }

            // Ventes recentes
            if !recent_licenses.is_empty() {
                div {
                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; border: 1px solid {c.border};",

                    div {
                        style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;",
                        h3 {
                            style: "font-size: 14px; color: {c.text_white};",
                            "\u{1F6D2} Ventes recentes"
                        }
                        button {
                            style: "font-size: 12px; color: {c.accent_blue}; background: transparent; border: none; cursor: pointer;",
                            onclick: move |_| { state.write().section = JayMangaSection::Sales; },
                            "Voir tout \u{2192}"
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        for license in recent_licenses.iter() {
                            {
                                let buyer = license.buyer_cog_id.clone().unwrap_or_else(|| "Inconnu".to_string());
                                let buyer_short = if buyer.len() > 12 { format!("{}...", &buyer[..12]) } else { buyer };
                                let amount = license.amount_paid.unwrap_or(0);
                                let amount_str = if amount == 0 { "Gratuit".to_string() } else { format!("{},{:02} \u{20AC}", amount / 100, amount % 100) };
                                let date = license.purchased_at.clone()
                                    .map(|d| if d.len() >= 16 { d[11..16].to_string() } else { d })
                                    .unwrap_or_default();
                                let ptype = license.purchase_type.clone().unwrap_or_else(|| "work".to_string());

                                rsx! {
                                    div {
                                        style: "display: flex; align-items: center; gap: 12px; padding: 8px 0; border-bottom: 1px solid {c.border};",

                                        span { style: "font-size: 16px;", "\u{1F4D6}" }
                                        div {
                                            style: "flex: 1;",
                                            span { style: "font-size: 13px; color: {c.text_white};", "par {buyer_short}" }
                                        }
                                        Badge { text: ptype, color: c.accent_blue.to_string() }
                                        span {
                                            style: "font-size: 14px; color: {c.accent_green}; font-weight: 600; min-width: 60px; text-align: right;",
                                            "{amount_str}"
                                        }
                                        span {
                                            style: "font-size: 11px; color: {c.text_muted}; min-width: 40px;",
                                            "{date}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Oeuvres populaires (par nombre de ventes)
            {
                let mut popular: Vec<_> = works.iter()
                    .map(|w| {
                        let wid = w.id.clone().unwrap_or_default();
                        let purchase_count = licenses.iter()
                            .filter(|l| l.work_id.as_deref() == Some(wid.as_str()) && l.status.as_deref() == Some("active"))
                            .count();
                        (w, purchase_count)
                    })
                    .filter(|(_, count)| *count > 0)
                    .collect();
                popular.sort_by(|a, b| b.1.cmp(&a.1));
                let popular: Vec<_> = popular.into_iter().take(6).collect();

                if !popular.is_empty() {
                    rsx! {
                        div {
                            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; border: 1px solid {c.border};",

                            h3 {
                                style: "font-size: 14px; color: {c.text_white}; margin-bottom: 16px;",
                                "\u{1F3C6} Oeuvres populaires"
                            }

                            div {
                                style: "display: grid; grid-template-columns: repeat(6, 1fr); gap: 12px;",

                                for (work, count) in popular.iter() {
                                    {
                                        let title = work.title.clone().unwrap_or_else(|| "Sans titre".to_string());
                                        let status = work.status.clone().unwrap_or_else(|| "draft".to_string());
                                        let status_color = match status.as_str() {
                                            "published" => c.accent_green,
                                            "draft" => c.text_muted,
                                            _ => c.text_muted,
                                        };
                                        let read_count = *count;
                                        let wid_reader = work.id.clone().unwrap_or_default();

                                        rsx! {
                                            button {
                                                style: "padding: 12px; background: {c.bg_main}; border-radius: 8px; border: 1px solid {c.border}; text-align: center; cursor: pointer; color: {c.text_primary};",
                                                onclick: move |_| {
                                                    let mut s = state.write();
                                                    s.reader_work_id = Some(wid_reader.clone());
                                                    s.reader_chapter_id = None;
                                                    s.section = JayMangaSection::Reader;
                                                },

                                                div {
                                                    style: "width: 60px; height: 84px; background: {c.bg_hover}; border-radius: 4px; display: flex; align-items: center; justify-content: center; font-size: 24px; margin: 0 auto 8px;",
                                                    "\u{1F4D6}"
                                                }
                                                p {
                                                    style: "font-size: 12px; color: {c.text_white}; font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                                                    "{title}"
                                                }
                                                p {
                                                    style: "font-size: 11px; color: {c.accent_green}; margin-top: 2px;",
                                                    "{read_count} vente(s)"
                                                }
                                                span {
                                                    style: "font-size: 10px; color: {status_color};",
                                                    "{status}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    rsx! {}
                }
            }

            // Acces rapides secondaires
            div {
                style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 12px;",

                QuickAccessCard {
                    icon: "\u{1F4DA}".to_string(),
                    title: "Series".to_string(),
                    count: format!("{series_count}"),
                    onclick: move |_| { state.write().section = JayMangaSection::Series; },
                }
                QuickAccessCard {
                    icon: "\u{1F381}".to_string(),
                    title: "Promotions".to_string(),
                    count: String::new(),
                    onclick: move |_| { state.write().section = JayMangaSection::Promotions; },
                }
                QuickAccessCard {
                    icon: "\u{1F464}".to_string(),
                    title: "Profil".to_string(),
                    count: format!("Nv.{level}"),
                    onclick: move |_| { state.write().section = JayMangaSection::Profile; },
                }
                QuickAccessCard {
                    icon: "\u{1F3EA}".to_string(),
                    title: "Boutique".to_string(),
                    count: String::new(),
                    onclick: move |_| { state.write().section = JayMangaSection::Boutique; },
                }
                QuickAccessCard {
                    icon: "\u{1F310}".to_string(),
                    title: "Agregation".to_string(),
                    count: String::new(),
                    onclick: move |_| { state.write().section = JayMangaSection::Aggregation; },
                }
            }

            // Actions rapides
            div {
                style: "display: flex; gap: 12px;",

                ActionButton {
                    label: "Ajouter une oeuvre".to_string(),
                    icon: "\u{2795}".to_string(),
                    accent: true,
                    onclick: move |_| {
                        let mut s = state.write();
                        s.editing_work_id = None;
                        s.section = JayMangaSection::NouvelleOeuvre;
                    },
                }
                ActionButton {
                    label: "Voir les ventes".to_string(),
                    icon: "\u{1F4B0}".to_string(),
                    onclick: move |_| { state.write().section = JayMangaSection::Sales; },
                }
                ActionButton {
                    label: "Portail Agrege".to_string(),
                    icon: "\u{1F310}".to_string(),
                    onclick: move |_| { state.write().section = JayMangaSection::Aggregation; },
                }
            }
        }
    }
}
