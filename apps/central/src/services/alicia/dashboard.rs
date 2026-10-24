//! Ecran Tableau de bord Alicia Home Assistante.
//!
//! 7 zones :
//! - Maison (resume global)
//! - Audio (cards pieces VAD/RMS)
//! - Dernieres activites (journal unifie)
//! - Meteo (auj + 3 jours, types canoniques jaymeteo-client)
//! - Agenda (evenements jour + semaine, types canoniques jaykoa-client)
//! - JayKonta (solde + 5 derniers mouvements, types canoniques jaykonta-client)
//! - Contacts (liste + ajout, types canoniques jaycontact-client)
//!
//! Lit les donnees Alicia depuis le AliciaSnapshot partage. Les zones 4-7
//! utilisent les types DTOs des `*-client` crates ; les valeurs viennent pour
//! l'instant de seeders synchrones (`sample_*`) — le branchement async
//! (use_resource + FakeBackend, puis vrais backends Open-Meteo / miyucontacts /
//! KindMother) est l'etape suivante.

use dioxus::prelude::*;
use miyualicia_capture::VadState;

use jaycontact_client::{ContactKind, ContactSummary};
use jaykoa_client::EventSummary;
use jaykonta_client::{BalanceSummary, MovementKind, MovementSummary};
use jaymeteo_client::{WeatherCondition, WeatherForecastDay, WeatherNow};

use super::state::{AliciaSnapshot, RoomLiveState};
use crate::state::use_app_state;

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

            // ── Zone 4 : Meteo ───────────────────────────────────────────
            SectionHeader { title: "Meteo" }
            WeatherZone {}

            // ── Zone 5 : Agenda ──────────────────────────────────────────
            SectionHeader { title: "Agenda" }
            AgendaZone {}

            // ── Zone 6 : JayKonta ────────────────────────────────────────
            SectionHeader { title: "JayKonta — comptabilite" }
            JayKontaZone {}

            // ── Zone 7 : Contacts ────────────────────────────────────────
            SectionHeader { title: "Contacts" }
            ContactsZone {}

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

    let mic_color = if room.mic_active {
        c.accent_green
    } else {
        c.text_muted
    };
    let mic_label = if room.mic_active {
        "Micro actif"
    } else {
        "Micro inactif"
    };

    let vad_active = matches!(room.vad_state, VadState::Speech | VadState::MaybeEnd);
    let vad_color = if vad_active {
        c.accent_orange
    } else {
        c.text_muted
    };
    let vad_label = if vad_active {
        "Voix detectee"
    } else {
        "Silence"
    };

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

// ─────────────────────────────────────────────────────────────────────────────
// Zones productivite (4-7) — types canoniques + sample seeders
// ─────────────────────────────────────────────────────────────────────────────
//
// Les `sample_*` synchrones renvoient les DTOs canoniques des `*-client`
// crates. Ils seront remplaces par `use_resource` + appel `ServiceClient.call()`
// (Fake puis vrai backend) au prochain incrément.

/// Etiquette FR d'une condition meteo (la methode `label_fr` est sur le type
/// `WeatherCondition`, mais on l'expose aussi en local pour clarete).
fn weather_label(c: WeatherCondition) -> &'static str {
    c.label_fr()
}

/// Pictogramme ASCII pour une condition meteo (specifique au dashboard).
fn weather_icon(c: WeatherCondition) -> &'static str {
    match c {
        WeatherCondition::Clear => "*",
        WeatherCondition::Cloudy => "~",
        WeatherCondition::Rain => "//",
        WeatherCondition::Snow => "**",
        WeatherCondition::Storm => "><",
        WeatherCondition::Fog => "..",
    }
}

/// Etiquette FR d'un type de contact pour le badge.
fn contact_kind_label(k: ContactKind) -> &'static str {
    match k {
        ContactKind::Friend => "ami",
        ContactKind::Foe => "bloque",
    }
}

