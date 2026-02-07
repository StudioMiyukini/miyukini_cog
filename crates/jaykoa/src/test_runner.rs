//! Runner des tests unitaires JayKoa — exécutable depuis MiyukiniAdmin.
//!
//! @id: jaykoa_test_runner
//! @do: run_all_unit_tests_and_return_report
//! @layer: infra
//! @human: Exécute les mêmes scénarios que les tests #[cfg(test)] et retourne un rapport
//!         sérialisable (JSON) pour affichage dans la console MiyukiniAdmin.

use crate::data::types::{
    Agenda, CalendarType, EntryType, EventSource, TemporalEntry, TemporalStatus, UserSettings,
};
use crate::data::JayKoaDb;
use crate::export::ical;
use kindmother::InstanceType;
use serde::Serialize;

/// Résultat d’un test individuel.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct JayKoaTestItem {
    /// Nom du test.
    pub name: String,
    /// True si le test a réussi.
    pub passed: bool,
    /// Message d'erreur si échec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Rapport global des tests unitaires JayKoa.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct JayKoaTestReport {
    /// Nombre de tests passés.
    pub passed: u32,
    /// Nombre de tests en échec.
    pub failed: u32,
    /// Nombre total de tests.
    pub total: u32,
    /// Statut global : ok ou partial.
    pub status: String,
    /// Détail par test.
    pub results: Vec<JayKoaTestItem>,
}

/// Exécute tous les tests unitaires JayKoa et retourne un rapport.
pub fn run_all() -> JayKoaTestReport {
    let mut results = Vec::new();

    // --- Types (domain) ---
    run("entry_type_roundtrip", run_entry_type_roundtrip, &mut results);
    run("entry_type_readonly", run_entry_type_readonly, &mut results);
    run("temporal_status_roundtrip", run_temporal_status_roundtrip, &mut results);
    run("event_source_roundtrip", run_event_source_roundtrip, &mut results);
    run("calendar_type_roundtrip", run_calendar_type_roundtrip, &mut results);
    run("default_user_settings", run_default_user_settings, &mut results);

    // --- DB ---
    run("db_open_and_init", run_db_open_and_init, &mut results);
    run("db_agenda_crud", run_db_agenda_crud, &mut results);
    run("db_entry_crud", run_db_entry_crud, &mut results);
    run("db_reflect_upsert", run_db_reflect_upsert, &mut results);
    run("db_detect_conflicts", run_db_detect_conflicts, &mut results);
    run("db_detect_no_conflict", run_db_detect_no_conflict, &mut results);
    run("db_check_conflict_api", run_db_check_conflict_api, &mut results);
    run("db_settings_roundtrip", run_db_settings_roundtrip, &mut results);

    // --- Export iCal ---
    run("ical_entries_to_ical", run_ical_entries_to_ical, &mut results);

    let passed = results.iter().filter(|r| r.passed).count() as u32;
    let failed = results.len() as u32 - passed;
    let status = if failed == 0 {
        "ok".to_string()
    } else {
        "partial".to_string()
    };

    JayKoaTestReport {
        passed,
        failed,
        total: results.len() as u32,
        status,
        results,
    }
}

fn run<F>(name: &str, f: F, results: &mut Vec<JayKoaTestItem>)
where
    F: FnOnce() -> Result<(), String>,
{
    match f() {
        Ok(()) => results.push(JayKoaTestItem {
            name: name.to_string(),
            passed: true,
            message: None,
        }),
        Err(e) => results.push(JayKoaTestItem {
            name: name.to_string(),
            passed: false,
            message: Some(e),
        }),
    }
}

// --- Types ---

fn run_entry_type_roundtrip() -> Result<(), String> {
    let variants = [
        EntryType::Internal,
        EntryType::ReflectJayFestival,
        EntryType::ReflectJayRDV,
        EntryType::ReflectOther,
    ];
    for v in variants {
        let roundtripped = EntryType::from_str(v.as_str());
        if roundtripped != v {
            return Err(format!("roundtrip failed for {:?}", v));
        }
    }
    Ok(())
}

fn run_entry_type_readonly() -> Result<(), String> {
    if EntryType::Internal.is_readonly() {
        return Err("Internal should NOT be readonly".to_string());
    }
    if !EntryType::ReflectJayFestival.is_readonly()
        || !EntryType::ReflectJayRDV.is_readonly()
        || !EntryType::ReflectOther.is_readonly()
    {
        return Err("Reflect* should be readonly".to_string());
    }
    Ok(())
}

