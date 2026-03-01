//! Ecran Tableau de bord Alicia Home Assistante.
//!
//! 4 zones : Maison (resume global), Audio (cards pieces VAD/RMS),
//! Dernieres activites (journal unifie), Meteo (placeholder Phase 7).
//! Lit les donnees depuis le AliciaSnapshot partage.

use dioxus::prelude::*;
use miyualicia_capture::VadState;

use crate::state::use_app_state;
use super::state::{RoomLiveState, AliciaSnapshot};

// ─────────────────────────────────────────────────────────────────────────────
// Composant principal
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn DashboardScreen() -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();
    let snapshot = use_context::<Signal<Option<AliciaSnapshot>>>();

    let snap = snapshot.read();

    // Etat de chargement
    if snap.is_none() {
        return rsx! {
            div {
                style: "display: flex; align-items: center; justify-content: center; height: 200px; color: {c.text_muted};",
                "Initialisation d'Alicia..."
            }
        };
    }

    let snap_ref = snap.as_ref().expect("checked above");
    let rooms = &snap_ref.rooms;
    let activity = &snap_ref.activity_log;
    let active_mic_count = rooms.iter().filter(|r| r.mic_active).count();
    let total_rooms = rooms.len();
    let keyword = &snap_ref.wake_word_config.keyword;
    let model_info = format!("rustpotter {}", snap_ref.wake_word_config.score_mode);

    // Placeholder values for home summary (devices count, MQTT status)
    let connected_devices: u32 = 0;
    let mqtt_status = "Non connecte";
    let mqtt_color = c.text_muted;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px; max-width: 1000px;",

            // ── Zone 1 : Maison ──────────────────────────────────────────
            SectionHeader { title: "Maison" }
            div {
                style: "display: flex; gap: 16px; flex-wrap: wrap;",
                SummaryCard {
                    icon: "//",
                    label: "Pieces actives",
                    value: format!("{active_mic_count} / {total_rooms}"),
                    accent: c.accent_green,
                }
                SummaryCard {
                    icon: "//",
                    label: "Dispositifs connectes",
                    value: format!("{connected_devices}"),
                    accent: c.accent_blue,
                }
                SummaryCard {
                    icon: "//",
                    label: "MQTT",
                    value: mqtt_status.to_string(),
                    accent: mqtt_color,
                }
                SummaryCard {
                    icon: "//",
                    label: "Mot-cle",
                    value: keyword.clone(),
                    accent: c.accent_orange,
                }
            }

            // ── Zone 2 : Audio ───────────────────────────────────────────
            SectionHeader { title: "Audio" }
            div {
                style: "display: flex; gap: 16px; flex-wrap: wrap; margin-bottom: 4px;",
                SummaryCard {
                    icon: "//",
                    label: "Modele",
                    value: model_info,
                    accent: c.accent_orange,
                }
            }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px;",
                for room in rooms.iter() {
                    AudioRoomCard { room: room.clone() }
                }
            }

            // ── Zone 3 : Dernieres activites ─────────────────────────────
            SectionHeader { title: "Dernieres activites" }
            if activity.is_empty() {
                div {
                    style: "padding: 24px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; text-align: center; color: {c.text_muted};",
                    "Aucune activite enregistree. Demarre une piece ou connecte un dispositif pour commencer."
                }
            } else {
                div {
                    style: "background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; overflow: hidden; max-height: 320px; overflow-y: auto;",
                    for (i, entry) in activity.iter().enumerate() {
                        {
                            let border_top = if i > 0 { "1px solid" } else { "none" };
                            let border_val = c.border;
                            let time_str = entry.time.clone();
                            let room_str = entry.room.clone();
                            let event_str = entry.event.clone();
                            rsx! {
                                div {
                                    style: "display: flex; align-items: center; gap: 16px; padding: 10px 16px; border-top: {border_top} {border_val};",
                                    span {
                                        style: "font-size: 12px; color: {c.text_muted}; min-width: 48px; font-family: monospace;",
                                        "{time_str}"
                                    }
                                    span {
                                        style: "font-size: 12px; color: {c.accent_blue}; min-width: 160px;",
                                        "{room_str}"
                                    }
                                    span {
                                        style: "font-size: 12px; color: {c.text_primary}; flex: 1;",
                                        "{event_str}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── Zone 4 : Meteo (placeholder Phase 7) ─────────────────────
            SectionHeader { title: "Meteo" }
            div {
                style: "padding: 24px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; display: flex; align-items: center; gap: 24px;",
                // Icone meteo placeholder
                div {
                    style: "width: 64px; height: 64px; border-radius: 50%; background: {c.bg_secondary}; display: flex; align-items: center; justify-content: center; font-size: 28px; color: {c.text_muted};",
                    "--"
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 6px;",
                    p {
                        style: "font-size: 28px; font-weight: 600; color: {c.text_muted};",
                        "-- C"
                    }
                    p {
                        style: "font-size: 13px; color: {c.text_muted};",
                        "Humidite : --%"
                    }
                    p {
                        style: "font-size: 11px; color: {c.text_muted}; font-style: italic;",
                        "Widget meteo disponible en Phase 7 (capteurs domotiques)"
                    }
                }
            }

            // ── Pied ─────────────────────────────────────────────────────
            div {
                style: "margin-top: 8px; padding-top: 16px; border-top: 1px solid {c.border};",
                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "Alicia fonctionne entierement en local. Aucun flux audio ne quitte ton COG."
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Composants atomiques
// ─────────────────────────────────────────────────────────────────────────────

/// Titre de section unifie (molecule).
#[derive(Props, Clone, PartialEq)]
struct SectionHeaderProps {
    title: &'static str,
}

#[component]
fn SectionHeader(props: SectionHeaderProps) -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();

    rsx! {
        h2 {
            style: "font-size: 15px; font-weight: 600; color: {c.text_white}; text-transform: uppercase; letter-spacing: 0.6px; padding-bottom: 4px; border-bottom: 2px solid {c.accent_blue}; display: inline-block;",
            "{props.title}"
        }
    }
}

/// Carte resume compacte (compteur / info) avec icone.
#[derive(Props, Clone, PartialEq)]
struct SummaryCardProps {
    icon: &'static str,
    label: &'static str,
    value: String,
    accent: &'static str,
}

#[component]
fn SummaryCard(props: SummaryCardProps) -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();

    rsx! {
        div {
            style: "flex: 1; min-width: 180px; padding: 16px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; display: flex; flex-direction: column; gap: 8px;",
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                span {
                    style: "font-size: 16px; color: {c.text_muted};",
                    "{props.icon}"
                }
                p {
                    style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                    "{props.label}"
                }
            }
            p {
                style: "font-size: 20px; font-weight: 600; color: {props.accent};",
                "{props.value}"
            }
        }
    }
}

/// Carte d'une piece pour la zone Audio (statut micro, VAD, dernier wake word).
#[derive(Props, Clone, PartialEq)]
struct AudioRoomCardProps {
    room: RoomLiveState,
}

impl PartialEq for RoomLiveState {
    fn eq(&self, other: &Self) -> bool {
        self.room_id == other.room_id
            && self.mic_active == other.mic_active
            && self.vad_state == other.vad_state
            && self.device_name == other.device_name
    }
}

#[component]
fn AudioRoomCard(props: AudioRoomCardProps) -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();
    let room = &props.room;

    let mic_color = if room.mic_active { c.accent_green } else { c.text_muted };
    let mic_label = if room.mic_active { "Micro actif" } else { "Micro inactif" };

    let vad_active = matches!(room.vad_state, VadState::Speech | VadState::MaybeEnd);
    let vad_color = if vad_active { c.accent_orange } else { c.text_muted };
    let vad_label = if vad_active { "Voix detectee" } else { "Silence" };

    let (wake_label, wake_color) = match &room.last_detection {
        Some(det) => {
            let label = det.detected_at.clone();
            (label, c.text_primary)
        }
        None => ("Jamais".to_string(), c.text_muted),
    };

    // RMS bar
    let rms_val = room.last_rms;
    let rms_bar_pct = (rms_val * 500.0).min(100.0) as u32;

    let error_msg = room.error.clone();
    let room_icon = room.icon.clone();
    let room_name = room.room_name.clone();
    let is_active = room.mic_active;

    rsx! {
        div {
            style: "padding: 16px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; display: flex; flex-direction: column; gap: 10px;",

            // Header piece
            div {
                style: "display: flex; align-items: center; gap: 10px;",
                span { style: "font-size: 22px;", "{room_icon}" }
                h3 {
                    style: "font-size: 14px; font-weight: 600; color: {c.text_white};",
                    "{room_name}"
                }
            }

            // Indicateurs
            div {
                style: "display: flex; flex-direction: column; gap: 6px;",

                // Micro
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    div {
                        style: "width: 7px; height: 7px; border-radius: 50%; background: {mic_color};",
                    }
                    span {
                        style: "font-size: 12px; color: {mic_color};",
                        "{mic_label}"
                    }
                }

                // VAD
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    div {
                        style: "width: 7px; height: 7px; border-radius: 50%; background: {vad_color};",
                    }
                    span {
                        style: "font-size: 12px; color: {vad_color};",
                        "{vad_label}"
                    }
                }

                // RMS bar (visible only when active)
                if is_active {
                    div {
                        style: "width: 100%; height: 3px; background: {c.bg_secondary}; border-radius: 2px; overflow: hidden; margin-top: 2px;",
                        div {
                            style: "width: {rms_bar_pct}%; height: 100%; background: {c.accent_orange}; border-radius: 2px; transition: width 0.1s;",
                        }
                    }
                }

                // Dernier wake word
                div {
                    style: "display: flex; align-items: center; gap: 6px; margin-top: 2px;",
                    span {
                        style: "font-size: 11px; color: {c.text_secondary};",
                        "Dernier \"Hey Alicia\" :"
                    }
                    span {
                        style: "font-size: 11px; color: {wake_color};",
                        "{wake_label}"
                    }
                }
            }

            // Erreur eventuelle
            if let Some(ref err) = error_msg {
                div {
                    style: "font-size: 11px; color: {c.accent_red}; padding: 6px 8px; background: {c.bg_secondary}; border-radius: 4px;",
                    "{err}"
                }
            }
        }
    }
}
