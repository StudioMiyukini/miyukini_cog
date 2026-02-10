//! VIS-E09 — Activités visiteur (jeux, concours, pass).

use dioxus::prelude::*;
use crate::state::use_app_state;
use super::components::{Badge, ActionButton, StatCard};

/// Hub des activités visiteur.
#[component]
pub fn VisActivites() -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            h2 {
                style: "font-size: 24px; color: {c.text_white};",
                "Activites"
            }

            // Stats
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                StatCard {
                    label: "Points cumules".to_string(),
                    value: "350".to_string(),
                    icon: "⭐".to_string(),
                    color: c.accent_orange.to_string(),
                }
                StatCard {
                    label: "Jeux completes".to_string(),
                    value: "5".to_string(),
                    icon: "🎮".to_string(),
                    color: c.accent_blue.to_string(),
                }
                StatCard {
                    label: "Participations concours".to_string(),
                    value: "2".to_string(),
                    icon: "🏆".to_string(),
                    color: c.accent_green.to_string(),
                }
            }

            // Jeux disponibles
            section {
                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                    "Jeux disponibles"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                    GameCard {
                        icon: "🎯",
                        title: "Chasse au tresor",
                        description: "Trouvez les 10 indices caches dans le salon",
                        reward: "100 points",
                        progress: Some((7, 10)),
                    }
                    GameCard {
                        icon: "❓",
                        title: "Quiz du jour",
                        description: "Repondez a 5 questions sur les exposants",
                        reward: "50 points",
                        progress: None,
                    }
                    GameCard {
                        icon: "📷",
                        title: "Photo challenge",
                        description: "Prenez des photos aux points indiques",
                        reward: "75 points",
                        progress: Some((3, 5)),
                    }
                    GameCard {
                        icon: "🗺️",
                        title: "Tour des stands",
                        description: "Visitez les 15 stands partenaires",
                        reward: "150 points",
                        progress: Some((8, 15)),
                    }
                }
            }

            // Concours en cours
            section {
                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                    "Concours en cours"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    ContestCard {
                        title: "Grand tirage au sort",
                        prize: "Week-end spa pour 2",
                        end_date: "Fin le 15 février 18h",
                        participants: 234,
                        is_participating: true,
                    }
                    ContestCard {
                        title: "Meilleure photo",
                        prize: "Appareil photo instantane",
                        end_date: "Fin le 16 février 12h",
                        participants: 89,
                        is_participating: false,
                    }
                }
            }

            // Pass et récompenses
            section {
                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                    "Mes recompenses"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                    RewardCard {
                        icon: "☕",
                        title: "Cafe offert",
                        cost: 50,
                        available: true,
                    }
                    RewardCard {
                        icon: "🎁",
                        title: "Goodies",
                        cost: 100,
                        available: true,
                    }
                    RewardCard {
                        icon: "⭐",
                        title: "Pass VIP",
                        cost: 500,
                        available: false,
                    }
                }
            }
        }
    }
}

#[component]
fn GameCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    reward: &'static str,
    progress: Option<(usize, usize)>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let progress_pct = progress.map(|(current, total)| (current * 100) / total).unwrap_or(0);

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

            div {
                style: "display: flex; gap: 16px; margin-bottom: 16px;",

                div {
                    style: "width: 48px; height: 48px; background: {c.bg_hover}; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 24px;",
                    "{icon}"
                }

                div {
                    style: "flex: 1;",
                    h4 {
                        style: "font-size: 14px; color: {c.text_white}; margin-bottom: 4px;",
                        "{title}"
                    }
                    p {
                        style: "font-size: 12px; color: {c.text_muted};",
                        "{description}"
                    }
                }
            }

            // Barre de progression
            if let Some((current, total)) = progress {
                div {
                    style: "margin-bottom: 12px;",

                    div {
                        style: "display: flex; justify-content: space-between; margin-bottom: 4px;",
                        span {
                            style: "font-size: 12px; color: {c.text_muted};",
                            "Progression"
                        }
                        span {
                            style: "font-size: 12px; color: {c.text_primary};",
                            "{current}/{total}"
                        }
                    }

                    div {
                        style: "width: 100%; height: 6px; background: {c.bg_hover}; border-radius: 3px; overflow: hidden;",
                        div {
                            style: "width: {progress_pct}%; height: 100%; background: {c.accent_blue}; border-radius: 3px;",
                        }
                    }
                }
            }

            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                span {
                    style: "font-size: 13px; color: {c.accent_orange}; font-weight: 500;",
                    "🎁 {reward}"
                }

                button {
                    style: "padding: 8px 16px; background: {c.accent_blue}; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 12px;",
                    "Jouer"
                }
            }
        }
    }
}

#[component]
fn ContestCard(
    title: &'static str,
    prize: &'static str,
    end_date: &'static str,
    participants: usize,
    is_participating: bool,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; display: flex; align-items: center; gap: 16px;",

            div {
                style: "width: 56px; height: 56px; background: {c.accent_orange}20; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 28px;",
                "🏆"
            }

            div {
                style: "flex: 1;",

                h4 {
                    style: "font-size: 14px; color: {c.text_white}; margin-bottom: 4px;",
                    "{title}"
                }
                p {
                    style: "font-size: 13px; color: {c.accent_orange}; margin-bottom: 4px;",
                    "🎁 {prize}"
                }
                div {
                    style: "display: flex; gap: 12px; font-size: 12px; color: {c.text_muted};",
                    span { "⏰ {end_date}" }
                    span { "👥 {participants} participants" }
                }
            }

            if is_participating {
                Badge {
                    text: "Participe".to_string(),
                    color: c.accent_green.to_string(),
                }
            } else {
                button {
                    style: "padding: 10px 20px; background: {c.accent_blue}; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 13px;",
                    "Participer"
                }
            }
        }
    }
}

#[component]
fn RewardCard(icon: &'static str, title: &'static str, cost: usize, available: bool) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let opacity = if available { "1" } else { "0.5" };
    let btn_bg = if available { c.accent_blue } else { c.bg_hover };
    let btn_color = if available { "white" } else { c.text_muted };

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; text-align: center; opacity: {opacity};",

            span {
                style: "font-size: 40px; display: block; margin-bottom: 12px;",
                "{icon}"
            }
            h4 {
                style: "font-size: 14px; color: {c.text_white}; margin-bottom: 4px;",
                "{title}"
            }
            p {
                style: "font-size: 13px; color: {c.accent_orange}; margin-bottom: 16px;",
                "⭐ {cost} points"
            }
            button {
                style: "width: 100%; padding: 10px; background: {btn_bg}; border: none; border-radius: 6px; color: {btn_color}; cursor: pointer; font-size: 13px;",
                if available { "Echanger" } else { "Indisponible" }
            }
        }
    }
}
