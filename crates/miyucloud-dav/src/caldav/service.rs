use rusqlite::params;
use std::sync::{Arc, Mutex};

use crate::caldav::ical;
use crate::caldav::types::{Calendar, CalendarEvent};
use crate::common::dav_error::DavError;

/// CalDAV service for calendar and event CRUD operations.
/// Backed by SQLite via rusqlite directly (CalDAV tables are DAV-specific).
#[derive(Clone)]
pub struct CalDavService {
    db: Arc<Mutex<rusqlite::Connection>>,
}

impl CalDavService {
    pub fn new(db: Arc<Mutex<rusqlite::Connection>>) -> Self {
        Self { db }
    }

    fn conn(&self) -> Result<std::sync::MutexGuard<'_, rusqlite::Connection>, DavError> {
        self.db
            .lock()
            .map_err(|e| DavError::Internal(format!("DB lock poisoned: {e}")))
    }

    /// Initialize CalDAV tables if they don't exist.
    pub fn init_tables(&self) -> Result<(), DavError> {
        let conn = self.conn()?;
        crate::caldav::schema::create_caldav_tables(&conn)?;
        Ok(())
    }

    // -- Calendar CRUD --

    pub fn create_calendar(&self, owner_id: &str, display_name: &str) -> Result<Calendar, DavError> {
        let cal = Calendar::new(owner_id, display_name);
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO cloud_calendars (id, owner_id, display_name, ctag) VALUES (?1, ?2, ?3, ?4)",
            params![cal.id.to_string(), cal.owner_id, cal.display_name, cal.ctag],
        )?;
        Ok(cal)
    }

    pub fn list_calendars(&self, owner_id: &str) -> Result<Vec<Calendar>, DavError> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, owner_id, display_name, description, color, timezone, ctag, created_at, updated_at
             FROM cloud_calendars WHERE owner_id = ?1"
        )?;
        let rows = stmt.query_map(params![owner_id], |row| {
            Ok(row_to_calendar(row))
        })?;
        let mut calendars = Vec::new();
        for row in rows {
            calendars.push(row??);
        }
        Ok(calendars)
    }

    pub fn get_calendar(&self, calendar_id: &str) -> Result<Option<Calendar>, DavError> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, owner_id, display_name, description, color, timezone, ctag, created_at, updated_at
             FROM cloud_calendars WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map(params![calendar_id], |row| {
            Ok(row_to_calendar(row))
        })?;
        match rows.next() {
            Some(row) => Ok(Some(row??)),
            None => Ok(None),
        }
    }

    pub fn get_calendar_by_owner_and_name(
        &self,
        owner_id: &str,
        display_name: &str,
    ) -> Result<Option<Calendar>, DavError> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, owner_id, display_name, description, color, timezone, ctag, created_at, updated_at
             FROM cloud_calendars WHERE owner_id = ?1 AND display_name = ?2"
        )?;
        let mut rows = stmt.query_map(params![owner_id, display_name], |row| {
            Ok(row_to_calendar(row))
        })?;
        match rows.next() {
            Some(row) => Ok(Some(row??)),
            None => Ok(None),
        }
    }

    pub fn delete_calendar(&self, calendar_id: &str) -> Result<(), DavError> {
        let conn = self.conn()?;
        conn.execute("DELETE FROM cloud_calendars WHERE id = ?1", params![calendar_id])?;
        Ok(())
    }

    fn bump_ctag_with_conn(&self, conn: &rusqlite::Connection, calendar_id: &str) -> Result<(), DavError> {
        let new_ctag = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "UPDATE cloud_calendars SET ctag = ?1, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?2",
            params![new_ctag, calendar_id],
        )?;
        Ok(())
    }

    // -- Event CRUD --

    pub fn create_event(
        &self,
        calendar_id: &str,
        ical_data: &str,
    ) -> Result<CalendarEvent, DavError> {
        ical::validate_ical(ical_data)?;
        let uid = ical::extract_vevent_uid(ical_data)?;
        let summary = ical::extract_vevent_summary(ical_data);
        let dtstart = ical::extract_vevent_dtstart(ical_data);
        let dtend = ical::extract_vevent_dtend(ical_data);

        let cal_uuid: uuid::Uuid = calendar_id
            .parse()
            .map_err(|_| DavError::BadRequest("Invalid calendar ID".into()))?;
        let event = CalendarEvent::new(cal_uuid, &uid, ical_data);

        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO cloud_calendar_events (id, calendar_id, uid, ical_data, etag, summary, dtstart, dtend)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                event.id.to_string(),
                calendar_id,
                event.uid,
                event.ical_data,
                event.etag,
                summary,
                dtstart,
                dtend,
            ],
        )?;
        self.bump_ctag_with_conn(&conn, calendar_id)?;
        Ok(event)
    }

    pub fn list_events(&self, calendar_id: &str) -> Result<Vec<CalendarEvent>, DavError> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, calendar_id, uid, ical_data, etag, summary, dtstart, dtend, created_at, updated_at
             FROM cloud_calendar_events WHERE calendar_id = ?1"
        )?;
        let rows = stmt.query_map(params![calendar_id], |row| {
            Ok(row_to_event(row))
        })?;
        let mut events = Vec::new();
        for row in rows {
            events.push(row??);
        }
        Ok(events)
    }

    pub fn get_event(&self, calendar_id: &str, uid: &str) -> Result<Option<CalendarEvent>, DavError> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, calendar_id, uid, ical_data, etag, summary, dtstart, dtend, created_at, updated_at
             FROM cloud_calendar_events WHERE calendar_id = ?1 AND uid = ?2"
        )?;
        let mut rows = stmt.query_map(params![calendar_id, uid], |row| {
            Ok(row_to_event(row))
        })?;
        match rows.next() {
            Some(row) => Ok(Some(row??)),
            None => Ok(None),
        }
    }

    pub fn get_events_by_uids(
        &self,
        calendar_id: &str,
        uids: &[String],
    ) -> Result<Vec<CalendarEvent>, DavError> {
        if uids.is_empty() {
            return Ok(vec![]);
        }
        let placeholders: Vec<String> = (0..uids.len()).map(|i| format!("?{}", i + 2)).collect();
        let sql = format!(
            "SELECT id, calendar_id, uid, ical_data, etag, summary, dtstart, dtend, created_at, updated_at
             FROM cloud_calendar_events WHERE calendar_id = ?1 AND uid IN ({})",
            placeholders.join(", ")
        );
        let conn = self.conn()?;
        let mut stmt = conn.prepare(&sql)?;
        let mut params_vec: Vec<Box<dyn rusqlite::types::ToSql>> = vec![Box::new(calendar_id.to_string())];
        for uid in uids {
            params_vec.push(Box::new(uid.clone()));
        }
        let params_refs: Vec<&dyn rusqlite::types::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        let rows = stmt.query_map(&*params_refs, |row| {
            Ok(row_to_event(row))
        })?;
        let mut events = Vec::new();
        for row in rows {
            events.push(row??);
        }
        Ok(events)
    }

    pub fn update_event(
        &self,
        calendar_id: &str,
        uid: &str,
        ical_data: &str,
    ) -> Result<CalendarEvent, DavError> {
        ical::validate_ical(ical_data)?;
        let summary = ical::extract_vevent_summary(ical_data);
        let dtstart = ical::extract_vevent_dtstart(ical_data);
        let dtend = ical::extract_vevent_dtend(ical_data);
        let etag = format!("{:x}", hash_bytes(ical_data.as_bytes()));

        let conn = self.conn()?;
        let affected = conn.execute(
            "UPDATE cloud_calendar_events
             SET ical_data = ?1, etag = ?2, summary = ?3, dtstart = ?4, dtend = ?5,
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE calendar_id = ?6 AND uid = ?7",
            params![ical_data, etag, summary, dtstart, dtend, calendar_id, uid],
        )?;

        if affected == 0 {
            return Err(DavError::NotFound(format!("Event {uid} not found")));
        }
        self.bump_ctag_with_conn(&conn, calendar_id)?;
        drop(conn);

        self.get_event(calendar_id, uid)?
            .ok_or_else(|| DavError::Internal("Event vanished after update".into()))
    }

    pub fn delete_event(&self, calendar_id: &str, uid: &str) -> Result<(), DavError> {
        let conn = self.conn()?;
        conn.execute(
            "DELETE FROM cloud_calendar_events WHERE calendar_id = ?1 AND uid = ?2",
            params![calendar_id, uid],
        )?;
        self.bump_ctag_with_conn(&conn, calendar_id)?;
        Ok(())
    }
}

