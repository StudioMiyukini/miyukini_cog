//! Store en mémoire MiyuStory (id → author_id, content_type, payload, expires_at).

use std::collections::HashMap;
use std::sync::Mutex;

/// story_id -> (author_id, content_type, payload, expires_at)
pub(crate) fn stories() -> &'static Mutex<HashMap<String, (String, String, Vec<u8>, String)>> {
    static S: std::sync::OnceLock<Mutex<HashMap<String, (String, String, Vec<u8>, String)>>> =
        std::sync::OnceLock::new();
    S.get_or_init(|| Mutex::new(HashMap::new()))
}
