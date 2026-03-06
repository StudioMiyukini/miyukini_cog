//! E9-02 -- Tests E2E CalDAV: full CRUD flow simulant un client CalDAV.
//!
//! Teste le service CalDAV en mode direct (sans HTTP) pour valider
//! le cycle complet: creation calendrier -> ajout evenements -> requetes
//! calendar-multiget/query -> modification -> suppression.

use std::sync::{Arc, Mutex};

use miyucloud_dav::caldav::service::CalDavService;

fn new_service() -> CalDavService {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    let svc = CalDavService::new(Arc::new(Mutex::new(conn)));
    svc.init_tables().unwrap();
    svc
}

// ════════════════════════════════════════════════════════════════════════════
// Scenario 1: Thunderbird-like flow
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn thunderbird_flow_create_calendar_and_events() {
    let svc = new_service();
    let owner = "user-thunderbird";

    // 1. Creer un calendrier
    let cal = svc.create_calendar(owner, "Travail").unwrap();
    assert_eq!(cal.display_name, "Travail");
    let cal_id = cal.id.to_string();

    // 2. Lister les calendriers
    let cals = svc.list_calendars(owner).unwrap();
    assert_eq!(cals.len(), 1);
    assert_eq!(cals[0].display_name, "Travail");

    // 3. Ajouter un evenement iCal
    let ical_data = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:Reunion standup\r\nDTSTART:20260310T090000Z\r\nDTEND:20260310T093000Z\r\nUID:evt-001@miyucloud\r\nEND:VEVENT\r\nEND:VCALENDAR";
    let evt = svc.create_event(&cal_id, ical_data).unwrap();
    assert_eq!(evt.uid, "evt-001@miyucloud");
    assert!(!evt.etag.is_empty());

    // 4. Ajouter un second evenement
    let ical_data2 = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:Dejeuner equipe\r\nDTSTART:20260310T120000Z\r\nDTEND:20260310T130000Z\r\nUID:evt-002@miyucloud\r\nEND:VEVENT\r\nEND:VCALENDAR";
    svc.create_event(&cal_id, ical_data2).unwrap();

    // 5. Lister les evenements
    let events = svc.list_events(&cal_id).unwrap();
    assert_eq!(events.len(), 2);

    // 6. calendar-multiget: recuperer des evenements par UID
    let uids = vec!["evt-001@miyucloud".to_string()];
    let subset = svc.get_events_by_uids(&cal_id, &uids).unwrap();
    assert_eq!(subset.len(), 1);
    assert_eq!(subset[0].uid, "evt-001@miyucloud");

    // 7. Modifier un evenement
    let updated_ical = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:Reunion standup (modifie)\r\nDTSTART:20260310T100000Z\r\nDTEND:20260310T103000Z\r\nUID:evt-001@miyucloud\r\nEND:VEVENT\r\nEND:VCALENDAR";
    let updated = svc
        .update_event(&cal_id, "evt-001@miyucloud", updated_ical)
        .unwrap();
    assert_ne!(updated.etag, evt.etag);

    // 8. Verifier la modification
    let fetched = svc
        .get_event(&cal_id, "evt-001@miyucloud")
        .unwrap()
        .unwrap();
    assert!(fetched.ical_data.contains("modifie"));

    // 9. Supprimer un evenement
    svc.delete_event(&cal_id, "evt-002@miyucloud").unwrap();
    let remaining = svc.list_events(&cal_id).unwrap();
    assert_eq!(remaining.len(), 1);

    // 10. Supprimer le calendrier (cascade)
    svc.delete_calendar(&cal_id).unwrap();
    let cals = svc.list_calendars(owner).unwrap();
    assert_eq!(cals.len(), 0);
}

// ════════════════════════════════════════════════════════════════════════════
// Scenario 2: macOS Calendar-like flow (multiple calendars)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn macos_flow_multiple_calendars() {
    let svc = new_service();
    let owner = "user-macos";

    let perso = svc.create_calendar(owner, "Personnel").unwrap();
    let pro = svc.create_calendar(owner, "Professionnel").unwrap();
    let anniv = svc.create_calendar(owner, "Anniversaires").unwrap();
    let perso_id = perso.id.to_string();
    let pro_id = pro.id.to_string();
    let anniv_id = anniv.id.to_string();

    let cals = svc.list_calendars(owner).unwrap();
    assert_eq!(cals.len(), 3);

    let ical_perso = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:Course\r\nDTSTART:20260315T100000Z\r\nUID:p1@mac\r\nEND:VEVENT\r\nEND:VCALENDAR";
    svc.create_event(&perso_id, ical_perso).unwrap();

    let ical_pro = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:Client call\r\nDTSTART:20260315T140000Z\r\nUID:w1@mac\r\nEND:VEVENT\r\nEND:VCALENDAR";
    svc.create_event(&pro_id, ical_pro).unwrap();

    let ical_anniv = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:Anniversaire Marie\r\nDTSTART:20260315\r\nUID:a1@mac\r\nEND:VEVENT\r\nEND:VCALENDAR";
    svc.create_event(&anniv_id, ical_anniv).unwrap();

    assert_eq!(svc.list_events(&perso_id).unwrap().len(), 1);
    assert_eq!(svc.list_events(&pro_id).unwrap().len(), 1);
    assert_eq!(svc.list_events(&anniv_id).unwrap().len(), 1);

    // Supprimer le calendrier Pro (cascade)
    svc.delete_calendar(&pro_id).unwrap();
    assert_eq!(svc.list_calendars(owner).unwrap().len(), 2);
    assert!(svc.list_events(&pro_id).unwrap().is_empty());
}