fn run_temporal_status_roundtrip() -> Result<(), String> {
    let variants = [
        TemporalStatus::Confirmed,
        TemporalStatus::Tentative,
        TemporalStatus::Cancelled,
    ];
    for v in variants {
        let roundtripped = TemporalStatus::from_str(v.as_str());
        if roundtripped != v {
            return Err(format!("roundtrip failed for {:?}", v));
        }
    }
    Ok(())
}

fn run_event_source_roundtrip() -> Result<(), String> {
    let variants = [
        EventSource::JayKoa,
        EventSource::JayFestival,
        EventSource::JayRDV,
        EventSource::Other,
    ];
    for v in variants {
        let roundtripped = EventSource::from_str(v.as_str());
        if roundtripped != v {
            return Err(format!("roundtrip failed for {:?}", v));
        }
    }
    Ok(())
}

fn run_calendar_type_roundtrip() -> Result<(), String> {
    let variants = [
        CalendarType::Personal,
        CalendarType::Work,
        CalendarType::SyncedService,
    ];
    for v in variants {
        let roundtripped = CalendarType::from_str(v.as_str());
        if roundtripped != v {
            return Err(format!("roundtrip failed for {:?}", v));
        }
    }
    Ok(())
}

fn run_default_user_settings() -> Result<(), String> {
    let s = UserSettings::default();
    if s.default_view != "week"
        || s.week_start_day != 0
        || s.day_start_hour != 8
        || s.day_end_hour != 20
        || s.timezone != "Europe/Paris"
    {
        return Err("default UserSettings mismatch".to_string());
    }
    Ok(())
}

// --- DB helpers ---

fn mem_db() -> Result<JayKoaDb, String> {
    JayKoaDb::open(":memory:").map_err(|e| e.to_string())
}

fn make_agenda(db: &JayKoaDb, profile_id: &str, name: &str) -> Result<String, String> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let agenda = Agenda {
        id: Some(id.clone()),
        profile_id: Some(profile_id.to_string()),
        name: Some(name.to_string()),
        description: None,
        color: Some("#4285F4".to_string()),
        calendar_type: Some("personal".to_string()),
        visible: true,
        is_default: false,
        source_service: None,
        created_at: Some(now.clone()),
        updated_at: Some(now),
    };
    db.agenda_insert(&agenda).map_err(|e| e.to_string())?;
    Ok(id)
}

fn make_entry(
    db: &JayKoaDb,
    agenda_id: &str,
    title: &str,
    start: &str,
    end: &str,
    entry_type: &str,
) -> Result<String, String> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let entry = TemporalEntry {
        id: Some(id.clone()),
        agenda_id: Some(agenda_id.to_string()),
        title: Some(title.to_string()),
        description: None,
        start_datetime: Some(start.to_string()),
        end_datetime: Some(end.to_string()),
        all_day: false,
        location: None,
        status: Some("confirmed".to_string()),
        entry_type: Some(entry_type.to_string()),
        source_service: None,
        source_event_id: None,
        color: None,
        recurrence_rule: None,
        reminders_json: None,
        created_at: Some(now.clone()),
        updated_at: Some(now),
        last_synced_at: None,
    };
    db.entry_insert(&entry).map_err(|e| e.to_string())?;
    Ok(id)
}

fn run_db_open_and_init() -> Result<(), String> {
    let db = mem_db()?;
    if db.instance.instance_type != InstanceType::Daughter {
        return Err("instance_type should be Daughter".to_string());
    }
    Ok(())
}

fn run_db_agenda_crud() -> Result<(), String> {
    let db = mem_db()?;
    let profile = "profile-test-1";
    let agenda_id = make_agenda(&db, profile, "Mon Agenda")?;
    let agendas = db.agendas_by_profile(profile).map_err(|e| e.to_string())?;
    if agendas.len() != 1 || agendas[0].id.as_deref() != Some(agenda_id.as_str()) {
        return Err("agenda read failed".to_string());
    }
    db.agenda_delete(&agenda_id).map_err(|e| e.to_string())?;
    let agendas = db.agendas_by_profile(profile).map_err(|e| e.to_string())?;
    if !agendas.is_empty() {
        return Err("agenda list should be empty after delete".to_string());
    }
    Ok(())
}