fn hash_bytes(data: &[u8]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

fn row_to_calendar(row: &rusqlite::Row) -> Result<Calendar, DavError> {
    Ok(Calendar {
        id: row.get::<_, String>(0)?
            .parse()
            .map_err(|_| DavError::Internal("Invalid UUID in DB".into()))?,
        owner_id: row.get(1)?,
        display_name: row.get(2)?,
        description: row.get(3)?,
        color: row.get(4)?,
        timezone: row.get(5)?,
        ctag: row.get(6)?,
        created_at: row.get::<_, String>(7)?
            .parse()
            .unwrap_or_default(),
        updated_at: row.get::<_, String>(8)?
            .parse()
            .unwrap_or_default(),
    })
}

fn row_to_event(row: &rusqlite::Row) -> Result<CalendarEvent, DavError> {
    Ok(CalendarEvent {
        id: row.get::<_, String>(0)?
            .parse()
            .map_err(|_| DavError::Internal("Invalid UUID in DB".into()))?,
        calendar_id: row.get::<_, String>(1)?
            .parse()
            .map_err(|_| DavError::Internal("Invalid UUID in DB".into()))?,
        uid: row.get(2)?,
        ical_data: row.get(3)?,
        etag: row.get(4)?,
        summary: row.get(5)?,
        dtstart: row.get::<_, Option<String>>(6)?
            .and_then(|s| s.parse().ok()),
        dtend: row.get::<_, Option<String>>(7)?
            .and_then(|s| s.parse().ok()),
        created_at: row.get::<_, String>(8)?
            .parse()
            .unwrap_or_default(),
        updated_at: row.get::<_, String>(9)?
            .parse()
            .unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_service() -> CalDavService {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        let svc = CalDavService::new(Arc::new(Mutex::new(conn)));
        svc.init_tables().unwrap();
        svc
    }

    const SAMPLE_ICAL: &str = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nBEGIN:VEVENT\r\nUID:evt-001@miyucloud\r\nSUMMARY:Standup\r\nDTSTART:20260310T090000Z\r\nDTEND:20260310T091500Z\r\nEND:VEVENT\r\nEND:VCALENDAR";

    #[test]
    fn test_create_and_list_calendars() {
        let svc = setup_service();
        let cal = svc.create_calendar("user1", "Work").unwrap();
        assert_eq!(cal.display_name, "Work");
        assert_eq!(cal.owner_id, "user1");

        let cals = svc.list_calendars("user1").unwrap();
        assert_eq!(cals.len(), 1);
        assert_eq!(cals[0].display_name, "Work");
    }

    #[test]
    fn test_get_calendar() {
        let svc = setup_service();
        let cal = svc.create_calendar("user1", "Personal").unwrap();
        let fetched = svc.get_calendar(&cal.id.to_string()).unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().display_name, "Personal");
    }

    #[test]
    fn test_delete_calendar() {
        let svc = setup_service();
        let cal = svc.create_calendar("user1", "Temp").unwrap();
        svc.delete_calendar(&cal.id.to_string()).unwrap();
        let fetched = svc.get_calendar(&cal.id.to_string()).unwrap();
        assert!(fetched.is_none());
    }

    #[test]
    fn test_create_and_list_events() {
        let svc = setup_service();
        let cal = svc.create_calendar("user1", "Work").unwrap();
        let event = svc.create_event(&cal.id.to_string(), SAMPLE_ICAL).unwrap();
        assert_eq!(event.uid, "evt-001@miyucloud");

        let events = svc.list_events(&cal.id.to_string()).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].uid, "evt-001@miyucloud");
    }

    #[test]
    fn test_get_event_by_uid() {
        let svc = setup_service();
        let cal = svc.create_calendar("user1", "Work").unwrap();
        svc.create_event(&cal.id.to_string(), SAMPLE_ICAL).unwrap();

        let event = svc.get_event(&cal.id.to_string(), "evt-001@miyucloud").unwrap();
        assert!(event.is_some());
    }

    #[test]
    fn test_update_event() {
        let svc = setup_service();
        let cal = svc.create_calendar("user1", "Work").unwrap();
        svc.create_event(&cal.id.to_string(), SAMPLE_ICAL).unwrap();

        let updated_ical = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nBEGIN:VEVENT\r\nUID:evt-001@miyucloud\r\nSUMMARY:Updated Standup\r\nDTSTART:20260310T100000Z\r\nDTEND:20260310T101500Z\r\nEND:VEVENT\r\nEND:VCALENDAR";
        let updated = svc.update_event(&cal.id.to_string(), "evt-001@miyucloud", updated_ical).unwrap();
        assert_eq!(updated.ical_data, updated_ical);
    }

    #[test]
    fn test_delete_event() {
        let svc = setup_service();
        let cal = svc.create_calendar("user1", "Work").unwrap();
        svc.create_event(&cal.id.to_string(), SAMPLE_ICAL).unwrap();
        svc.delete_event(&cal.id.to_string(), "evt-001@miyucloud").unwrap();

        let event = svc.get_event(&cal.id.to_string(), "evt-001@miyucloud").unwrap();
        assert!(event.is_none());
    }

    #[test]
    fn test_cascade_delete() {
        let svc = setup_service();
        let cal = svc.create_calendar("user1", "Work").unwrap();
        svc.create_event(&cal.id.to_string(), SAMPLE_ICAL).unwrap();
        svc.delete_calendar(&cal.id.to_string()).unwrap();

        let events = svc.list_events(&cal.id.to_string()).unwrap();
        assert!(events.is_empty());
    }
}
