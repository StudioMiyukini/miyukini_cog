//! E9-03 -- Tests E2E CardDAV: full CRUD flow simulant un client CardDAV.
//!
//! Teste le service CardDAV en mode direct (sans HTTP) pour valider
//! le cycle complet: creation carnet -> ajout contacts -> requetes
//! addressbook-multiget -> modification -> suppression.

use std::sync::{Arc, Mutex};

use miyucloud_dav::carddav::service::CardDavService;

fn new_service() -> CardDavService {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    let svc = CardDavService::new(Arc::new(Mutex::new(conn)));
    svc.init_tables().unwrap();
    svc
}

// ════════════════════════════════════════════════════════════════════════════
// Scenario 1: DAVx5-like contact sync
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn davx5_flow_create_book_and_contacts() {
    let svc = new_service();
    let owner = "user-davx5";

    // 1. Creer un carnet
    let book = svc.create_addressbook(owner, "Contacts").unwrap();
    assert_eq!(book.display_name, "Contacts");
    let book_id = book.id.to_string();

    // 2. Lister les carnets
    let books = svc.list_addressbooks(owner).unwrap();
    assert_eq!(books.len(), 1);

    // 3. Ajouter un contact vCard
    let vcard = "BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Jean Dupont\r\nEMAIL:jean@example.com\r\nTEL:+33612345678\r\nUID:ct-001@miyucloud\r\nEND:VCARD";
    let ct = svc.create_contact(&book_id, vcard).unwrap();
    assert_eq!(ct.uid, "ct-001@miyucloud");
    assert!(!ct.etag.is_empty());

    // 4. Ajouter un second contact
    let vcard2 = "BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Marie Martin\r\nEMAIL:marie@example.com\r\nUID:ct-002@miyucloud\r\nEND:VCARD";
    svc.create_contact(&book_id, vcard2).unwrap();

    // 5. Lister les contacts
    let contacts = svc.list_contacts(&book_id).unwrap();
    assert_eq!(contacts.len(), 2);

    // 6. addressbook-multiget: par UID
    let uids = vec!["ct-001@miyucloud".to_string()];
    let subset = svc.get_contacts_by_uids(&book_id, &uids).unwrap();
    assert_eq!(subset.len(), 1);
    assert_eq!(subset[0].uid, "ct-001@miyucloud");

    // 7. Modifier un contact
    let updated_vcard = "BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Jean Dupont-Martin\r\nEMAIL:jean.dm@example.com\r\nTEL:+33612345678\r\nUID:ct-001@miyucloud\r\nEND:VCARD";
    let updated = svc
        .update_contact(&book_id, "ct-001@miyucloud", updated_vcard)
        .unwrap();
    assert_ne!(updated.etag, ct.etag);

    // 8. Verifier la modification
    let fetched = svc
        .get_contact(&book_id, "ct-001@miyucloud")
        .unwrap()
        .unwrap();
    assert!(fetched.vcard_data.contains("Jean Dupont-Martin"));

    // 9. Supprimer un contact
    svc.delete_contact(&book_id, "ct-002@miyucloud").unwrap();
    let remaining = svc.list_contacts(&book_id).unwrap();
    assert_eq!(remaining.len(), 1);

    // 10. Supprimer le carnet (cascade)
    svc.delete_addressbook(&book_id).unwrap();
    let books = svc.list_addressbooks(owner).unwrap();
    assert_eq!(books.len(), 0);
}

// ════════════════════════════════════════════════════════════════════════════
// Scenario 2: macOS Contacts-like flow (multiple address books)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn macos_flow_multiple_addressbooks() {
    let svc = new_service();
    let owner = "user-macos";

    let perso = svc.create_addressbook(owner, "Personnel").unwrap();
    let pro = svc.create_addressbook(owner, "Professionnel").unwrap();
    let perso_id = perso.id.to_string();
    let pro_id = pro.id.to_string();

    let books = svc.list_addressbooks(owner).unwrap();
    assert_eq!(books.len(), 2);

    let vcard_perso = "BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Ami Pierre\r\nUID:perso1@mac\r\nEND:VCARD";
    svc.create_contact(&perso_id, vcard_perso).unwrap();

    let vcard_pro = "BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Client Sophie\r\nUID:pro1@mac\r\nEND:VCARD";
    svc.create_contact(&pro_id, vcard_pro).unwrap();

    assert_eq!(svc.list_contacts(&perso_id).unwrap().len(), 1);
    assert_eq!(svc.list_contacts(&pro_id).unwrap().len(), 1);

    svc.delete_addressbook(&pro_id).unwrap();
    assert_eq!(svc.list_addressbooks(owner).unwrap().len(), 1);
    assert!(svc.list_contacts(&pro_id).unwrap().is_empty());
}

// ════════════════════════════════════════════════════════════════════════════
// Scenario 3: CTag change tracking
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn ctag_changes_on_contact_modification() {
    let svc = new_service();
    let owner = "user-ctag";

    let book = svc.create_addressbook(owner, "CTag Test").unwrap();
    let book_id = book.id.to_string();
    let initial_ctag = book.ctag.clone();

    let vcard = "BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Test Contact\r\nUID:ctag1@test\r\nEND:VCARD";
    svc.create_contact(&book_id, vcard).unwrap();

    let book_after = svc.get_addressbook(&book_id).unwrap().unwrap();
    assert_ne!(book_after.ctag, initial_ctag);

    let ctag_after_create = book_after.ctag.clone();

    let vcard_mod = "BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Test Contact Modifie\r\nUID:ctag1@test\r\nEND:VCARD";
    svc.update_contact(&book_id, "ctag1@test", vcard_mod)
        .unwrap();

    let book_final = svc.get_addressbook(&book_id).unwrap().unwrap();
    assert_ne!(book_final.ctag, ctag_after_create);
}

// ════════════════════════════════════════════════════════════════════════════
// Scenario 4: Owner isolation
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn owner_isolation_addressbooks_not_shared() {
    let svc = new_service();

    svc.create_addressbook("alice", "Alice Contacts").unwrap();
    svc.create_addressbook("bob", "Bob Contacts").unwrap();

    let alice_books = svc.list_addressbooks("alice").unwrap();
    assert_eq!(alice_books.len(), 1);
    assert_eq!(alice_books[0].display_name, "Alice Contacts");

    let bob_books = svc.list_addressbooks("bob").unwrap();
    assert_eq!(bob_books.len(), 1);
    assert_eq!(bob_books[0].display_name, "Bob Contacts");
}

// ════════════════════════════════════════════════════════════════════════════
// Scenario 5: vCard 4.0 handling
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn vcard_40_support() {
    let svc = new_service();
    let owner = "user-v4";
    let book = svc.create_addressbook(owner, "v4 Book").unwrap();
    let book_id = book.id.to_string();

    let vcard = "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Sophie Durand\r\nEMAIL;TYPE=work:sophie@corp.com\r\nTEL;VALUE=uri:tel:+33698765432\r\nORG:MiyuCorp\r\nUID:v4-001@test\r\nEND:VCARD";
    let ct = svc.create_contact(&book_id, vcard).unwrap();
    assert_eq!(ct.uid, "v4-001@test");

    let fetched = svc
        .get_contact(&book_id, "v4-001@test")
        .unwrap()
        .unwrap();
    assert!(fetched.vcard_data.contains("VERSION:4.0"));
    assert!(fetched.vcard_data.contains("Sophie Durand"));
    assert!(fetched.vcard_data.contains("MiyuCorp"));
}