fn run_db_entry_crud() -> Result<(), String> {
    let db = mem_db()?;
    let profile = "profile-entry-crud";
    let agenda_id = make_agenda(&db, profile, "Agenda CRUD")?;
    let entry_id = make_entry(
        &db,
        &agenda_id,
        "Réunion",
        "2026-02-06T10:00:00",
        "2026-02-06T11:00:00",
        "internal",
    )?;
    let entry = db
        .entry_by_id(&entry_id)
        .map_err(|e| e.to_string())?
        .ok_or("entry should exist")?;
    if entry.title.as_deref() != Some("Réunion") {
        return Err("entry title mismatch".to_string());
    }
    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let updated = TemporalEntry {
        id: Some(entry_id.clone()),
        title: Some("Réunion mise à jour".to_string()),
        description: Some("Avec nouvelle description".to_string()),
        start_datetime: Some("2026-02-06T10:30:00".to_string()),
        end_datetime: Some("2026-02-06T11:30:00".to_string()),
        status: Some("tentative".to_string()),
        updated_at: Some(now),
        ..TemporalEntry::default()
    };
    db.entry_update(&updated).map_err(|e| e.to_string())?;
    let entry = db
        .entry_by_id(&entry_id)
        .map_err(|e| e.to_string())?
        .ok_or("entry should still exist after update")?;
    if entry.title.as_deref() != Some("Réunion mise à jour") {
        return Err("entry title after update mismatch".to_string());
    }
    db.entry_delete(&entry_id).map_err(|e| e.to_string())?;
    let entry = db.entry_by_id(&entry_id).map_err(|e| e.to_string())?;
    if entry.is_some() {
        return Err("entry should be gone after delete".to_string());
    }
    Ok(())
}

fn run_db_reflect_upsert() -> Result<(), String> {
    let db = mem_db()?;
    let profile = "profile-reflect";
    let agenda_id = make_agenda(&db, profile, "Synced")?;
    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let source_svc = "jayfestival";
    let source_eid = "ext-event-42";
    let reflect_id = uuid::Uuid::new_v4().to_string();
    let reflect = TemporalEntry {
        id: Some(reflect_id.clone()),
        agenda_id: Some(agenda_id.clone()),
        title: Some("Festival opening".to_string()),
        start_datetime: Some("2026-07-01T18:00:00".to_string()),
        end_datetime: Some("2026-07-01T22:00:00".to_string()),
        entry_type: Some("reflect_jayfestival".to_string()),
        source_service: Some(source_svc.to_string()),
        source_event_id: Some(source_eid.to_string()),
        status: Some("confirmed".to_string()),
        created_at: Some(now.clone()),
        updated_at: Some(now.clone()),
        last_synced_at: Some(now.clone()),
        ..TemporalEntry::default()
    };
    db.reflect_upsert(&reflect).map_err(|e| e.to_string())?;
    let entry = db
        .entry_by_id(&reflect_id)
        .map_err(|e| e.to_string())?
        .ok_or("reflect should exist")?;
    if entry.title.as_deref() != Some("Festival opening") {
        return Err("reflect title mismatch".to_string());
    }
    let now2 = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let reflect2 = TemporalEntry {
        id: Some(uuid::Uuid::new_v4().to_string()),
        agenda_id: Some(agenda_id.clone()),
        title: Some("Festival opening (updated)".to_string()),
        start_datetime: Some("2026-07-01T19:00:00".to_string()),
        end_datetime: Some("2026-07-01T23:00:00".to_string()),
        entry_type: Some("reflect_jayfestival".to_string()),
        source_service: Some(source_svc.to_string()),
        source_event_id: Some(source_eid.to_string()),
        status: Some("confirmed".to_string()),
        updated_at: Some(now2.clone()),
        last_synced_at: Some(now2),
        ..TemporalEntry::default()
    };
    db.reflect_upsert(&reflect2).map_err(|e| e.to_string())?;
    let entry = db
        .entry_by_id(&reflect_id)
        .map_err(|e| e.to_string())?
        .ok_or("reflect should still exist after upsert")?;
    if entry.title.as_deref() != Some("Festival opening (updated)") {
        return Err("reflect title after upsert mismatch".to_string());
    }
    let entries = db.entries_by_agenda(&agenda_id).map_err(|e| e.to_string())?;
    if entries.len() != 1 {
        return Err("upsert should not create duplicates".to_string());
    }
    Ok(())
}

