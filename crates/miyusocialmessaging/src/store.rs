//! Store en mémoire MiyuSocialMessaging (conversations, messages).

use std::collections::HashMap;
use std::sync::Mutex;

/// conversation_id -> participant_ids
pub(crate) fn conversations() -> &'static Mutex<HashMap<String, Vec<String>>> {
    static C: std::sync::OnceLock<Mutex<HashMap<String, Vec<String>>>> = std::sync::OnceLock::new();
    C.get_or_init(|| Mutex::new(HashMap::new()))
}

/// conversation_id -> Vec<message_id>
pub(crate) fn conversation_messages() -> &'static Mutex<HashMap<String, Vec<String>>> {
    static M: std::sync::OnceLock<Mutex<HashMap<String, Vec<String>>>> = std::sync::OnceLock::new();
    M.get_or_init(|| Mutex::new(HashMap::new()))
}

/// message_id -> (conversation_id, sender_id, content)
pub(crate) fn messages() -> &'static Mutex<HashMap<String, (String, String, String)>> {
    static MS: std::sync::OnceLock<Mutex<HashMap<String, (String, String, String)>>> = std::sync::OnceLock::new();
    MS.get_or_init(|| Mutex::new(HashMap::new()))
}
