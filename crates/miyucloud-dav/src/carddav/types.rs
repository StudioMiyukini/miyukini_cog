use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBook {
    pub id: Uuid,
    pub owner_id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub ctag: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub addressbook_id: Uuid,
    pub uid: String,
    pub vcard_data: String,
    pub etag: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AddressBook {
    pub fn new(owner_id: &str, display_name: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            owner_id: owner_id.to_string(),
            display_name: display_name.to_string(),
            description: None,
            ctag: Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl Contact {
    pub fn new(addressbook_id: Uuid, uid: &str, vcard_data: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            addressbook_id,
            uid: uid.to_string(),
            vcard_data: vcard_data.to_string(),
            etag: format!("{:x}", {
                use std::hash::{Hash, Hasher};
                let mut h = std::collections::hash_map::DefaultHasher::new();
                vcard_data.hash(&mut h);
                h.finish()
            }),
            full_name: None,
            email: None,
            created_at: now,
            updated_at: now,
        }
    }
}
