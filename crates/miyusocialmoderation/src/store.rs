//! Store en mémoire MiyuSocialModeration (reports, blocks).

use std::collections::HashMap;
use std::sync::Mutex;

/// report_id -> (target_type, target_id)
pub(crate) fn reports() -> &'static Mutex<HashMap<String, (String, String)>> {
    static R: std::sync::OnceLock<Mutex<HashMap<String, (String, String)>>> =
        std::sync::OnceLock::new();
    R.get_or_init(|| Mutex::new(HashMap::new()))
}

/// user_id -> Vec<blocked_id>
pub(crate) fn blocks() -> &'static Mutex<HashMap<String, Vec<String>>> {
    static B: std::sync::OnceLock<Mutex<HashMap<String, Vec<String>>>> = std::sync::OnceLock::new();
    B.get_or_init(|| Mutex::new(HashMap::new()))
}
