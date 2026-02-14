//! Vue pour Lord of the Click (MiyuClicker).
//! Miyukini Survivor est intégré via SurvivorEmbed dans l'onglet lord_of_the_castle.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;

/// Proprietes de la vue de jeu.
#[derive(Props, Clone, PartialEq)]
pub struct GameViewProps {
    pub service_id: String,
}

/// Vue d'un jeu.
#[component]
pub fn GameView(props: GameViewProps) -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
    let conns = use_service_connections();
    let service = state.read().services.iter()
        .find(|s| s.id == props.service_id)
        .cloned();

    let (title, subtitle, description) = (
        "Lord of the Click",
        "Idle/Clicker + Strategie",
        "Cliquez pour accumuler des ressources, developpez votre royaume et conquerez de nouvelles terres.",
    );

    let icon = service.as_ref().map_or_else(|| "🎮".to_string(), |s| s.icon.clone());

    let base_path = conns.read().miyuclicker_data_dir.clone();
    let game_stats = load_miyuclicker_stats(&base_path);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px; height: 100%;",

            // Banner du jeu
            div {
                style: "background: linear-gradient(135deg, #1a1a2e 0%, #2a3f5f 50%, #1a1a2e 100%); border-radius: 12px; padding: 32px; position: relative; overflow: hidden;",

                div {
                    style: "position: relative; z-index: 1; display: flex; gap: 24px; align-items: center;",

                    // Grande icone
                    div {
                        style: "width: 120px; height: 120px; background: linear-gradient(135deg, {c.accent_blue}40 0%, {c.accent_blue}20 100%); border-radius: 16px; display: flex; align-items: center; justify-content: center; font-size: 56px; box-shadow: 0 8px 32px rgba(0,0,0,0.3);",
                        "{icon}"
                    }

                    div {
                        style: "flex: 1;",

                        h1 {
                            style: "font-size: 32px; font-weight: 700; color: white; margin-bottom: 4px;",
                            "{title}"
                        }
                        p {
                            style: "font-size: 14px; color: {c.accent_blue}; margin-bottom: 12px;",
                            "{subtitle}"
                        }
                        p {
                            style: "font-size: 14px; color: {c.text_secondary}; max-width: 500px; line-height: 1.6;",
                            "{description}"
                        }
                    }

                    div {
                        style: "padding: 12px 24px; background: {c.bg_secondary}; border-radius: 8px; font-size: 14px; color: {c.text_secondary}; border: 1px solid {c.border};",
                        "Jouez depuis cet onglet — intégration complète à venir."
                    }
                }
            }

            // Contenu en 2 colonnes
            div {
                style: "display: grid; grid-template-columns: 2fr 1fr; gap: 24px; flex: 1;",

                // Stats et progression
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    // Statistiques
                    div {
                        style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                        h3 {
                            style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                            "📊 Statistiques"
                        }

                        if let Some(ref stats) = game_stats {
                            div {
                                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                                GameStat { label: "Temps de jeu".to_string(), value: stats.play_time.clone() }
                                GameStat { label: "Population".to_string(), value: stats.population.clone() }
                                GameStat { label: "Ouvriers".to_string(), value: stats.ouvriers.clone() }
                            }
                            div {
                                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; margin-top: 16px;",

                                GameStat { label: "Bâtisseurs".to_string(), value: stats.batisseurs.clone() }
                                GameStat { label: "Soldats".to_string(), value: stats.soldiers.clone() }
                                GameStat { label: "Nourriture".to_string(), value: stats.food.clone() }
                            }
                        } else {
                            p {
                                style: "font-size: 13px; color: {c.text_muted}; text-align: center; padding: 32px;",
                                "Aucune sauvegarde trouvee"
                            }
                        }
                    }
                }

                // Actions
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                        h3 {
                            style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                            "Actions"
                        }

                        div {
                            style: "display: flex; flex-direction: column; gap: 8px;",

                            GameAction { icon: "📖", label: "Guide du jeu" }
                            GameAction { icon: "⚙️", label: "Options" }
                            GameAction { icon: "💾", label: "Sauvegardes" }
                        }
                    }

                    // Slots de sauvegarde MiyuClicker
                    div {
                        style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                        h3 {
                            style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                            "💾 Slots de sauvegarde"
                        }
                        {
                            let slots = miyuclicker::save::slot_list(&base_path);
                            rsx! {
                                div {
                                    style: "display: flex; flex-direction: column; gap: 8px;",
                                    for slot in slots.iter() {
                                        div {
                                            style: "display: flex; justify-content: space-between; align-items: center; padding: 10px 12px; background: {c.bg_hover}; border-radius: 4px; font-size: 13px;",

                                            span { style: "color: {c.text_primary};", { format!("Slot {}", slot.slot_id) } }
                                            span {
                                                style: "color: {c.text_muted};",
                                                { if slot.occupied { slot.summary.clone().unwrap_or_else(|| "Occupe".to_string()) } else { "Vide".to_string() } }
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

/// Stats de jeu extraites d'une sauvegarde.
struct MiyuClickerStats {
    play_time: String,
    population: String,
    ouvriers: String,
    batisseurs: String,
    soldiers: String,
    food: String,
}

/// Charge les stats depuis le premier slot occupe.
fn load_miyuclicker_stats(data_dir: &std::path::Path) -> Option<MiyuClickerStats> {
    let slots = miyuclicker::save::slot_list(data_dir);
    let first_occupied = slots.iter().find(|s| s.occupied)?;
    let state = miyuclicker::save::slot_read(data_dir, first_occupied.slot_id).ok()?;

    let hours = (state.temps_simule_s / 3600.0) as u32;
    let minutes = ((state.temps_simule_s % 3600.0) / 60.0) as u32;

    Some(MiyuClickerStats {
        play_time: format!("{hours}h {minutes:02}m"),
        population: state.pop_totale().to_string(),
        ouvriers: state.ouvriers.to_string(),
        batisseurs: state.batisseurs.to_string(),
        soldiers: state.soldats.to_string(),
        food: format!("{:.0}", state.nourriture),
    })
}

#[component]
fn GameStat(label: String, value: String) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "text-align: center;",

            p {
                style: "font-size: 24px; font-weight: 600; color: {c.accent_blue};",
                "{value}"
            }
            p {
                style: "font-size: 12px; color: {c.text_secondary};",
                "{label}"
            }
        }
    }
}

#[component]
fn GameAction(icon: &'static str, label: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        button {
            style: "display: flex; align-items: center; gap: 12px; padding: 10px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 13px; width: 100%; text-align: left; transition: background 0.2s;",

            span { "{icon}" }
            span { "{label}" }
        }
    }
}
