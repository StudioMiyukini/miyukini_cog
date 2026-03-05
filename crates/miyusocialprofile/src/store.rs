//! Store en mémoire MiyuSocialProfile (profils, follow).

use std::collections::HashMap;
use std::sync::Mutex;

/// user_id -> (key -> value)
pub(crate) fn profiles() -> &'static Mutex<HashMap<String, HashMap<String, String>>> {
    static P: std::sync::OnceLock<Mutex<HashMap<String, HashMap<String, String>>>> =
        std::sync::OnceLock::new();
    P.get_or_init(|| Mutex::new(HashMap::new()))
}

/// user_id -> Vec<target_id> (following)
pub(crate) fn following() -> &'static Mutex<HashMap<String, Vec<String>>> {
    static F: std::sync::OnceLock<Mutex<HashMap<String, Vec<String>>>> = std::sync::OnceLock::new();
    F.get_or_init(|| Mutex::new(HashMap::new()))
}

/// user_id -> Vec<follower_id>
pub(crate) fn followers() -> &'static Mutex<HashMap<String, Vec<String>>> {
    static FW: std::sync::OnceLock<Mutex<HashMap<String, Vec<String>>>> =
        std::sync::OnceLock::new();
    FW.get_or_init(|| Mutex::new(HashMap::new()))
}
