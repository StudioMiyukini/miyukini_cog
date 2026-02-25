//! Statistiques lecteurs JayManga — progression, favoris, badges.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{PageHeader, StatCard, EmptyState};
use super::JayMangaState;

#[component]
pub fn ReaderStats(state: Signal<JayMangaState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    let db = &conns.read().jaymanga;
    let progression = db.progression_get().ok().flatten();
    let favorites = db.favorite_list().unwrap_or_default();
    let badges = db.badge_list().unwrap_or_default();

    let total_xp = progression.as_ref().and_then(|p| p.total_xp).unwrap_or(0);
    let current_level = progression.as_ref().and_then(|p| p.current_level).unwrap_or(1);
    let current_streak = progression.as_ref().and_then(|p| p.current_streak).unwrap_or(0);
    let pages_read = progression.as_ref().and_then(|p| p.total_pages_read).unwrap_or(0);
    let works_completed = progression.as_ref().and_then(|p| p.total_works_completed).unwrap_or(0);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            PageHeader {
                title: "📈 Lecteurs".to_string(),
                subtitle: "Progression de lecture et statistiques".to_string(),
            }

            // Statistiques globales
            div {
                style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 16px;",

                StatCard { label: "Niveau".to_string(), value: current_level.to_string(), icon: "⭐".to_string(), color: "#eab308".to_string() }
                StatCard { label: "XP total".to_string(), value: total_xp.to_string(), icon: "✨".to_string(), color: c.accent_blue.to_string() }
                StatCard { label: "Série actuelle".to_string(), value: format!("{current_streak}j"), icon: "🔥".to_string(), color: c.accent_orange.to_string() }
                StatCard { label: "Pages lues".to_string(), value: pages_read.to_string(), icon: "📄".to_string(), color: "#8b5cf6".to_string() }
                StatCard { label: "Œuvres finies".to_string(), value: works_completed.to_string(), icon: "🏆".to_string(), color: c.accent_green.to_string() }
            }

            // Favoris
            h3 {
                style: "font-size: 16px; color: {c.text_white};",
                "❤️ Favoris ({favorites.len()})"
            }

            if favorites.is_empty() {
                EmptyState {
                    title: "Aucun favori".to_string(),
                    message: "Les œuvres ajoutées en favoris par les lecteurs apparaîtront ici.".to_string(),
                    icon: "❤️".to_string(),
                }
            } else {
                div {
                    style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px;",

                    for fav in favorites.iter() {
                        {
                            let title = fav.cached_title.clone().unwrap_or_else(|| "Titre inconnu".to_string());
                            let progress = fav.reading_progress.unwrap_or(0.0);
                            let progress_pct = (progress * 100.0) as i32;
                            let status = fav.purchase_status.clone().unwrap_or_else(|| "demo".to_string());

                            let status_color = match status.as_str() {
                                "owned" => c.accent_green,
                                "demo" => c.accent_blue,
                                _ => c.text_muted,
                            };

                            rsx! {
                                div {
                                    style: "padding: 16px; background: {c.bg_secondary}; border-radius: 8px; border: 1px solid {c.border};",

                                    h4 {
                                        style: "font-size: 13px; color: {c.text_white}; margin-bottom: 8px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                                        "{title}"
                                    }

                                    // Barre de progression
                                    div {
                                        style: "width: 100%; height: 6px; background: {c.bg_hover}; border-radius: 3px; overflow: hidden; margin-bottom: 8px;",
                                        div {
                                            style: "width: {progress_pct}%; height: 100%; background: {c.accent_blue}; border-radius: 3px;",
                                        }
                                    }

                                    div {
                                        style: "display: flex; justify-content: space-between; font-size: 11px;",
                                        span { style: "color: {c.text_muted};", "{progress_pct}%" }
                                        span { style: "color: {status_color};", "{status}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Badges
            h3 {
                style: "font-size: 16px; color: {c.text_white};",
                "🏅 Badges ({badges.len()})"
            }

            if badges.is_empty() {
                EmptyState {
                    title: "Aucun badge".to_string(),
                    message: "Les badges obtenus par les lecteurs apparaîtront ici.".to_string(),
                    icon: "🏅".to_string(),
                }
            } else {
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 12px;",

                    for badge in badges.iter() {
                        {
                            let name = badge.badge_name.clone().unwrap_or_else(|| "Badge".to_string());
                            let category = badge.badge_category.clone().unwrap_or_else(|| "reading".to_string());
                            let earned = badge.earned_at.clone()
                                .map(|d| if d.len() >= 10 { d[..10].to_string() } else { d })
                                .unwrap_or_default();

                            let badge_icon = match category.as_str() {
                                "reading" => "📖",
                                "social" => "👥",
                                "collection" => "📚",
                                "streak" => "🔥",
                                "achievement" => "🏆",
                                _ => "🏅",
                            };

                            rsx! {
                                div {
                                    style: "display: flex; align-items: center; gap: 8px; padding: 10px 16px; background: {c.bg_secondary}; border-radius: 8px; border: 1px solid {c.border};",

                                    span { style: "font-size: 20px;", "{badge_icon}" }
                                    div {
                                        p {
                                            style: "font-size: 13px; color: {c.text_white}; font-weight: 500;",
                                            "{name}"
                                        }
                                        p {
                                            style: "font-size: 11px; color: {c.text_muted};",
                                            "{earned}"
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
}
