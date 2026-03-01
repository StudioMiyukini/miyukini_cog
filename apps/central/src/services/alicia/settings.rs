//! Ecran Parametres globaux Alicia Home Assistante.
//!
//! Sections : Audio (wake word, modele), MQTT (broker, client),
//! API (port, token, endpoints), Systeme (version, toolkit, infos COG).
//! Lit les donnees depuis le AliciaSnapshot partage.

use dioxus::prelude::*;
use crate::state::use_app_state;
use super::state::AliciaSnapshot;

// ─────────────────────────────────────────────────────────────────────────────
// Composant principal
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn SettingsScreen() -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
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
    let wk = &snap_ref.wake_word_config;

    let keyword = wk.keyword.clone();
    let model_path = wk.model_path.clone();
    let threshold_pct = format!("{:.0}%", wk.threshold * 100.0);
    let sample_rate_str = format!("{} Hz", wk.sample_rate);
    let score_mode = wk.score_mode.clone();
    let band_pass_label = if wk.band_pass_filter { "Actif" } else { "Inactif" };
    let gain_norm_label = if wk.gain_normalizer { "Actif" } else { "Inactif" };

    let device_count = snap_ref.available_devices.len();
    let device_count_str = format!("{device_count} device(s) detecte(s)");

    // MQTT placeholder values
    let mqtt_host = "localhost";
    let mqtt_port = "1883";
    let mqtt_client_id = "alicia-cog-001";
    let mqtt_status = "Non connecte";
    let mqtt_status_color = c.text_muted;

    // API placeholder values
    let api_port = "7890";
    let api_token_status = "Non configure";
    let api_endpoints = vec![
        ("GET", "/api/v1/rooms", "Liste des pieces"),
        ("GET", "/api/v1/devices", "Liste des dispositifs"),
        ("POST", "/api/v1/devices/:id/toggle", "Toggle dispositif"),
        ("GET", "/api/v1/automations", "Liste des automatisations"),
        ("GET", "/api/v1/status", "Statut global Alicia"),
    ];

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px; max-width: 700px;",

            // ── Section Audio ────────────────────────────────────────────
            SettingsSection {
                title: "Audio / Detection vocale",
                children: rsx! {
                    SettingRow {
                        label: "Mot-cle (wake word)",
                        value: keyword,
                        muted: None,
                    }
                    SettingRow {
                        label: "Chemin du modele",
                        value: model_path,
                        muted: Some("Fichier rustpotter .rpw"),
                    }
                    SettingRow {
                        label: "Seuil de detection",
                        value: threshold_pct,
                        muted: None,
                    }
                    SettingRow {
                        label: "Mode de score",
                        value: score_mode,
                        muted: None,
                    }
                    SettingRow {
                        label: "Taux d'echantillonnage",
                        value: sample_rate_str,
                        muted: None,
                    }
                    SettingRow {
                        label: "Canaux",
                        value: "1 (mono)".to_string(),
                        muted: None,
                    }
                    SettingRow {
                        label: "Filtre passe-bande",
                        value: band_pass_label.to_string(),
                        muted: Some("Filtrage frequentiel pre-detection"),
                    }
                    SettingRow {
                        label: "Normalisation gain",
                        value: gain_norm_label.to_string(),
                        muted: Some("Normalisation AGC pre-detection"),
                    }
                    SettingRow {
                        label: "Peripheriques",
                        value: device_count_str,
                        muted: None,
                    }
                }
            }

            // ── Section MQTT ─────────────────────────────────────────────
            div {
                style: "padding: 20px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border};",
                h3 {
                    style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 16px; text-transform: uppercase; letter-spacing: 0.5px;",
                    "MQTT"
                }
                SettingRow {
                    label: "Broker",
                    value: format!("{mqtt_host}:{mqtt_port}"),
                    muted: Some("Adresse du broker MQTT"),
                }
                SettingRow {
                    label: "Client ID",
                    value: mqtt_client_id.to_string(),
                    muted: None,
                }
                div {
                    style: "display: flex; align-items: baseline; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid {c.border};",
                    div {
                        style: "display: flex; flex-direction: column; gap: 2px;",
                        span {
                            style: "font-size: 13px; color: {c.text_primary};",
                            "Statut connexion"
                        }
                        span {
                            style: "font-size: 11px; color: {c.text_muted}; font-style: italic;",
                            "Connexion au broker MQTT pour la domotique"
                        }
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 6px; padding: 4px 10px; background: {mqtt_status_color}20; border-radius: 12px;",
                        div {
                            style: "width: 6px; height: 6px; border-radius: 50%; background: {mqtt_status_color};",
                        }
                        span {
                            style: "font-size: 12px; color: {mqtt_status_color}; font-weight: 500;",
                            "{mqtt_status}"
                        }
                    }
                }
                div {
                    style: "margin-top: 12px; padding: 8px 12px; background: {c.bg_secondary}; border-radius: 4px;",
                    p {
                        style: "font-size: 11px; color: {c.text_muted}; font-style: italic;",
                        "La connexion MQTT sera disponible avec le crate miyualicia-mqtt."
                    }
                }
            }

            // ── Section API ──────────────────────────────────────────────
            div {
                style: "padding: 20px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border};",
                h3 {
                    style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 16px; text-transform: uppercase; letter-spacing: 0.5px;",
                    "API REST"
                }
                SettingRow {
                    label: "Port",
                    value: api_port.to_string(),
                    muted: Some("Port d'ecoute de l'API Alicia"),
                }
                SettingRow {
                    label: "Token d'authentification",
                    value: api_token_status.to_string(),
                    muted: Some("Bearer token pour securiser l'API"),
                }

                // Endpoints disponibles
                div {
                    style: "margin-top: 12px;",
                    p {
                        style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 8px;",
                        "Endpoints disponibles"
                    }
                    div {
                        style: "background: {c.bg_secondary}; border-radius: 4px; overflow: hidden;",
                        for (i, (method, path, desc)) in api_endpoints.iter().enumerate() {
                            {
                                let border_top = if i > 0 { "1px solid" } else { "none" };
                                let border_val = c.border;
                                let method_color = if *method == "GET" { c.accent_green } else { c.accent_orange };
                                rsx! {
                                    div {
                                        style: "display: flex; align-items: center; gap: 12px; padding: 8px 12px; border-top: {border_top} {border_val};",
                                        span {
                                            style: "font-size: 11px; color: {method_color}; font-weight: 600; font-family: monospace; min-width: 40px;",
                                            "{method}"
                                        }
                                        span {
                                            style: "font-size: 12px; color: {c.text_primary}; font-family: monospace;",
                                            "{path}"
                                        }
                                        span {
                                            style: "font-size: 11px; color: {c.text_muted}; margin-left: auto;",
                                            "{desc}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── Section Systeme ──────────────────────────────────────────
            SettingsSection {
                title: "Systeme",
                children: rsx! {
                    SettingRow {
                        label: "Toolkit capture",
                        value: "MIYUALICIA-CAPTURE".to_string(),
                        muted: None,
                    }
                    SettingRow {
                        label: "Version capture",
                        value: env!("CARGO_PKG_VERSION").to_string(),
                        muted: None,
                    }
                    SettingRow {
                        label: "Toolkit wake word",
                        value: "MIYUALICIA-WAKEWORD".to_string(),
                        muted: None,
                    }
                    SettingRow {
                        label: "Version wake word",
                        value: env!("CARGO_PKG_VERSION").to_string(),
                        muted: None,
                    }
                    SettingRow {
                        label: "Toolkit dispositifs",
                        value: "MIYUALICIA-DEVICES (non integre)".to_string(),
                        muted: None,
                    }
                    SettingRow {
                        label: "Toolkit automatisations",
                        value: "MIYUALICIA-AUTOMATIONS (non integre)".to_string(),
                        muted: None,
                    }
                    SettingRow {
                        label: "Toolkit MQTT",
                        value: "MIYUALICIA-MQTT (non integre)".to_string(),
                        muted: None,
                    }
                }
            }

            // Note finale
            div {
                style: "padding: 12px 16px; background: {c.bg_secondary}; border-radius: 8px;",
                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "Alicia Home Assistante est un service interne COG. "
                    "Il utilise cpal pour la capture audio et rustpotter pour la detection de mot-cle. "
                    "Aucune donnee audio ne quitte le peripherique local. "
                    "La domotique (MQTT, dispositifs, automatisations) sera integree progressivement."
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Composants reutilisables
// ─────────────────────────────────────────────────────────────────────────────

/// Section de parametres avec titre et contenu.
#[derive(Props, Clone, PartialEq)]
struct SettingsSectionProps {
    title: &'static str,
    children: Element,
}

#[component]
fn SettingsSection(props: SettingsSectionProps) -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();

    rsx! {
        div {
            style: "padding: 20px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border};",
            h3 {
                style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 16px; text-transform: uppercase; letter-spacing: 0.5px;",
                "{props.title}"
            }
            {props.children}
        }
    }
}

/// Ligne de parametre : label + valeur + note optionnelle.
#[derive(Props, Clone, PartialEq)]
struct SettingRowProps {
    label: &'static str,
    value: String,
    #[props(default)]
    muted: Option<&'static str>,
}

#[component]
fn SettingRow(props: SettingRowProps) -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; align-items: baseline; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid {c.border};",
            div {
                style: "display: flex; flex-direction: column; gap: 2px;",
                span {
                    style: "font-size: 13px; color: {c.text_primary};",
                    "{props.label}"
                }
                if let Some(hint) = props.muted {
                    span {
                        style: "font-size: 11px; color: {c.text_muted}; font-style: italic;",
                        "{hint}"
                    }
                }
            }
            span {
                style: "font-size: 13px; color: {c.text_white}; font-family: monospace; background: {c.bg_secondary}; padding: 4px 10px; border-radius: 4px;",
                "{props.value}"
            }
        }
    }
}
