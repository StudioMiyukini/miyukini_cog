//! Store en mémoire MiyuWidgets (templates, layouts).

use std::collections::HashMap;
use std::sync::Mutex;

/// template_id -> content
pub(crate) fn templates() -> &'static Mutex<HashMap<String, String>> {
    static T: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();
    T.get_or_init(|| Mutex::new(HashMap::new()))
}

/// layout_id -> payload
pub(crate) fn layouts() -> &'static Mutex<HashMap<String, String>> {
    static L: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();
    L.get_or_init(|| Mutex::new(HashMap::new()))
}
