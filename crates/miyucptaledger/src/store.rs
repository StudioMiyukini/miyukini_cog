//! Store en mémoire MiyuComptaLedger. Persistance réelle = KindMother.

use std::collections::HashMap;
use std::sync::Mutex;

/// structure_id -> payload
pub(crate) fn structures() -> &'static Mutex<HashMap<String, String>> {
    static S: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();
    S.get_or_init(|| Mutex::new(HashMap::new()))
}

/// siret -> json info (cache)
pub(crate) fn siret_cache() -> &'static Mutex<HashMap<String, String>> {
    static C: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();
    C.get_or_init(|| Mutex::new(HashMap::new()))
}
