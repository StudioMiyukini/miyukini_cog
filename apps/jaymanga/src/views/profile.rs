//! Profil de progression JayManga — niveau, XP, streaks, badges, statistiques, historique.

use dioxus::prelude::*;
use jaymanga::domain::gamification::{
    gamification_level_for_xp, gamification_level_progress, gamification_xp_for_next_level,
    LEVEL_THRESHOLDS,
};
use chrono::{Datelike, NaiveDate};
use miyukini_service_ui::use_palette;
use crate::use_db;
use super::components::{PageHeader, EmptyState, Badge, StatCard};
use super::JayMangaState;

#[component]
pub fn Profile(state: Signal<JayMangaState>) -> Element {
    let c = use_palette();
    let db = use_db();

    let progression = db.progression_get().ok().flatten();
    let badges = db.badge_list().unwrap_or_default();
    let favorites = db.favorite_list().unwrap_or_default();

    let total_xp = progression.as_ref().and_then(|p| p.total_xp).unwrap_or(0);
    let current_streak = progression.as_ref().and_then(|p| p.current_streak).unwrap_or(0);
    let longest_streak = progression.as_ref().and_then(|p| p.longest_streak).unwrap_or(0);
    let pages_read = progression.as_ref().and_then(|p| p.total_pages_read).unwrap_or(0);
    let works_completed = progression.as_ref().and_then(|p| p.total_works_completed).unwrap_or(0);
    // genres_explored et cogs_visited sont des JSON Vec<String> — on compte les elements
    let genres_explored = progression.as_ref()
        .and_then(|p| p.genres_explored.as_ref())
        .and_then(|s| serde_json::from_str::<Vec<String>>(s).ok())
        .map(|v| v.len())
        .unwrap_or(0);
    let cogs_visited = progression.as_ref()
        .and_then(|p| p.cogs_visited.as_ref())
        .and_then(|s| serde_json::from_str::<Vec<String>>(s).ok())
        .map(|v| v.len())
        .unwrap_or(0);

    // Temps de lecture estime (env. 30 secondes par page -> pages_read * 0.5 min)
    let reading_time_min = (pages_read as f64 * 0.5) as i64;
    let reading_time_str = if reading_time_min >= 60 {
        format!("{}h{:02}", reading_time_min / 60, reading_time_min % 60)
    } else {
        format!("{reading_time_min} min")
    };

    let (level, level_name) = gamification_level_for_xp(total_xp);
    let level_progress = gamification_level_progress(total_xp);
    let xp_for_next = gamification_xp_for_next_level(total_xp);
    let progress_pct = (level_progress * 100.0) as i32;

    let next_level_xp = total_xp + xp_for_next;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            PageHeader {
                title: "\u{1F464} Profil de progression".to_string(),
                subtitle: "Votre parcours de lecture".to_string(),
            }

            // -- Niveau et XP --
            div {
                style: "background: {c.bg_secondary}; border-radius: 12px; padding: 24px; border: 1px solid {c.border};",

                div {
                    style: "display: flex; align-items: center; gap: 20px;",

                    // Avatar niveau
                    div {
                        style: "width: 72px; height: 72px; background: linear-gradient(135deg, #FFD700, #FF6B35); border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 32px; font-weight: 700; color: white; flex-shrink: 0;",
                        "{level}"
                    }

                    div {
                        style: "flex: 1;",
                        div {
                            style: "display: flex; align-items: baseline; gap: 8px; margin-bottom: 4px;",
                            h3 {
                                style: "font-size: 22px; color: {c.text_white}; font-weight: 700;",
                                "{level_name}"
                            }
                            span {
                                style: "font-size: 14px; color: {c.text_muted};",
                                "Niveau {level}"
                            }
                        }

                        // Barre XP
                        div {
                            style: "margin-top: 8px;",
                            div {
                                style: "display: flex; justify-content: space-between; margin-bottom: 4px;",
                                span {
                                    style: "font-size: 12px; color: {c.text_secondary};",
                                    "{total_xp} XP"
                                }
                                span {
                                    style: "font-size: 12px; color: {c.text_muted};",
                                    {
                                        if xp_for_next > 0 {
                                            format!("{} XP", next_level_xp)
                                        } else {
                                            "Niveau maximum !".to_string()
                                        }
                                    }
                                }
                            }
                            div {
                                style: "width: 100%; height: 12px; background: {c.bg_hover}; border-radius: 6px; overflow: hidden;",
                                div {
                                    style: "width: {progress_pct}%; height: 100%; background: linear-gradient(90deg, #FFD700, #FF6B35); border-radius: 6px; transition: width 0.5s;",
                                }
                            }
                            if xp_for_next > 0 {
                                p {
                                    style: "font-size: 11px; color: {c.text_muted}; margin-top: 4px;",
                                    "Encore {xp_for_next} XP pour atteindre le prochain niveau"
                                }
                            }
                        }
                    }
                }

                // Paliers de niveaux
                div {
                    style: "display: flex; gap: 8px; margin-top: 20px; overflow-x: auto;",

                    for &(lvl, name, threshold) in LEVEL_THRESHOLDS.iter() {
                        {
                            let is_current = lvl == level;
                            let is_reached = total_xp >= threshold;
                            let bg = if is_current { "#FFD70030".to_string() } else if is_reached { format!("{}20", c.accent_green) } else { c.bg_hover.to_string() };
                            let text_c = if is_current { "#FFD700" } else if is_reached { c.accent_green } else { c.text_muted };
                            let border = if is_current { "2px solid #FFD700".to_string() } else { format!("1px solid {}", c.border) };

                            rsx! {
                                div {
                                    style: "flex: 1; min-width: 80px; padding: 8px; background: {bg}; border: {border}; border-radius: 8px; text-align: center;",
                                    p {
                                        style: "font-size: 16px; font-weight: 600; color: {text_c};",
                                        "{lvl}"
                                    }
                                    p {
                                        style: "font-size: 10px; color: {text_c}; margin-top: 2px;",
                                        "{name}"
                                    }
                                    p {
                                        style: "font-size: 9px; color: {c.text_muted}; margin-top: 2px;",
                                        "{threshold} XP"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // -- Streak --
            div {
                style: "display: flex; gap: 16px;",

                // Streak actuel
                div {
                    style: "flex: 1; background: {c.bg_secondary}; border-radius: 12px; padding: 24px; border: 1px solid {c.border}; display: flex; align-items: center; gap: 16px;",

                    div {
                        style: "width: 64px; height: 64px; background: #FF450020; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 36px; flex-shrink: 0;",
                        "\u{1F525}"
                    }
                    div {
                        h3 {
                            style: "font-size: 28px; color: #FF4500; font-weight: 700;",
                            "{current_streak} jours"
                        }
                        p {
                            style: "font-size: 13px; color: {c.text_secondary};",
                            "Serie actuelle"
                        }
                    }
                }

                // Record
                div {
                    style: "flex: 1; background: {c.bg_secondary}; border-radius: 12px; padding: 24px; border: 1px solid {c.border}; display: flex; align-items: center; gap: 16px;",

                    div {
                        style: "width: 64px; height: 64px; background: #FFD70020; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 36px; flex-shrink: 0;",
                        "\u{1F3C6}"
                    }
                    div {
                        h3 {
                            style: "font-size: 28px; color: #FFD700; font-weight: 700;",
                            "{longest_streak} jours"
                        }
                        p {
                            style: "font-size: 13px; color: {c.text_secondary};",
                            "Record personnel"
                        }
                    }
                }
            }

            // -- Calendrier hebdomadaire streak --
            {
                // Generer les 7 derniers jours (Lun->Dim de cette semaine)
                let today = chrono::Local::now().date_naive();
                let weekday_num = today.weekday().num_days_from_monday(); // 0=Lun, 6=Dim
                let week_start = today - chrono::TimeDelta::days(weekday_num as i64);
                let day_labels = ["L", "M", "M", "J", "V", "S", "D"];

                // Calculer les jours valides : les `current_streak` derniers jours en remontant depuis aujourd'hui
                let streak_days: Vec<NaiveDate> = (0..current_streak as i64)
                    .map(|d| today - chrono::TimeDelta::days(d))
                    .collect();

                rsx! {
                    div {
                        style: "background: {c.bg_secondary}; border-radius: 12px; padding: 20px; border: 1px solid {c.border};",

                        h4 {
                            style: "font-size: 13px; color: {c.text_white}; margin-bottom: 12px;",
                            "\u{1F4C5} Cette semaine"
                        }

                        div {
                            style: "display: flex; gap: 8px; justify-content: space-between;",

                            for day_offset in 0..7i64 {
                                {
                                    let day_date = week_start + chrono::TimeDelta::days(day_offset);
                                    let is_today = day_date == today;
                                    let is_validated = streak_days.contains(&day_date);
                                    let is_future = day_date > today;
                                    let label = day_labels[day_offset as usize];

                                    let bg = if is_validated { "#FF450030".to_string() } else if is_today { format!("{}20", c.accent_blue) } else { c.bg_hover.to_string() };
                                    let border_col = if is_today { c.accent_blue } else if is_validated { "#FF4500" } else { c.border };
                                    let text_col = if is_future { c.text_muted } else if is_validated { "#FF4500" } else { c.text_secondary };

                                    let opacity = if is_future { "0.5" } else { "1" };

                                    rsx! {
                                        div {
                                            style: "flex: 1; text-align: center; padding: 10px 4px; background: {bg}; border: 2px solid {border_col}; border-radius: 8px; opacity: {opacity};",

                                            p {
                                                style: "font-size: 11px; color: {c.text_muted}; margin-bottom: 4px;",
                                                "{label}"
                                            }
                                            p {
                                                style: "font-size: 14px; color: {text_col}; font-weight: 600;",
                                                {day_date.format("%d").to_string()}
                                            }
                                            if is_validated {
                                                p {
                                                    style: "font-size: 14px; margin-top: 2px;",
                                                    "\u{2705}"
                                                }
                                            } else if is_today {
                                                p {
                                                    style: "font-size: 14px; margin-top: 2px;",
                                                    "\u{1F4D6}"
                                                }
                                            } else if !is_future {
                                                p {
                                                    style: "font-size: 14px; margin-top: 2px; opacity: 0.3;",
                                                    "\u{2014}"
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

            // -- Statistiques --
            h3 {
                style: "font-size: 16px; color: {c.text_white};",
                "\u{1F4CA} Statistiques"
            }
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                StatCard { label: "Pages lues".to_string(), value: pages_read.to_string(), icon: "\u{1F4C4}".to_string(), color: c.accent_blue.to_string() }
                StatCard { label: "Oeuvres terminees".to_string(), value: works_completed.to_string(), icon: "\u{1F4DA}".to_string(), color: c.accent_green.to_string() }
                StatCard { label: "Temps de lecture".to_string(), value: reading_time_str.clone(), icon: "\u{23F1}\u{FE0F}".to_string(), color: "#FF6B35".to_string() }
            }
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                StatCard { label: "Genres explores".to_string(), value: genres_explored.to_string(), icon: "\u{1F3AD}".to_string(), color: "#8b5cf6".to_string() }
                StatCard { label: "COGs visites".to_string(), value: cogs_visited.to_string(), icon: "\u{1F310}".to_string(), color: c.accent_orange.to_string() }
                StatCard { label: "Favoris".to_string(), value: favorites.len().to_string(), icon: "\u{2764}\u{FE0F}".to_string(), color: "#ef4444".to_string() }
            }

            // -- Badges --
            h3 {
                style: "font-size: 16px; color: {c.text_white};",
                "\u{1F3C5} Badges ({badges.len()})"
            }

            if badges.is_empty() {
                EmptyState {
                    title: "Aucun badge".to_string(),
                    message: "Lisez des manga pour debloquer des badges !".to_string(),
                    icon: "\u{1F3C5}".to_string(),
                }
            } else {
                div {
                    style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px;",

                    for badge in badges.iter() {
                        {
                            let name = badge.badge_name.clone().unwrap_or_else(|| "Badge".to_string());
                            let category = badge.badge_category.clone().unwrap_or_else(|| "reading".to_string());
                            let description = String::new(); // ReaderBadge n'a pas de champ description
                            let earned = badge.earned_at.clone()
                                .map(|d| if d.len() >= 10 { d[..10].to_string() } else { d })
                                .unwrap_or_default();

                            let (badge_icon, badge_color) = match category.as_str() {
                                "reading" => ("\u{1F4D6}", c.accent_blue),
                                "social" => ("\u{1F465}", "#8b5cf6"),
                                "collection" => ("\u{1F4DA}", c.accent_green),
                                "streak" => ("\u{1F525}", "#FF4500"),
                                "achievement" => ("\u{1F3C6}", "#FFD700"),
                                _ => ("\u{1F3C5}", c.text_muted),
                            };

                            rsx! {
                                div {
                                    style: "padding: 16px; background: {c.bg_secondary}; border-radius: 8px; border: 2px solid {badge_color}40; text-align: center;",

                                    span { style: "font-size: 32px; display: block; margin-bottom: 8px;", "{badge_icon}" }
                                    h4 {
                                        style: "font-size: 13px; color: {c.text_white}; font-weight: 600; margin-bottom: 4px;",
                                        "{name}"
                                    }
                                    if !description.is_empty() {
                                        p {
                                            style: "font-size: 11px; color: {c.text_secondary}; margin-bottom: 6px;",
                                            "{description}"
                                        }
                                    }
                                    div {
                                        style: "display: flex; justify-content: center; gap: 6px;",
                                        Badge { text: category.clone(), color: badge_color.to_string() }
                                    }
                                    if !earned.is_empty() {
                                        p {
                                            style: "font-size: 10px; color: {c.text_muted}; margin-top: 4px;",
                                            "Obtenu le {earned}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // -- Historique de lecture --
            h3 {
                style: "font-size: 16px; color: {c.text_white};",
                "\u{1F4DC} Lectures recentes"
            }

            {
                let mut recent_favs: Vec<_> = favorites.iter()
                    .filter(|f| f.last_synced_at.is_some())
                    .collect();
                recent_favs.sort_by(|a, b| {
                    let da = a.last_synced_at.as_deref().unwrap_or("");
                    let db_val = b.last_synced_at.as_deref().unwrap_or("");
                    db_val.cmp(da)
                });
                let recent_favs: Vec<_> = recent_favs.into_iter().take(10).collect();

                if recent_favs.is_empty() {
                    rsx! {
                        EmptyState {
                            title: "Aucune lecture recente".to_string(),
                            message: "Votre historique de lecture apparaitra ici.".to_string(),
                            icon: "\u{1F4DC}".to_string(),
                        }
                    }
                } else {
                    rsx! {
                        div {
                            style: "display: flex; flex-direction: column; gap: 6px;",

                            for fav in recent_favs.iter() {
                                {
                                    let title = fav.cached_title.clone().unwrap_or_else(|| "Titre inconnu".to_string());
                                    let progress = fav.reading_progress.unwrap_or(0.0);
                                    let progress_pct = (progress * 100.0) as i32;
                                    let last_read = fav.last_synced_at.clone()
                                        .map(|d| if d.len() >= 10 { d[..10].to_string() } else { d })
                                        .unwrap_or_default();
                                    let chapter = fav.last_read_chapter.map(|n| format!("Ch. {n}")).unwrap_or_default();

                                    rsx! {
                                        div {
                                            style: "display: flex; align-items: center; gap: 12px; padding: 10px 16px; background: {c.bg_secondary}; border-radius: 4px;",

                                            span { style: "font-size: 16px;", "\u{1F4D6}" }
                                            span {
                                                style: "flex: 1; font-size: 13px; color: {c.text_white};",
                                                "{title}"
                                            }
                                            if !chapter.is_empty() {
                                                span {
                                                    style: "font-size: 12px; color: {c.text_secondary};",
                                                    "{chapter}"
                                                }
                                            }
                                            span {
                                                style: "font-size: 12px; color: {c.accent_blue};",
                                                "{progress_pct}%"
                                            }
                                            span {
                                                style: "font-size: 11px; color: {c.text_muted};",
                                                "{last_read}"
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
}
