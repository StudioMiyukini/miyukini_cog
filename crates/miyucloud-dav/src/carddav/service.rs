use rusqlite::params;
use std::sync::{Arc, Mutex};

use crate::carddav::vcard;
use crate::carddav::types::{AddressBook, Contact};
use crate::common::dav_error::DavError;

/// CardDAV service for address book and contact CRUD operations.
#[derive(Clone)]
pub struct CardDavService {
    db: Arc<Mutex<rusqlite::Connection>>,
}

impl CardDavService {
    pub fn new(db: Arc<Mutex<rusqlite::Connection>>) -> Self {
        Self { db }
    }

    fn conn(&self) -> Result<std::sync::MutexGuard<'_, rusqlite::Connection>, DavError> {
        self.db
            .lock()
            .map_err(|e| DavError::Internal(format!("DB lock poisoned: {e}")))
    }

    pub fn init_tables(&self) -> Result<(), DavError> {
        let conn = self.conn()?;
        crate::carddav::schema::create_carddav_tables(&conn)?;
        Ok(())
    }

    // -- AddressBook CRUD --

    pub fn create_addressbook(&self, owner_id: &str, display_name: &str) -> Result<AddressBook, DavError> {
        let ab = AddressBook::new(owner_id, display_name);
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO cloud_addressbooks (id, owner_id, display_name, ctag) VALUES (?1, ?2, ?3, ?4)",
            params![ab.id.to_string(), ab.owner_id, ab.display_name, ab.ctag],
        )?;
        Ok(ab)
    }

    pub fn list_addressbooks(&self, owner_id: &str) -> Result<Vec<AddressBook>, DavError> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, owner_id, display_name, description, ctag, created_at, updated_at
             FROM cloud_addressbooks WHERE owner_id = ?1"
        )?;
        let rows = stmt.query_map(params![owner_id], |row| Ok(row_to_addressbook(row)))?;
        let mut books = Vec::new();
        for row in rows {
            books.push(row??);
        }
        Ok(books)
    }

    pub fn get_addressbook(&self, addressbook_id: &str) -> Result<Option<AddressBook>, DavError> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, owner_id, display_name, description, ctag, created_at, updated_at
             FROM cloud_addressbooks WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map(params![addressbook_id], |row| Ok(row_to_addressbook(row)))?;
        match rows.next() {
            Some(row) => Ok(Some(row??)),
            None => Ok(None),
        }
    }

    pub fn get_addressbook_by_owner_and_name(
        &self,
        owner_id: &str,
        display_name: &str,
    ) -> Result<Option<AddressBook>, DavError> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, owner_id, display_name, description, ctag, created_at, updated_at
             FROM cloud_addressbooks WHERE owner_id = ?1 AND display_name = ?2"
        )?;
        let mut rows = stmt.query_map(params![owner_id, display_name], |row| Ok(row_to_addressbook(row)))?;
        match rows.next() {
            Some(row) => Ok(Some(row??)),
            None => Ok(None),
        }
    }

    pub fn delete_addressbook(&self, addressbook_id: &str) -> Result<(), DavError> {
        let conn = self.conn()?;
        conn.execute("DELETE FROM cloud_addressbooks WHERE id = ?1", params![addressbook_id])?;
        Ok(())
    }

    fn bump_ctag_with_conn(&self, conn: &rusqlite::Connection, addressbook_id: &str) -> Result<(), DavError> {
        let new_ctag = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "UPDATE cloud_addressbooks SET ctag = ?1, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?2",
            params![new_ctag, addressbook_id],
        )?;
        Ok(())
    }

    // -- Contact CRUD --

    pub fn create_contact(&self, addressbook_id: &str, vcard_data: &str) -> Result<Contact, DavError> {
        vcard::validate_vcard(vcard_data)?;
        let uid = vcard::extract_vcard_uid(vcard_data)?;
        let full_name = vcard::extract_vcard_fn(vcard_data);
        let email = vcard::extract_vcard_email(vcard_data);

        let ab_uuid: uuid::Uuid = addressbook_id
            .parse()
            .map_err(|_| DavError::BadRequest("Invalid addressbook ID".into()))?;
        let contact = Contact::new(ab_uuid, &uid, vcard_data);

        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO cloud_contacts (id, addressbook_id, uid, vcard_data, etag, full_name, email)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                contact.id.to_string(),
                addressbook_id,
                contact.uid,
                contact.vcard_data,
                contact.etag,
                full_name,
                email,
            ],
        )?;
        self.bump_ctag_with_conn(&conn, addressbook_id)?;
        Ok(contact)
    }

    pub fn list_contacts(&self, addressbook_id: &str) -> Result<Vec<Contact>, DavError> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, addressbook_id, uid, vcard_data, etag, full_name, email, created_at, updated_at
             FROM cloud_contacts WHERE addressbook_id = ?1"
        )?;
        let rows = stmt.query_map(params![addressbook_id], |row| Ok(row_to_contact(row)))?;
        let mut contacts = Vec::new();
        for row in rows {
            contacts.push(row??);
        }
        Ok(contacts)
    }

    pub fn get_contact(&self, addressbook_id: &str, uid: &str) -> Result<Option<Contact>, DavError> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, addressbook_id, uid, vcard_data, etag, full_name, email, created_at, updated_at
             FROM cloud_contacts WHERE addressbook_id = ?1 AND uid = ?2"
        )?;
        let mut rows = stmt.query_map(params![addressbook_id, uid], |row| Ok(row_to_contact(row)))?;
        match rows.next() {
            Some(row) => Ok(Some(row??)),
            None => Ok(None),
        }
    }

    pub fn get_contacts_by_uids(
        &self,
        addressbook_id: &str,
        uids: &[String],
    ) -> Result<Vec<Contact>, DavError> {
        if uids.is_empty() {
            return Ok(vec![]);
        }
        let placeholders: Vec<String> = (0..uids.len()).map(|i| format!("?{}", i + 2)).collect();
        let sql = format!(
            "SELECT id, addressbook_id, uid, vcard_data, etag, full_name, email, created_at, updated_at
             FROM cloud_contacts WHERE addressbook_id = ?1 AND uid IN ({})",
            placeholders.join(", ")
        );
        let conn = self.conn()?;
        let mut stmt = conn.prepare(&sql)?;
        let mut params_vec: Vec<Box<dyn rusqlite::types::ToSql>> = vec![Box::new(addressbook_id.to_string())];
        for uid in uids {
            params_vec.push(Box::new(uid.clone()));
        }
        let params_refs: Vec<&dyn rusqlite::types::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        let rows = stmt.query_map(&*params_refs, |row| Ok(row_to_contact(row)))?;
        let mut contacts = Vec::new();
        for row in rows {
            contacts.push(row??);
        }
        Ok(contacts)
    }

    pub fn update_contact(
        &self,
        addressbook_id: &str,
        uid: &str,
        vcard_data: &str,
    ) -> Result<Contact, DavError> {
        vcard::validate_vcard(vcard_data)?;
        let full_name = vcard::extract_vcard_fn(vcard_data);
        let email = vcard::extract_vcard_email(vcard_data);
        let etag = format!("{:x}", hash_bytes(vcard_data.as_bytes()));

        let conn = self.conn()?;
        let affected = conn.execute(
            "UPDATE cloud_contacts
             SET vcard_data = ?1, etag = ?2, full_name = ?3, email = ?4,
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE addressbook_id = ?5 AND uid = ?6",
            params![vcard_data, etag, full_name, email, addressbook_id, uid],
        )?;

        if affected == 0 {
            return Err(DavError::NotFound(format!("Contact {uid} not found")));
        }
        self.bump_ctag_with_conn(&conn, addressbook_id)?;
        drop(conn);

        self.get_contact(addressbook_id, uid)?
            .ok_or_else(|| DavError::Internal("Contact vanished after update".into()))
    }

    pub fn delete_contact(&self, addressbook_id: &str, uid: &str) -> Result<(), DavError> {
        let conn = self.conn()?;
        conn.execute(
            "DELETE FROM cloud_contacts WHERE addressbook_id = ?1 AND uid = ?2",
            params![addressbook_id, uid],
        )?;
        self.bump_ctag_with_conn(&conn, addressbook_id)?;
        Ok(())
    }
}

