//! Ecran Configuration des pieces Alicia Home Assistante.
//!
//! Grille 2x2 avec les 4 pieces surveillees.
//! Chaque piece : micro (start/stop, device dropdown, VAD), dispositifs associes.
//! Lit les donnees depuis le AliciaSnapshot et controle le AliciaService.

use dioxus::prelude::*;
use miyualicia_capture::VadState;

use crate::state::use_app_state;
use super::state::{RoomLiveState, SharedAliciaService, AliciaSnapshot};

// ─────────────────────────────────────────────────────────────────────────────
// Composant principal
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn RoomsScreen() -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();
    let snapshot = use_context::<Signal<Option<AliciaSnapshot>>>();

    let snap = snapshot.read();

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
    let devices = &snap_ref.available_devices;

    // Collecter les noms de devices pour le dropdown
    let device_names: Vec<String> = devices.iter().map(|d| d.name.clone()).collect();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px; max-width: 1000px;",

            // Header
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",
                h2 {
                    style: "font-size: 16px; font-weight: 600; color: {c.text_white};",
                    "Configuration des pieces"
                }
                RefreshDevicesButton {}
            }

            // Grille 2x2
            div {
                style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",
                for room in rooms.iter() {
                    RoomConfigCard {
                        room: room.clone(),
                        device_names: device_names.clone(),
                    }
                }
            }

            // Note de pied
            div {
                style: "padding: 16px; background: {c.bg_secondary}; border-radius: 8px;",
                p {
                    style: "font-size: 13px; color: {c.text_secondary};",
                    "Les devices audio sont detectes via cpal. "
                    "Selectionne un device puis clique Demarrer pour activer la capture sur une piece. "
                    "Les dispositifs domotiques seront configurables une fois le crate miyualicia-devices integre."
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Bouton Rafraichir les devices
// ─────────────────────────────────────────────────────────────────────────────

#[component]
fn RefreshDevicesButton() -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();
    let voice_svc = use_context::<Signal<Option<SharedAliciaService>>>();

    rsx! {
        button {
            style: "padding: 6px 14px; background: {c.bg_secondary}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 13px;",
            onclick: move |_| {
                let svc_ref = voice_svc.read();
                if let Some(svc) = svc_ref.as_ref() {
                    if let Ok(guard) = svc.lock() {
                        match guard.refresh_devices() {
                            Ok(devs) => tracing::info!("Devices rafraichis: {}", devs.len()),
                            Err(e) => tracing::error!("Erreur rafraichissement devices: {e}"),
                        }
                    }
                }
            },
            "Rafraichir devices"
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Carte de configuration d'une piece
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct RoomConfigCardProps {
    room: RoomLiveState,
    device_names: Vec<String>,
}

#[component]
fn RoomConfigCard(props: RoomConfigCardProps) -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();
    let voice_svc = use_context::<Signal<Option<SharedAliciaService>>>();
    let room = &props.room;

    // Device selectionne (signal local pour le dropdown)
    let mut selected_device = use_signal(|| room.device_name.clone().unwrap_or_default());

    let status_color = if room.mic_active { c.accent_green } else { c.text_muted };
    let status_label = if room.mic_active { "Actif" } else { "Inactif" };

    let vad_active = matches!(room.vad_state, VadState::Speech | VadState::MaybeEnd);
    let vad_color = if vad_active { c.accent_orange } else { c.text_muted };
    let vad_label = if vad_active { "Voix" } else { "Silence" };

    // Seuils pour les barres de progression (en pourcentage 0-100)
    let vad_pct = (room.vad_threshold * 100.0) as u32;
    let wake_pct = (room.wake_threshold * 100.0) as u32;

    let room_id = room.room_id.clone();
    let room_icon = room.icon.clone();
    let room_name = room.room_name.clone();
    let is_active = room.mic_active;
    let error_msg = room.error.clone();

    // Texte RMS live
    let rms_val = room.last_rms;
    let rms_text = format!("{rms_val:.3}");
    let rms_bar_pct = (rms_val * 500.0).min(100.0) as u32;

    rsx! {
        div {
            style: "padding: 20px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; display: flex; flex-direction: column; gap: 16px;",

            // Header
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    span { style: "font-size: 20px;", "{room_icon}" }
                    h3 {
                        style: "font-size: 15px; font-weight: 600; color: {c.text_white};",
                        "{room_name}"
                    }
                }
                div {
                    style: "display: flex; align-items: center; gap: 6px; padding: 3px 10px; border-radius: 12px; background: {status_color}20;",
                    div {
                        style: "width: 6px; height: 6px; border-radius: 50%; background: {status_color};",
                    }
                    span {
                        style: "font-size: 11px; font-weight: 500; color: {status_color};",
                        "{status_label}"
                    }
                }
            }

            // ── Section Micro ────────────────────────────────────────────
            div {
                style: "padding: 12px; background: {c.bg_secondary}; border-radius: 6px; display: flex; flex-direction: column; gap: 10px;",
                p {
                    style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px; font-weight: 600;",
                    "Microphone"
                }

                // Device audio dropdown
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label {
                        style: "font-size: 11px; color: {c.text_secondary};",
                        "Device audio"
                    }
                    select {
                        style: "padding: 8px 12px; background: {c.bg_card}; border-radius: 4px; border: 1px solid {c.border}; color: {c.text_primary}; font-size: 13px;",
                        disabled: is_active,
                        onchange: move |evt: Event<FormData>| {
                            selected_device.set(evt.value());
                        },
                        option {
                            value: "",
                            selected: selected_device.read().is_empty(),
                            "-- Selectionner un device --"
                        }
                        for dev_name in props.device_names.iter() {
                            {
                                let is_selected = *selected_device.read() == *dev_name;
                                let val = dev_name.clone();
                                rsx! {
                                    option {
                                        value: "{val}",
                                        selected: is_selected,
                                        "{val}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Indicateurs live
                if is_active {
                    // VAD indicator
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        div {
                            style: "width: 8px; height: 8px; border-radius: 50%; background: {vad_color};",
                        }
                        span {
                            style: "font-size: 12px; color: {vad_color};",
                            "{vad_label}"
                        }
                        span {
                            style: "font-size: 11px; color: {c.text_muted}; margin-left: auto; font-family: monospace;",
                            "RMS: {rms_text}"
                        }
                    }

                    // RMS bar
                    div {
                        style: "width: 100%; height: 4px; background: {c.bg_card}; border-radius: 2px; overflow: hidden;",
                        div {
                            style: "width: {rms_bar_pct}%; height: 100%; background: {c.accent_orange}; border-radius: 2px; transition: width 0.1s;",
                        }
                    }
                }

                // Seuil VAD
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    div {
                        style: "display: flex; align-items: center; justify-content: space-between;",
                        label {
                            style: "font-size: 11px; color: {c.text_secondary};",
                            "Seuil VAD"
                        }
                        span {
                            style: "font-size: 11px; color: {c.text_primary}; font-family: monospace;",
                            "{vad_pct}%"
                        }
                    }
                    div {
                        style: "width: 100%; height: 4px; background: {c.bg_card}; border-radius: 2px; overflow: hidden;",
                        div {
                            style: "width: {vad_pct}%; height: 100%; background: {c.accent_orange}; border-radius: 2px;",
                        }
                    }
                }

                // Seuil wake word
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    div {
                        style: "display: flex; align-items: center; justify-content: space-between;",
                        label {
                            style: "font-size: 11px; color: {c.text_secondary};",
                            "Seuil wake word"
                        }
                        span {
                            style: "font-size: 11px; color: {c.text_primary}; font-family: monospace;",
                            "{wake_pct}%"
                        }
                    }
                    div {
                        style: "width: 100%; height: 4px; background: {c.bg_card}; border-radius: 2px; overflow: hidden;",
                        div {
                            style: "width: {wake_pct}%; height: 100%; background: {c.accent_blue}; border-radius: 2px;",
                        }
                    }
                }

                // Bouton Demarrer / Arreter
                {
                    let btn_bg = if is_active { c.accent_red } else { c.accent_green };
                    let btn_label = if is_active { "Arreter" } else { "Demarrer" };
                    let rid = room_id.clone();
                    rsx! {
                        button {
                            style: "padding: 8px 16px; background: {btn_bg}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer; font-size: 13px; font-weight: 500; align-self: flex-start;",
                            onclick: move |_| {
                                let svc_ref = voice_svc.read();
                                if let Some(svc) = svc_ref.as_ref() {
                                    if let Ok(mut guard) = svc.lock() {
                                        if is_active {
                                            guard.stop_room(&rid);
                                        } else {
                                            let dev = selected_device.read().clone();
                                            let dev_opt = if dev.is_empty() { None } else { Some(dev.as_str()) };
                                            if let Err(e) = guard.start_room(&rid, dev_opt) {
                                                tracing::error!("Erreur demarrage {}: {e}", rid);
                                            }
                                        }
                                    }
                                }
                            },
                            "{btn_label}"
                        }
                    }
                }
            }

            // ── Section Dispositifs (placeholder) ────────────────────────
            div {
                style: "padding: 12px; background: {c.bg_secondary}; border-radius: 6px; display: flex; flex-direction: column; gap: 10px;",
                div {
                    style: "display: flex; align-items: center; justify-content: space-between;",
                    p {
                        style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px; font-weight: 600;",
                        "Dispositifs de la piece"
                    }
                    button {
                        style: "padding: 4px 10px; background: {c.accent_blue}20; color: {c.accent_blue}; border: 1px solid {c.accent_blue}40; border-radius: 4px; cursor: pointer; font-size: 11px;",
                        "Ajouter"
                    }
                }
                div {
                    style: "padding: 16px; text-align: center; color: {c.text_muted}; font-size: 12px; border: 1px dashed {c.border}; border-radius: 4px;",
                    "Aucun dispositif associe a cette piece. "
                    "L'ajout de dispositifs sera disponible avec le crate miyualicia-devices."
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