/// `YYYY-MM-DD` → libellé relatif court ("Auj", "Demain", "J+3", etc.).
fn relative_date_label(iso_date: &str) -> String {
    match chrono::NaiveDate::parse_from_str(iso_date, "%Y-%m-%d") {
        Ok(date) => {
            let today = chrono::Local::now().date_naive();
            let diff = (date - today).num_days();
            match diff {
                0 => "Auj".to_string(),
                1 => "Demain".to_string(),
                2 => "Apres-demain".to_string(),
                d if d > 0 => format!("J+{d}"),
                d => format!("J{d}"),
            }
        }
        Err(_) => iso_date.to_string(),
    }
}

/// RFC 3339 → `HH:MM` local. Si l'événement est `all_day` ou la date n'est
/// pas aujourd'hui, prefixe avec une étiquette de jour relative.
fn event_when_label(rfc3339_start: &str, all_day: bool) -> String {
    let parsed = chrono::DateTime::parse_from_rfc3339(rfc3339_start).ok();
    match parsed {
        Some(dt) => {
            let today = chrono::Local::now().date_naive();
            let date = dt.date_naive();
            let diff = (date - today).num_days();
            let day_prefix = match diff {
                0 => None,
                1 => Some("Demain ".to_string()),
                2 => Some("Apres-demain ".to_string()),
                d if d > 0 && d <= 6 => Some(format!("J+{d} ")),
                d if d > 0 => Some(format!("{} ", date.format("%d/%m"))),
                _ => Some(format!("{} ", date.format("%d/%m"))),
            };
            if all_day {
                day_prefix.unwrap_or_else(|| "Toute la journee".to_string())
            } else {
                let hm = dt.format("%H:%M").to_string();
                match day_prefix {
                    Some(prefix) => format!("{prefix}{hm}"),
                    None => hm,
                }
            }
        }
        None => rfc3339_start.to_string(),
    }
}

/// RFC 3339 → date courte `JJ/MM` (utilisé pour les mouvements compta).
fn movement_date_short(rfc3339: &str) -> String {
    chrono::DateTime::parse_from_rfc3339(rfc3339)
        .ok()
        .map(|dt| {
            let today = chrono::Local::now().date_naive();
            let date = dt.date_naive();
            let diff = (today - date).num_days();
            match diff {
                0 => "Auj".to_string(),
                1 => "Hier".to_string(),
                _ => dt.format("%d/%m").to_string(),
            }
        })
        .unwrap_or_else(|| rfc3339.to_string())
}

/// Indique si l'événement est aujourd'hui (locale).
fn event_is_today(rfc3339_start: &str) -> bool {
    chrono::DateTime::parse_from_rfc3339(rfc3339_start)
        .ok()
        .map_or(false, |dt| {
            dt.date_naive() == chrono::Local::now().date_naive()
        })
}

// ─────────────────────────────────────────────────────────────────────────────
// Sample seeders (synchrones, retournent les DTOs canoniques)
// ─────────────────────────────────────────────────────────────────────────────

fn sample_weather_now() -> WeatherNow {
    WeatherNow {
        city: "Paris".into(),
        temp_c: 18.5,
        condition: WeatherCondition::Cloudy,
        humidity_pct: 62,
        wind_kmh: 14,
    }
}

fn sample_forecast() -> Vec<WeatherForecastDay> {
    let today = chrono::Local::now().date_naive();
    vec![
        WeatherForecastDay {
            date_iso: (today + chrono::Duration::days(1)).format("%Y-%m-%d").to_string(),
            max_c: 21.0,
            min_c: 12.0,
            condition: WeatherCondition::Clear,
        },
        WeatherForecastDay {
            date_iso: (today + chrono::Duration::days(2)).format("%Y-%m-%d").to_string(),
            max_c: 19.0,
            min_c: 11.0,
            condition: WeatherCondition::Cloudy,
        },
        WeatherForecastDay {
            date_iso: (today + chrono::Duration::days(3)).format("%Y-%m-%d").to_string(),
            max_c: 16.0,
            min_c: 9.0,
            condition: WeatherCondition::Rain,
        },
    ]
}

