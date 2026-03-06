use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar {
    pub id: Uuid,
    pub owner_id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub timezone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub ctag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: Uuid,
    pub calendar_id: Uuid,
    pub uid: String,
    pub ical_data: String,
    pub etag: String,
    pub summary: Option<String>,
    pub dtstart: Option<DateTime<Utc>>,
    pub dtend: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Calendar {
    pub fn new(owner_id: &str, display_name: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            owner_id: owner_id.to_string(),
            display_name: display_name.to_string(),
            description: None,
            color: None,
            timezone: None,
            created_at: now,
            updated_at: now,
            ctag: Uuid::new_v4().to_string(),
        }
    }
}

impl CalendarEvent {
    pub fn new(calendar_id: Uuid, uid: &str, ical_data: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            calendar_id,
            uid: uid.to_string(),
            ical_data: ical_data.to_string(),
            etag: format!("{:x}", md5_hash(ical_data.as_bytes())),
            summary: None,
            dtstart: None,
            dtend: None,
            created_at: now,
            updated_at: now,
        }
    }
}

fn md5_hash(data: &[u8]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}
