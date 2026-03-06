use rusqlite::Connection;

pub fn create_carddav_tables(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS cloud_addressbooks (
            id TEXT PRIMARY KEY NOT NULL,
            owner_id TEXT NOT NULL,
            display_name TEXT NOT NULL,
            description TEXT,
            ctag TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
            updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
        );

        CREATE INDEX IF NOT EXISTS idx_addressbooks_owner ON cloud_addressbooks(owner_id);

        CREATE TABLE IF NOT EXISTS cloud_contacts (
            id TEXT PRIMARY KEY NOT NULL,
            addressbook_id TEXT NOT NULL REFERENCES cloud_addressbooks(id) ON DELETE CASCADE,
            uid TEXT NOT NULL,
            vcard_data TEXT NOT NULL,
            etag TEXT NOT NULL,
            full_name TEXT,
            email TEXT,
            created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
            updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
            UNIQUE(addressbook_id, uid)
        );

        CREATE INDEX IF NOT EXISTS idx_contacts_addressbook ON cloud_contacts(addressbook_id);
        CREATE INDEX IF NOT EXISTS idx_contacts_uid ON cloud_contacts(uid);
        CREATE INDEX IF NOT EXISTS idx_contacts_fullname ON cloud_contacts(full_name);
        ",
    )
}