fn ev(id: &str, title: &str, start_rfc3339: &str, location: Option<&str>) -> EventSummary {
    EventSummary {
        id: id.into(),
        title: title.into(),
        start: start_rfc3339.into(),
        end: None,
        location: location.map(ToString::to_string),
        all_day: false,
        source_service: None,
    }
}

fn sample_events_today() -> Vec<EventSummary> {
    let today = chrono::Local::now().date_naive();
    let today_str = today.format("%Y-%m-%d").to_string();
    vec![
        ev(
            "ev_origin",
            "Reunion equipe Origin",
            &format!("{today_str}T14:30:00+02:00"),
            Some("Salle 2"),
        ),
        ev(
            "ev_festosh",
            "Appel client Festosh",
            &format!("{today_str}T16:00:00+02:00"),
            None,
        ),
    ]
}

fn sample_events_week() -> Vec<EventSummary> {
    let today = chrono::Local::now().date_naive();
    vec![
        ev(
            "ev_dej_marc",
            "Dej Marc",
            &format!(
                "{}T10:00:00+02:00",
                (today + chrono::Duration::days(1)).format("%Y-%m-%d")
            ),
            Some("Cafe Central"),
        ),
        ev(
            "ev_sprint",
            "Sprint review",
            &format!(
                "{}T09:30:00+02:00",
                (today + chrono::Duration::days(2)).format("%Y-%m-%d")
            ),
            None,
        ),
        ev(
            "ev_demo",
            "Demo Alicia",
            &format!(
                "{}T14:00:00+02:00",
                (today + chrono::Duration::days(3)).format("%Y-%m-%d")
            ),
            Some("Visio"),
        ),
        ev(
            "ev_apero",
            "Apero equipe",
            &format!(
                "{}T18:00:00+02:00",
                (today + chrono::Duration::days(4)).format("%Y-%m-%d")
            ),
            Some("Bar du Coin"),
        ),
    ]
}

fn sample_balance() -> BalanceSummary {
    BalanceSummary {
        purse_id: "default".into(),
        balance: 1284.50,
        currency: "EUR".into(),
    }
}

fn mv(
    id: &str,
    label: &str,
    signed_amount: f64,
    category: Option<&str>,
    days_ago: i64,
) -> MovementSummary {
    let kind = if signed_amount >= 0.0 {
        MovementKind::Income
    } else {
        MovementKind::Expense
    };
    let when = chrono::Local::now() - chrono::Duration::days(days_ago);
    MovementSummary {
        id: id.into(),
        purse_id: "default".into(),
        kind,
        signed_amount,
        label: Some(label.into()),
        category: category.map(ToString::to_string),
        created_at: when.to_rfc3339(),
    }
}

fn sample_movements() -> Vec<MovementSummary> {
    vec![
        mv("mv_5", "Courses Carrefour", -42.30, Some("Alimentation"), 0),
        mv("mv_4", "Salaire mai", 2400.00, Some("Revenus"), 1),
        mv("mv_3", "Essence", -65.00, Some("Transport"), 2),
        mv("mv_2", "Loyer", -780.00, Some("Logement"), 3),
        mv("mv_1", "Abonnement IA", -20.00, Some("Tech"), 4),
    ]
}

fn ct(id: &str, name: &str) -> ContactSummary {
    ContactSummary {
        id: id.into(),
        display_name: name.into(),
        kind: ContactKind::Friend,
    }
}

fn sample_contacts() -> Vec<ContactSummary> {
    vec![
        ct("user_marc", "Marc Dupont"),
        ct("user_sophie", "Sophie Lemoine"),
        ct("user_alex", "Alex Martin"),
        ct("user_julien", "Julien Roux"),
    ]
}

// ─────────────────────────────────────────────────────────────────────────────
// Zone 4 — Meteo
// ─────────────────────────────────────────────────────────────────────────────

