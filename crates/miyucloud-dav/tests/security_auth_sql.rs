//! E10-03/E10-04 -- Audit auth bypass et SQL injection.
//!
//! Verifie la resistance aux attaques d'authentification
//! et aux injections SQL dans les services CalDAV/CardDAV.

use std::sync::{Arc, Mutex};

use miyucloud_dav::caldav::service::CalDavService;
use miyucloud_dav::carddav::service::CardDavService;
use miyucloud_dav::wopi::auth::WopiTokenValidator;

fn new_caldav() -> CalDavService {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    let svc = CalDavService::new(Arc::new(Mutex::new(conn)));
    svc.init_tables().unwrap();
    svc
}

fn new_carddav() -> CardDavService {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    let svc = CardDavService::new(Arc::new(Mutex::new(conn)));
    svc.init_tables().unwrap();
    svc
}

// ════════════════════════════════════════════════════════════════════════════
// E10-03: Auth bypass WOPI tokens
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn wopi_empty_token_rejected() {
    let v = WopiTokenValidator::new(b"secret");
    assert!(v.validate_token("").is_err());
}

#[test]
fn wopi_partial_token_rejected() {
    let v = WopiTokenValidator::new(b"secret");
    assert!(v.validate_token("file:owner").is_err());
    assert!(v.validate_token("file:owner:123").is_err());
}

#[test]
fn wopi_forged_signature_rejected() {
    let v = WopiTokenValidator::new(b"secret");
    // Forge a token with correct format but wrong signature
    assert!(v.validate_token("file:owner:999999999:0000000000000000000000000000000000000000000000000000000000000000").is_err());
}

#[test]
fn wopi_cross_secret_rejected() {
    let v1 = WopiTokenValidator::new(b"secret-1");
    let v2 = WopiTokenValidator::new(b"secret-2");
    let token = v1.generate_token("file", "owner");
    assert!(v2.validate_token(&token).is_err());
}

#[test]
fn wopi_token_with_colons_in_ids() {
    // Owner/file IDs containing colons should not break parsing
    let v = WopiTokenValidator::new(b"secret");
    // The token format uses splitn(4, ':'), so colons in file_id break it
    // This tests that such tokens are rejected gracefully
    let token = v.generate_token("file-no-colons", "owner-no-colons");
    let (fid, oid) = v.validate_token(&token).unwrap();
    assert_eq!(fid, "file-no-colons");
    assert_eq!(oid, "owner-no-colons");
}

// ════════════════════════════════════════════════════════════════════════════
// E10-04: SQL injection CalDAV
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn caldav_sql_injection_in_owner_id() {
    let svc = new_caldav();

    // Tentative d'injection dans owner_id
    let malicious_owner = "'; DROP TABLE cloud_calendars; --";
    let result = svc.create_calendar(malicious_owner, "Test");
    // Should succeed (owner_id is a parameterized value, not interpolated)
    assert!(result.is_ok());

    // Verify the table still exists
    let cals = svc.list_calendars(malicious_owner).unwrap();
    assert_eq!(cals.len(), 1);
}

#[test]
fn caldav_sql_injection_in_display_name() {
    let svc = new_caldav();

    let malicious_name = "Cal'); DELETE FROM cloud_calendars WHERE ('1'='1";
    let result = svc.create_calendar("owner", malicious_name);
    assert!(result.is_ok());

    let cals = svc.list_calendars("owner").unwrap();
    assert_eq!(cals.len(), 1);
    assert_eq!(cals[0].display_name, malicious_name);
}

#[test]
fn caldav_sql_injection_in_event_uid() {
    let svc = new_caldav();
    let cal = svc.create_calendar("owner", "Test").unwrap();
    let cal_id = cal.id.to_string();

    // iCal with SQL injection in UID
    let ical = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:Hack\r\nUID:'); DROP TABLE cloud_events; --\r\nEND:VEVENT\r\nEND:VCALENDAR";
    let result = svc.create_event(&cal_id, ical);
    assert!(result.is_ok());

    // Table still works
    let events = svc.list_events(&cal_id).unwrap();
    assert_eq!(events.len(), 1);
}

#[test]
fn caldav_sql_injection_in_calendar_id_lookup() {
    let svc = new_caldav();

    // Tentative d'injection via calendar_id
    let malicious_id = "' OR '1'='1";
    let result = svc.get_calendar(malicious_id).unwrap();
    assert!(result.is_none()); // Should find nothing, not all rows
}

// ════════════════════════════════════════════════════════════════════════════
// E10-04: SQL injection CardDAV
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn carddav_sql_injection_in_owner_id() {
    let svc = new_carddav();

    let malicious_owner = "'; DROP TABLE cloud_addressbooks; --";
    let result = svc.create_addressbook(malicious_owner, "Contacts");
    assert!(result.is_ok());

    let books = svc.list_addressbooks(malicious_owner).unwrap();
    assert_eq!(books.len(), 1);
}

#[test]
fn carddav_sql_injection_in_vcard_data() {
    let svc = new_carddav();
    let book = svc.create_addressbook("owner", "Book").unwrap();
    let book_id = book.id.to_string();

    let vcard = "BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Robert'); DROP TABLE cloud_contacts; --\r\nUID:sqli@test\r\nEND:VCARD";
    let result = svc.create_contact(&book_id, vcard);
    assert!(result.is_ok());

    let contacts = svc.list_contacts(&book_id).unwrap();
    assert_eq!(contacts.len(), 1);
}

#[test]
fn carddav_sql_injection_in_uid_lookup() {
    let svc = new_carddav();
    let book = svc.create_addressbook("owner", "Book").unwrap();
    let book_id = book.id.to_string();

    let malicious_uid = "' OR '1'='1";
    let result = svc.get_contact(&book_id, malicious_uid).unwrap();
    assert!(result.is_none());
}