fn run_db_detect_conflicts() -> Result<(), String> {
    let db = mem_db()?;
    let profile = "profile-conflict";
    let agenda_id = make_agenda(&db, profile, "Conflits")?;
    make_entry(
        &db,
        &agenda_id,
        "A",
        "2026-03-01T10:00:00",
        "2026-03-01T11:00:00",
        "internal",
    )?;
    make_entry(
        &db,
        &agenda_id,
        "B",
        "2026-03-01T10:30:00",
        "2026-03-01T11:30:00",
        "internal",
    )?;
    let conflicts = db
        .detect_conflicts(&[agenda_id], "2026-03-01T00:00:00", "2026-03-01T23:59:59")
        .map_err(|e| e.to_string())?;
    if conflicts.len() != 1 {
        return Err(format!("expected 1 conflict, got {}", conflicts.len()));
    }
    Ok(())
}

fn run_db_detect_no_conflict() -> Result<(), String> {
    let db = mem_db()?;
    let profile = "profile-no-conflict";
    let agenda_id = make_agenda(&db, profile, "Pas de conflit")?;
    make_entry(
        &db,
        &agenda_id,
        "Matin",
        "2026-03-01T09:00:00",
        "2026-03-01T10:00:00",
        "internal",
    )?;
    make_entry(
        &db,
        &agenda_id,
        "Midi",
        "2026-03-01T11:00:00",
        "2026-03-01T12:00:00",
        "internal",
    )?;
    let conflicts = db
        .detect_conflicts(&[agenda_id], "2026-03-01T00:00:00", "2026-03-01T23:59:59")
        .map_err(|e| e.to_string())?;
    if !conflicts.is_empty() {
        return Err("expected 0 conflicts".to_string());
    }
    Ok(())
}

fn run_db_check_conflict_api() -> Result<(), String> {
    let db = mem_db()?;
    let profile = "profile-check-api";
    let agenda_a = make_agenda(&db, profile, "Pro")?;
    let agenda_b = make_agenda(&db, profile, "Perso")?;
    make_entry(
        &db,
        &agenda_a,
        "Call client",
        "2026-04-10T14:00:00",
        "2026-04-10T15:00:00",
        "internal",
    )?;
    make_entry(
        &db,
        &agenda_b,
        "Sport",
        "2026-04-10T14:30:00",
        "2026-04-10T15:30:00",
        "internal",
    )?;
    let (has_conflict, conflicts) = db
        .check_conflict(profile, "2026-04-10T00:00:00", "2026-04-10T23:59:59")
        .map_err(|e| e.to_string())?;
    if !has_conflict || conflicts.len() != 1 {
        return Err("cross-agenda overlap should be detected".to_string());
    }
    Ok(())
}

fn run_db_settings_roundtrip() -> Result<(), String> {
    let db = mem_db()?;
    let profile = "profile-settings";
    let settings = UserSettings {
        profile_id: Some(profile.to_string()),
        default_view: "month".to_string(),
        week_start_day: 6,
        day_start_hour: 7,
        day_end_hour: 22,
        timezone: "America/New_York".to_string(),
        default_reminder_minutes: 15,
        use_24h_format: false,
        show_week_numbers: true,
        show_cancelled_events: true,
    };
    db.settings_upsert(&settings).map_err(|e| e.to_string())?;
    let loaded = db.settings_get(profile).map_err(|e| e.to_string())?;
    if loaded.default_view != "month"
        || loaded.week_start_day != 6
        || loaded.timezone != "America/New_York"
        || loaded.use_24h_format
        || !loaded.show_week_numbers
    {
        return Err("settings roundtrip mismatch".to_string());
    }
    Ok(())
}

fn run_ical_entries_to_ical() -> Result<(), String> {
    let entries = vec![TemporalEntry {
        id: Some("test-1".to_string()),
        title: Some("Test event".to_string()),
        start_datetime: Some("2026-06-01T10:00:00".to_string()),
        end_datetime: Some("2026-06-01T11:00:00".to_string()),
        ..TemporalEntry::default()
    }];
    let ics = ical::entries_to_ical(&entries, "JayKoa Test");
    if !ics.contains("BEGIN:VCALENDAR")
        || !ics.contains("BEGIN:VEVENT")
        || !ics.contains("UID:test-1")
        || !ics.contains("SUMMARY:Test event")
    {
        return Err("iCal output missing expected fields".to_string());
    }
    Ok(())
}
