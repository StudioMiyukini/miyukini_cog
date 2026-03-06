use rusqlite::Connection;

pub fn create_caldav_tables(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS cloud_calendars (
            id TEXT PRIMARY KEY NOT NULL,
            owner_id TEXT NOT NULL,
            display_name TEXT NOT NULL,
            description TEXT,
            color TEXT,
            timezone TEXT,
            ctag TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
            updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
        );

        CREATE INDEX IF NOT EXISTS idx_calendars_owner ON cloud_calendars(owner_id);

        CREATE TABLE IF NOT EXISTS cloud_calendar_events (
            id TEXT PRIMARY KEY NOT NULL,
            calendar_id TEXT NOT NULL REFERENCES cloud_calendars(id) ON DELETE CASCADE,
            uid TEXT NOT NULL,
            ical_data TEXT NOT NULL,
            etag TEXT NOT NULL,
            summary TEXT,
            dtstart TEXT,
            dtend TEXT,
            created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
            updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
            UNIQUE(calendar_id, uid)
        );

        CREATE INDEX IF NOT EXISTS idx_events_calendar ON cloud_calendar_events(calendar_id);
        CREATE INDEX IF NOT EXISTS idx_events_uid ON cloud_calendar_events(uid);
        CREATE INDEX IF NOT EXISTS idx_events_dtstart ON cloud_calendar_events(dtstart);
        ",
    )
}
