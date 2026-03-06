//! Vue calendrier CalDAV dans Central.
//!
//! @id: miyucloud_calendar_view
//! @do: display_caldav_calendars_events
//! @role: component
//! @layer: presentation
//!
//! Affiche la liste des calendriers (sidebar gauche) et une grille
//! mensuelle avec les evenements du calendrier selectionne.

use dioxus::prelude::*;

use super::client::MiyuCloudClient;
use super::state::{CalendarEvent, MiyuCloudState};
use crate::state::use_app_state;

/// Vue principale calendrier CalDAV.
#[component]
pub fn CalendarView(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let calendars = state.read().calendars.clone();
    let selected_cal = state.read().selected_calendar.clone();
    let events = state.read().calendar_events.clone();
    let month = state.read().calendar_month;
    let year = state.read().calendar_year;
    let error_msg = state.read().error_message.clone();

    // Charger les calendriers au montage
    let mut loaded = use_signal(|| false);
    use_effect(move || {
        if *loaded.read() {
            return;
        }
        loaded.set(true);

        spawn(async move {
            let http = { client.read().clone() };
            let Some(http) = http else { return };

            match http.list_calendars().await {
                Ok(cals) => {
                    let first_name = cals.first().map(|c| c.name.clone());
                    state.write().calendars = cals;
                    if state.read().selected_calendar.is_none() {
                        if let Some(name) = first_name {
                            state.write().selected_calendar = Some(name);
                        }
                    }
                }
                Err(e) => {
                    state.write().error_message =
                        Some(format!("Echec chargement calendriers : {e}"));
                }
            }
        });
    });

    // Charger les evenements quand le calendrier selectionne change
    let selected_cal_memo = use_memo(move || state.read().selected_calendar.clone());
    use_effect(move || {
        let cal_name = selected_cal_memo.read().clone();
        let Some(cal_name) = cal_name else { return };

        spawn(async move {
            let http = { client.read().clone() };
            let Some(http) = http else { return };

            match http.list_events(&cal_name).await {
                Ok(evts) => {
                    state.write().calendar_events = evts;
                }
                Err(e) => {
                    state.write().error_message =
                        Some(format!("Echec chargement evenements : {e}"));
                }
            }
        });
    });

    let month_name = month_label(month);

    rsx! {
        div {
            style: "flex: 1; display: flex; overflow: hidden;",

            // Sidebar calendriers
            div {
                style: "width: 220px; min-width: 220px; border-right: 1px solid {c.border}; display: flex; flex-direction: column; overflow-y: auto; background: {c.bg_secondary};",

                // Header
                div {
                    style: "padding: 12px 16px; border-bottom: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between;",
                    span { style: "font-size: 14px; color: {c.text_white}; font-weight: 600;", "Calendriers" }
                    button {
                        style: "padding: 2px 8px; background: {c.accent_blue}20; color: {c.accent_blue}; border: 1px solid {c.accent_blue}40; border-radius: 4px; cursor: pointer; font-size: 12px;",
                        onclick: move |_| {
                            spawn(async move {
                                let http = { client.read().clone() };
                                let Some(http) = http else { return };
                                let name = format!("cal-{}", rand_id());
                                match http.create_calendar(&name, "Nouveau calendrier", Some("#4A90D9")).await {
                                    Ok(cal) => {
                                        let mut s = state.write();
                                        s.calendars.push(cal);
                                        s.selected_calendar = Some(name);
                                    }
                                    Err(e) => {
                                        state.write().error_message = Some(format!("Echec creation : {e}"));
                                    }
                                }
                            });
                        },
                        "+"
                    }
                }

                // Liste des calendriers
                for cal in calendars.iter() {
                    {
                        let cal_name = cal.name.clone();
                        let display = cal.display_name.clone();
                        let color = cal.color.clone().unwrap_or_else(|| "#4A90D9".to_string());
                        let is_selected = selected_cal.as_deref() == Some(cal_name.as_str());
                        let bg = if is_selected { c.bg_hover } else { "transparent" };
                        let text_col = if is_selected { c.text_white } else { c.text_secondary };

                        rsx! {
                            button {
                                style: "display: flex; align-items: center; gap: 8px; padding: 8px 16px; background: {bg}; color: {text_col}; border: none; cursor: pointer; font-size: 13px; text-align: left; width: 100%;",
                                onclick: move |_| {
                                    state.write().selected_calendar = Some(cal_name.clone());
                                    state.write().selected_event_uid = None;
                                },
                                span {
                                    style: "width: 10px; height: 10px; border-radius: 50%; background: {color}; flex-shrink: 0;",
                                }
                                span { style: "overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{display}" }
                            }
                        }
                    }
                }
            }

            // Zone principale : grille mensuelle
            div {
                style: "flex: 1; display: flex; flex-direction: column; overflow: hidden;",

                // Erreur
                if let Some(ref err) = error_msg {
                    div {
                        style: "padding: 8px 16px; background: {c.accent_red}20; border-left: 3px solid {c.accent_red}; margin: 8px 16px 0; border-radius: 4px; color: {c.accent_red}; font-size: 13px;",
                        "{err}"
                    }
                }

                // Navigation mois
                div {
                    style: "display: flex; align-items: center; justify-content: center; gap: 16px; padding: 12px 16px; border-bottom: 1px solid {c.border};",
                    button {
                        style: "padding: 4px 12px; background: {c.bg_hover}; color: {c.text_secondary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 14px;",
                        onclick: move |_| {
                            let mut s = state.write();
                            if s.calendar_month == 1 {
                                s.calendar_month = 12;
                                s.calendar_year -= 1;
                            } else {
                                s.calendar_month -= 1;
                            }
                        },
                        "\u{25C0}"
                    }
                    span {
                        style: "font-size: 16px; font-weight: 600; color: {c.text_white}; min-width: 160px; text-align: center;",
                        "{month_name} {year}"
                    }
                    button {
                        style: "padding: 4px 12px; background: {c.bg_hover}; color: {c.text_secondary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 14px;",
                        onclick: move |_| {
                            let mut s = state.write();
                            if s.calendar_month == 12 {
                                s.calendar_month = 1;
                                s.calendar_year += 1;
                            } else {
                                s.calendar_month += 1;
                            }
                        },
                        "\u{25B6}"
                    }
                }

                // Jours de la semaine
                div {
                    style: "display: grid; grid-template-columns: repeat(7, 1fr); border-bottom: 1px solid {c.border};",
                    for day_name in ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"] {
                        div {
                            style: "padding: 6px; text-align: center; font-size: 11px; color: {c.text_muted}; font-weight: 600;",
                            "{day_name}"
                        }
                    }
                }

                // Grille du mois
                div {
                    style: "flex: 1; display: grid; grid-template-columns: repeat(7, 1fr); grid-auto-rows: minmax(80px, 1fr); overflow-y: auto;",
                    {
                        let cells = month_cells(year, month);
                        rsx! {
                            for (day_num, is_current_month) in cells.iter() {
                                {
                                    let day = *day_num;
                                    let in_month = *is_current_month;
                                    let day_events = if in_month {
                                        events_for_day(&events, year, month, day)
                                    } else {
                                        Vec::new()
                                    };
                                    let text_color = if in_month { c.text_primary } else { c.text_muted };
                                    let bg_cell = if in_month { "transparent".to_string() } else { format!("{}08", c.bg_hover) };

                                    rsx! {
                                        div {
                                            style: "border: 1px solid {c.border}; padding: 4px; background: {bg_cell}; overflow: hidden;",
                                            div {
                                                style: "font-size: 12px; color: {text_color}; margin-bottom: 2px;",
                                                "{day}"
                                            }
                                            for evt in day_events.iter() {
                                                {
                                                    let summary = evt.summary.clone();
                                                    let uid = evt.uid.clone();
                                                    rsx! {
                                                        div {
                                                            style: "font-size: 10px; padding: 1px 4px; background: {c.accent_blue}30; color: {c.accent_blue}; border-radius: 2px; margin-bottom: 1px; cursor: pointer; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                                            onclick: move |_| {
                                                                state.write().selected_event_uid = Some(uid.clone());
                                                            },
                                                            "{summary}"
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
        }
    }
}

/// Genere un petit ID aleatoire pour les noms de calendrier.
fn rand_id() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("{:x}", now & 0xFFFF_FFFF)
}

/// Nom du mois en francais.
fn month_label(month: u32) -> &'static str {
    match month {
        1 => "Janvier",
        2 => "Fevrier",
        3 => "Mars",
        4 => "Avril",
        5 => "Mai",
        6 => "Juin",
        7 => "Juillet",
        8 => "Aout",
        9 => "Septembre",
        10 => "Octobre",
        11 => "Novembre",
        12 => "Decembre",
        _ => "?",
    }
}

/// Genere les cellules du calendrier mensuel : (jour, est_dans_le_mois).
fn month_cells(year: i32, month: u32) -> Vec<(u32, bool)> {
    let total_days = days_in_month(year, month);
    // Jour de la semaine du 1er (0=Lun, 6=Dim) via Sakamoto
    let first_dow = day_of_week(year, month, 1);

    let mut cells = Vec::new();

    // Jours du mois precedent
    if first_dow > 0 {
        let prev_month = if month == 1 { 12 } else { month - 1 };
        let prev_year = if month == 1 { year - 1 } else { year };
        let prev_days = days_in_month(prev_year, prev_month);
        for d in (prev_days - first_dow + 1)..=prev_days {
            cells.push((d, false));
        }
    }

    // Jours du mois courant
    for d in 1..=total_days {
        cells.push((d, true));
    }

    // Jours du mois suivant (remplir jusqu'a 42 cellules = 6 semaines)
    let remaining = 42 - cells.len();
    for d in 1..=(remaining as u32) {
        cells.push((d, false));
    }

    cells
}

/// Nombre de jours dans un mois.
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Jour de la semaine (0=Lun, 6=Dim) via algorithme de Tomohiko Sakamoto.
fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
    let t = [0i32, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if month < 3 { year - 1 } else { year };
    let dow = (y + y / 4 - y / 100 + y / 400 + t[(month - 1) as usize] + day as i32) % 7;
    // Convertir: 0=Dim -> 6, 1=Lun -> 0, etc.
    ((dow + 6) % 7) as u32
}

/// Filtre les evenements pour un jour donne.
fn events_for_day(events: &[CalendarEvent], year: i32, month: u32, day: u32) -> Vec<CalendarEvent> {
    let target = format!("{year:04}-{month:02}-{day:02}");
    events
        .iter()
        .filter(|e| e.dtstart.starts_with(&target))
        .cloned()
        .collect()
}
