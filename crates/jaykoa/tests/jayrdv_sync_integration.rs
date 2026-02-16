//! Test d'intégration JayRDV → JayKoa : sync_appointments_from_store.
//!
//! Vérifie que les rendez-vous confirmés du JayRdvStore sont bien reflétés
//! dans un agenda JayKoa (reflets en lecture seule).
//!
//! Ce test nécessite un service KindMother disponible (default: 127.0.0.1:50051).
//! Sans serveur, exécuter avec `cargo test --package jaykoa jayrdv_sync_integration -- --ignored`
//! lorsque le serveur est démarré.

use jaykoa::data::{Agenda, CalendarType, JayKoaDb};
use jaykoa::services::jayrdv::JayRDVAdapter;
use jayrdv::{AppointmentStatus, JayRdvStore};
use std::sync::Arc;
use uuid::Uuid;

fn make_confirmed_appointment(
    id: &str,
    title: &str,
    start: &str,
    end: &str,
    location: Option<&str>,
    now: &str,
) -> jayrdv::Appointment {
    jayrdv::Appointment {
        id: id.to_string(),
        title: title.to_string(),
        start_at: start.to_string(),
        end_at: end.to_string(),
        resource_id: None,
        slot_id: None,
        service_id: None,
        status: AppointmentStatus::Confirmed,
        client_email: None,
        client_name: None,
        client_phone: None,
        notes: None,
        location: location.map(String::from),
        cancellation_reason: None,
        cancelled_by: None,
        created_at: now.to_string(),
        updated_at: now.to_string(),
    }
}

#[test]
#[ignore = "Requires KindMother server running (e.g. kindmother-service)"]
fn sync_appointments_from_store_creates_reflections() {
    let now = chrono::Utc::now().to_rfc3339();
    let store = JayRdvStore::new();

    let a1 = make_confirmed_appointment(
        "rdv-sync-1",
        "RDV Test Alpha",
        "2026-06-01T10:00:00+00:00",
        "2026-06-01T11:00:00+00:00",
        Some("Salle A"),
        &now,
    );
    let a2 = make_confirmed_appointment(
        "rdv-sync-2",
        "RDV Test Beta",
        "2026-06-02T14:00:00+00:00",
        "2026-06-02T15:00:00+00:00",
        None,
        &now,
    );
    store.appointment_create(a1.clone()).expect("create a1");
    store.appointment_create(a2.clone()).expect("create a2");

    let _ = JayKoaDb::init_global_sync(None).expect("KindMother init");
    let db = JayKoaDb::open(".jaykoa_jayrdv_test").expect("open db");
    db.init_schema_sync().expect("init schema");
    let db = Arc::new(db);

    let agenda_id = Uuid::new_v4().to_string();
    let agenda = Agenda {
        id: Some(agenda_id.clone()),
        profile_id: Some("test-profile".to_string()),
        name: Some("Test Agenda JayRDV".to_string()),
        description: None,
        color: Some("#4285F4".to_string()),
        calendar_type: Some(CalendarType::Personal.as_str().to_string()),
        visible: true,
        is_default: false,
        source_service: None,
        created_at: Some(now.clone()),
        updated_at: Some(now.clone()),
    };
    db.agenda_insert(&agenda).expect("agenda_insert");

    let count = JayRDVAdapter::sync_appointments_from_store(
        &db,
        &agenda_id,
        &store,
        Some("2026-06-01T00:00:00+00:00"),
        Some("2026-06-30T23:59:59+00:00"),
    )
    .expect("sync");

    assert_eq!(count, 2, "two confirmed appointments should be synced");

    let entries = db.entries_by_agenda(&agenda_id).expect("entries_by_agenda");
    assert_eq!(entries.len(), 2);
    let titles: Vec<String> = entries
        .iter()
        .filter_map(|e| e.title.clone())
        .collect();
    assert!(titles.contains(&"RDV Test Alpha".to_string()));
    assert!(titles.contains(&"RDV Test Beta".to_string()));
    let source_ids: Vec<String> = entries
        .iter()
        .filter_map(|e| e.source_event_id.clone())
        .collect();
    assert!(source_ids.contains(&"rdv-sync-1".to_string()));
    assert!(source_ids.contains(&"rdv-sync-2".to_string()));
}