#[component]
fn WeatherZone() -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();

    let now = use_signal(sample_weather_now);
    let forecast = use_signal(sample_forecast);

    let now_ref = now.read();
    let now_city = now_ref.city.clone();
    let now_temp = format!("{:.1} °C", now_ref.temp_c);
    let now_condition_label = weather_label(now_ref.condition);
    let now_condition_icon = weather_icon(now_ref.condition);
    let now_humidity = now_ref.humidity_pct;
    let now_wind = now_ref.wind_kmh;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 12px;",

            // Carte principale : meteo actuelle
            div {
                style: "padding: 20px 24px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; display: flex; align-items: center; gap: 24px; flex-wrap: wrap;",
                div {
                    style: "width: 64px; height: 64px; border-radius: 50%; background: {c.bg_secondary}; display: flex; align-items: center; justify-content: center; font-size: 28px; color: {c.accent_orange}; flex-shrink: 0;",
                    "{now_condition_icon}"
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 4px; flex: 1; min-width: 200px;",
                    p {
                        style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                        "{now_city}"
                    }
                    p {
                        style: "font-size: 28px; font-weight: 600; color: {c.text_white};",
                        "{now_temp}"
                    }
                    p {
                        style: "font-size: 13px; color: {c.text_primary};",
                        "{now_condition_label}"
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 6px; min-width: 140px;",
                    p {
                        style: "font-size: 12px; color: {c.text_muted};",
                        "Humidite : "
                        span { style: "color: {c.text_primary};", "{now_humidity}%" }
                    }
                    p {
                        style: "font-size: 12px; color: {c.text_muted};",
                        "Vent : "
                        span { style: "color: {c.text_primary};", "{now_wind} km/h" }
                    }
                }
            }

            // Previsions 3 jours
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(160px, 1fr)); gap: 12px;",
                for day in forecast.read().iter() {
                    WeatherForecastDayCard { day: day.clone() }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct WeatherForecastDayCardProps {
    day: WeatherForecastDay,
}

#[component]
fn WeatherForecastDayCard(props: WeatherForecastDayCardProps) -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();
    let day = &props.day;

    let date_label = relative_date_label(&day.date_iso);
    let icon = weather_icon(day.condition);
    let label = weather_label(day.condition);
    let max_str = format!("{:.0}°", day.max_c);
    let min_str = format!("{:.0}°", day.min_c);

    rsx! {
        div {
            style: "padding: 14px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; display: flex; flex-direction: column; align-items: center; gap: 6px;",
            p {
                style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                "{date_label}"
            }
            span {
                style: "font-size: 22px; color: {c.accent_blue};",
                "{icon}"
            }
            p {
                style: "font-size: 11px; color: {c.text_muted};",
                "{label}"
            }
            div {
                style: "display: flex; gap: 8px; align-items: baseline;",
                span { style: "font-size: 16px; font-weight: 600; color: {c.text_white};", "{max_str}" }
                span { style: "font-size: 12px; color: {c.text_muted};", "{min_str}" }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Zone 5 — Agenda
// ─────────────────────────────────────────────────────────────────────────────

#[component]
fn AgendaZone() -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();

    let today = use_signal(sample_events_today);
    let week = use_signal(sample_events_week);

    let today_events = today.read().clone();
    let week_events = week.read().clone();

    let next_label = today_events.first().map(|e| {
        let when = event_when_label(&e.start, e.all_day);
        format!("Prochain : {} a {when}", e.title)
    });

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px;",

            // Bandeau "prochain RDV"
            if let Some(label) = next_label {
                div {
                    style: "padding: 12px 16px; background: {c.bg_secondary}; border-left: 3px solid {c.accent_blue}; border-radius: 4px;",
                    p {
                        style: "font-size: 13px; color: {c.text_primary};",
                        "{label}"
                    }
                }
            }

            // Aujourd'hui
            div {
                style: "background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; padding: 16px; display: flex; flex-direction: column; gap: 8px;",
                p {
                    style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 4px;",
                    "Aujourd'hui"
                }
                if today_events.is_empty() {
                    p {
                        style: "font-size: 12px; color: {c.text_muted}; font-style: italic;",
                        "Aucun evenement prevu aujourd'hui."
                    }
                } else {
                    for ev in today_events.iter() {
                        AgendaEventRow { event: ev.clone() }
                    }
                }
            }

            // Cette semaine
            div {
                style: "background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; padding: 16px; display: flex; flex-direction: column; gap: 8px;",
                p {
                    style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 4px;",
                    "Cette semaine"
                }
                if week_events.is_empty() {
                    p {
                        style: "font-size: 12px; color: {c.text_muted}; font-style: italic;",
                        "Aucun evenement a venir."
                    }
                } else {
                    for ev in week_events.iter() {
                        AgendaEventRow { event: ev.clone() }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct AgendaEventRowProps {
    event: EventSummary,
}

#[component]
fn AgendaEventRow(props: AgendaEventRowProps) -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();
    let ev = &props.event;

    let is_today = event_is_today(&ev.start);
    let time_color = if is_today {
        c.accent_blue
    } else {
        c.text_secondary
    };
    let when = event_when_label(&ev.start, ev.all_day);
    let title = ev.title.clone();
    let location = ev.location.clone();

    rsx! {
        div {
            style: "display: flex; align-items: baseline; gap: 12px; padding: 6px 0;",
            span {
                style: "font-size: 12px; color: {time_color}; min-width: 110px; font-family: monospace;",
                "{when}"
            }
            div {
                style: "display: flex; flex-direction: column; gap: 2px; flex: 1;",
                span {
                    style: "font-size: 13px; color: {c.text_white};",
                    "{title}"
                }
                if let Some(loc) = location {
                    span {
                        style: "font-size: 11px; color: {c.text_muted};",
                        "@ {loc}"
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Zone 6 — JayKonta (compta)
// ─────────────────────────────────────────────────────────────────────────────

#[component]
fn JayKontaZone() -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();

    let balance = use_signal(sample_balance);
    let movements = use_signal(sample_movements);

    let balance_ref = balance.read();
    let balance_val = balance_ref.balance;
    let currency = balance_ref.currency.clone();
    let balance_color = if balance_val >= 0.0 {
        c.accent_green
    } else {
        c.accent_red
    };
    let balance_str = format!("{balance_val:+.2} {currency}");

    let movements_data = movements.read().clone();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 12px;",

            // Solde + bouton ajouter (en tete)
            div {
                style: "background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; padding: 16px 20px; display: flex; align-items: center; justify-content: space-between; gap: 16px; flex-wrap: wrap;",
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    p {
                        style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Solde courant"
                    }
                    p {
                        style: "font-size: 24px; font-weight: 600; color: {balance_color};",
                        "{balance_str}"
                    }
                }
                button {
                    r#type: "button",
                    style: "padding: 10px 18px; background: {c.accent_blue}; color: {c.text_white}; border: none; border-radius: 6px; font-size: 13px; font-weight: 600; cursor: pointer;",
                    "+ Ajouter un mouvement"
                }
            }

            // Liste mouvements
            div {
                style: "background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; overflow: hidden;",
                p {
                    style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px; padding: 12px 16px; border-bottom: 1px solid {c.border};",
                    "5 derniers mouvements"
                }
                for (i, m) in movements_data.iter().enumerate() {
                    MovementRow { idx: i, movement: m.clone() }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MovementRowProps {
    idx: usize,
    movement: MovementSummary,
}

#[component]
fn MovementRow(props: MovementRowProps) -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();
    let m = &props.movement;

    let amount_color = if m.signed_amount >= 0.0 {
        c.accent_green
    } else {
        c.accent_red
    };
    let amount_str = format!("{:+.2} €", m.signed_amount);
    let date = movement_date_short(&m.created_at);
    let label = m.label.clone().unwrap_or_else(|| match m.kind {
        MovementKind::Expense => "depense".to_string(),
        MovementKind::Income => "revenu".to_string(),
    });
    let category = m.category.clone();
    let border_top = if props.idx > 0 { "1px solid" } else { "none" };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 10px 16px; border-top: {border_top} {c.border};",
            span {
                style: "font-size: 12px; color: {c.text_muted}; min-width: 64px; font-family: monospace;",
                "{date}"
            }
            div {
                style: "display: flex; flex-direction: column; gap: 2px; flex: 1; min-width: 0;",
                span {
                    style: "font-size: 13px; color: {c.text_white};",
                    "{label}"
                }
                if let Some(cat) = category {
                    span {
                        style: "font-size: 11px; color: {c.text_secondary};",
                        "{cat}"
                    }
                }
            }
            span {
                style: "font-size: 14px; font-weight: 600; color: {amount_color}; font-family: monospace;",
                "{amount_str}"
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Zone 7 — Contacts
// ─────────────────────────────────────────────────────────────────────────────

#[component]
fn ContactsZone() -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();

    let contacts = use_signal(sample_contacts);
    let mut new_contact = use_signal(String::new);

    let contacts_data = contacts.read().clone();
    let count = contacts_data.len();
    let count_label = format!("{count} contact{}", if count > 1 { "s" } else { "" });

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 12px;",

            // Bandeau ajout
            div {
                style: "background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; padding: 12px 16px; display: flex; gap: 8px; align-items: center; flex-wrap: wrap;",
                input {
                    r#type: "text",
                    placeholder: "Nom ou identifiant a ajouter",
                    value: "{new_contact.read()}",
                    oninput: move |e| new_contact.set(e.value()),
                    style: "flex: 1; min-width: 200px; padding: 8px 12px; background: {c.bg_secondary}; color: {c.text_white}; border: 1px solid {c.border}; border-radius: 6px; font-size: 13px;",
                }
                button {
                    r#type: "button",
                    style: "padding: 8px 16px; background: {c.accent_green}; color: {c.text_white}; border: none; border-radius: 6px; font-size: 13px; font-weight: 600; cursor: pointer;",
                    "+ Ajouter"
                }
            }

            // Liste
            div {
                style: "background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; overflow: hidden;",
                p {
                    style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px; padding: 12px 16px; border-bottom: 1px solid {c.border};",
                    "{count_label}"
                }
                if contacts_data.is_empty() {
                    p {
                        style: "padding: 16px; font-size: 12px; color: {c.text_muted}; font-style: italic; text-align: center;",
                        "Aucun contact pour le moment."
                    }
                } else {
                    for (i, contact) in contacts_data.iter().enumerate() {
                        ContactRow { idx: i, contact: contact.clone() }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ContactRowProps {
    idx: usize,
    contact: ContactSummary,
}

#[component]
fn ContactRow(props: ContactRowProps) -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();
    let ct = &props.contact;

    let initial = ct
        .display_name
        .chars()
        .next()
        .map(|ch| ch.to_string().to_uppercase())
        .unwrap_or_default();
    let name = ct.display_name.clone();
    let kind = contact_kind_label(ct.kind);
    let id_label = ct.id.clone();
    let border_top = if props.idx > 0 { "1px solid" } else { "none" };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 10px 16px; border-top: {border_top} {c.border};",
            div {
                style: "width: 32px; height: 32px; border-radius: 50%; background: {c.bg_secondary}; color: {c.accent_blue}; display: flex; align-items: center; justify-content: center; font-size: 13px; font-weight: 600; flex-shrink: 0;",
                "{initial}"
            }
            div {
                style: "display: flex; flex-direction: column; gap: 2px; flex: 1; min-width: 0;",
                span {
                    style: "font-size: 13px; color: {c.text_white};",
                    "{name}"
                }
                span {
                    style: "font-size: 11px; color: {c.text_muted};",
                    "{id_label}"
                }
            }
            span {
                style: "font-size: 11px; color: {c.text_secondary}; padding: 3px 8px; background: {c.bg_secondary}; border-radius: 999px; text-transform: uppercase; letter-spacing: 0.4px;",
                "{kind}"
            }
        }
    }
}