fn hash_bytes(data: &[u8]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

fn row_to_addressbook(row: &rusqlite::Row) -> Result<AddressBook, DavError> {
    Ok(AddressBook {
        id: row.get::<_, String>(0)?
            .parse()
            .map_err(|_| DavError::Internal("Invalid UUID in DB".into()))?,
        owner_id: row.get(1)?,
        display_name: row.get(2)?,
        description: row.get(3)?,
        ctag: row.get(4)?,
        created_at: row.get::<_, String>(5)?.parse().unwrap_or_default(),
        updated_at: row.get::<_, String>(6)?.parse().unwrap_or_default(),
    })
}

fn row_to_contact(row: &rusqlite::Row) -> Result<Contact, DavError> {
    Ok(Contact {
        id: row.get::<_, String>(0)?
            .parse()
            .map_err(|_| DavError::Internal("Invalid UUID in DB".into()))?,
        addressbook_id: row.get::<_, String>(1)?
            .parse()
            .map_err(|_| DavError::Internal("Invalid UUID in DB".into()))?,
        uid: row.get(2)?,
        vcard_data: row.get(3)?,
        etag: row.get(4)?,
        full_name: row.get(5)?,
        email: row.get(6)?,
        created_at: row.get::<_, String>(7)?.parse().unwrap_or_default(),
        updated_at: row.get::<_, String>(8)?.parse().unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_service() -> CardDavService {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        let svc = CardDavService::new(Arc::new(Mutex::new(conn)));
        svc.init_tables().unwrap();
        svc
    }

    const SAMPLE_VCARD: &str = "BEGIN:VCARD\r\nVERSION:3.0\r\nUID:contact-001@miyucloud\r\nFN:Marie Curie\r\nEMAIL:marie@example.com\r\nEND:VCARD";

    #[test]
    fn test_create_and_list_addressbooks() {
        let svc = setup_service();
        let ab = svc.create_addressbook("user1", "Contacts").unwrap();
        assert_eq!(ab.display_name, "Contacts");

        let books = svc.list_addressbooks("user1").unwrap();
        assert_eq!(books.len(), 1);
        assert_eq!(books[0].display_name, "Contacts");
    }

    #[test]
    fn test_delete_addressbook() {
        let svc = setup_service();
        let ab = svc.create_addressbook("user1", "Temp").unwrap();
        svc.delete_addressbook(&ab.id.to_string()).unwrap();
        let fetched = svc.get_addressbook(&ab.id.to_string()).unwrap();
        assert!(fetched.is_none());
    }

    #[test]
    fn test_create_and_list_contacts() {
        let svc = setup_service();
        let ab = svc.create_addressbook("user1", "Contacts").unwrap();
        let contact = svc.create_contact(&ab.id.to_string(), SAMPLE_VCARD).unwrap();
        assert_eq!(contact.uid, "contact-001@miyucloud");

        let contacts = svc.list_contacts(&ab.id.to_string()).unwrap();
        assert_eq!(contacts.len(), 1);
    }

    #[test]
    fn test_get_contact_by_uid() {
        let svc = setup_service();
        let ab = svc.create_addressbook("user1", "Contacts").unwrap();
        svc.create_contact(&ab.id.to_string(), SAMPLE_VCARD).unwrap();

        let contact = svc.get_contact(&ab.id.to_string(), "contact-001@miyucloud").unwrap();
        assert!(contact.is_some());
    }

    #[test]
    fn test_update_contact() {
        let svc = setup_service();
        let ab = svc.create_addressbook("user1", "Contacts").unwrap();
        svc.create_contact(&ab.id.to_string(), SAMPLE_VCARD).unwrap();

        let updated_vcard = "BEGIN:VCARD\r\nVERSION:3.0\r\nUID:contact-001@miyucloud\r\nFN:Marie Sklodowska-Curie\r\nEMAIL:marie@nobel.org\r\nEND:VCARD";
        let updated = svc.update_contact(&ab.id.to_string(), "contact-001@miyucloud", updated_vcard).unwrap();
        assert_eq!(updated.vcard_data, updated_vcard);
    }

    #[test]
    fn test_delete_contact() {
        let svc = setup_service();
        let ab = svc.create_addressbook("user1", "Contacts").unwrap();
        svc.create_contact(&ab.id.to_string(), SAMPLE_VCARD).unwrap();
        svc.delete_contact(&ab.id.to_string(), "contact-001@miyucloud").unwrap();

        let contact = svc.get_contact(&ab.id.to_string(), "contact-001@miyucloud").unwrap();
        assert!(contact.is_none());
    }

    #[test]
    fn test_cascade_delete() {
        let svc = setup_service();
        let ab = svc.create_addressbook("user1", "Contacts").unwrap();
        svc.create_contact(&ab.id.to_string(), SAMPLE_VCARD).unwrap();
        svc.delete_addressbook(&ab.id.to_string()).unwrap();

        let contacts = svc.list_contacts(&ab.id.to_string()).unwrap();
        assert!(contacts.is_empty());
    }
}