// ════════════════════════════════════════════════════════════════════════════
// Scenario 3: DAVx5-like flow (CTag changes)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn davx5_flow_ctag_changes_on_modification() {
    let svc = new_service();
    let owner = "user-davx5";

    let cal = svc.create_calendar(owner, "Sync Cal").unwrap();
    let cal_id = cal.id.to_string();
    let initial_ctag = cal.ctag.clone();

    let ical = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:Test\r\nDTSTART:20260320T080000Z\r\nUID:sync1@davx5\r\nEND:VEVENT\r\nEND:VCALENDAR";
    svc.create_event(&cal_id, ical).unwrap();

    let cal_updated = svc.get_calendar(&cal_id).unwrap().unwrap();
    assert_ne!(cal_updated.ctag, initial_ctag);

    let ctag_after_create = cal_updated.ctag.clone();

    let ical_mod = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:Test modifie\r\nDTSTART:20260320T090000Z\r\nUID:sync1@davx5\r\nEND:VEVENT\r\nEND:VCALENDAR";
    svc.update_event(&cal_id, "sync1@davx5", ical_mod).unwrap();

    let cal_final = svc.get_calendar(&cal_id).unwrap().unwrap();
    assert_ne!(cal_final.ctag, ctag_after_create);

    let ctag_before_delete = cal_final.ctag.clone();
    svc.delete_event(&cal_id, "sync1@davx5").unwrap();
    let cal_after_delete = svc.get_calendar(&cal_id).unwrap().unwrap();
    assert_ne!(cal_after_delete.ctag, ctag_before_delete);
}

// ════════════════════════════════════════════════════════════════════════════
// Scenario 4: Isolation entre proprietaires
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn owner_isolation_calendars_not_shared() {
    let svc = new_service();

    svc.create_calendar("alice", "Alice Cal").unwrap();
    svc.create_calendar("bob", "Bob Cal").unwrap();

    let alice_cals = svc.list_calendars("alice").unwrap();
    assert_eq!(alice_cals.len(), 1);
    assert_eq!(alice_cals[0].display_name, "Alice Cal");

    let bob_cals = svc.list_calendars("bob").unwrap();
    assert_eq!(bob_cals.len(), 1);
    assert_eq!(bob_cals[0].display_name, "Bob Cal");

    assert_ne!(alice_cals[0].id, bob_cals[0].id);
}

// ════════════════════════════════════════════════════════════════════════════
// Scenario 5: multiget avec UIDs multiples
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn calendar_multiget_multiple_uids() {
    let svc = new_service();
    let owner = "user-multiget";
    let cal = svc.create_calendar(owner, "Multiget Test").unwrap();
    let cal_id = cal.id.to_string();

    for i in 1..=5 {
        let ical = format!(
            "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:Event {i}\r\nDTSTART:202603{i:02}T100000Z\r\nUID:mg-{i}@test\r\nEND:VEVENT\r\nEND:VCALENDAR"
        );
        svc.create_event(&cal_id, &ical).unwrap();
    }

    let uids = vec![
        "mg-1@test".to_string(),
        "mg-3@test".to_string(),
        "mg-5@test".to_string(),
    ];
    let result = svc.get_events_by_uids(&cal_id, &uids).unwrap();
    assert_eq!(result.len(), 3);

    let fetched_uids: Vec<&str> = result.iter().map(|e| e.uid.as_str()).collect();
    assert!(fetched_uids.contains(&"mg-1@test"));
    assert!(fetched_uids.contains(&"mg-3@test"));
    assert!(fetched_uids.contains(&"mg-5@test"));

    // Multiget avec UID inexistant
    let uids_missing = vec!["mg-99@test".to_string()];
    let result_missing = svc.get_events_by_uids(&cal_id, &uids_missing).unwrap();
    assert!(result_missing.is_empty());
}
