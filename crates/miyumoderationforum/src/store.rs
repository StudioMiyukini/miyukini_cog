//! Store en mémoire MiyuModerationForum. Décision ALLOW/DENY = StrongFather (hors toolkit).

use std::collections::HashMap;
use std::sync::Mutex;

/// id -> (kind, payload)
pub(crate) fn queue_items() -> &'static Mutex<HashMap<String, (String, String)>> {
    static Q: std::sync::OnceLock<Mutex<HashMap<String, (String, String)>>> =
        std::sync::OnceLock::new();
    Q.get_or_init(|| Mutex::new(HashMap::new()))
}

/// post_id -> (content, locked)
pub(crate) fn posts() -> &'static Mutex<HashMap<String, (String, bool)>> {
    static P: std::sync::OnceLock<Mutex<HashMap<String, (String, bool)>>> =
        std::sync::OnceLock::new();
    P.get_or_init(|| Mutex::new(HashMap::new()))
}

/// topic_id -> (board_id, locked)
pub(crate) fn topics() -> &'static Mutex<HashMap<String, (String, bool)>> {
    static T: std::sync::OnceLock<Mutex<HashMap<String, (String, bool)>>> =
        std::sync::OnceLock::new();
    T.get_or_init(|| Mutex::new(HashMap::new()))
}

/// report_id -> (target_type, target_id, reason)
pub(crate) fn reports() -> &'static Mutex<HashMap<String, (String, String, String)>> {
    static R: std::sync::OnceLock<Mutex<HashMap<String, (String, String, String)>>> =
        std::sync::OnceLock::new();
    R.get_or_init(|| Mutex::new(HashMap::new()))
}

/// warning_id -> (user_id, reason)
pub(crate) fn warnings() -> &'static Mutex<HashMap<String, (String, String)>> {
    static W: std::sync::OnceLock<Mutex<HashMap<String, (String, String)>>> =
        std::sync::OnceLock::new();
    W.get_or_init(|| Mutex::new(HashMap::new()))
}

/// usernote_id -> (user_id, content)
pub(crate) fn usernotes() -> &'static Mutex<HashMap<String, (String, String)>> {
    static U: std::sync::OnceLock<Mutex<HashMap<String, (String, String)>>> =
        std::sync::OnceLock::new();
    U.get_or_init(|| Mutex::new(HashMap::new()))
}

/// user_id -> Vec<usernote_id>
pub(crate) fn usernotes_by_user() -> &'static Mutex<HashMap<String, Vec<String>>> {
    static UB: std::sync::OnceLock<Mutex<HashMap<String, Vec<String>>>> =
        std::sync::OnceLock::new();
    UB.get_or_init(|| Mutex::new(HashMap::new()))
}

/// ban_id -> (user_id, reason, until)
pub(crate) fn bans() -> &'static Mutex<HashMap<String, (String, String, Option<String>)>> {
    static B: std::sync::OnceLock<Mutex<HashMap<String, (String, String, Option<String>)>>> =
        std::sync::OnceLock::new();
    B.get_or_init(|| Mutex::new(HashMap::new()))
}
