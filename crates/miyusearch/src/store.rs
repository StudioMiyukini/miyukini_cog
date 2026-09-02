//! @id     miyusearch_store
//! @do     definir_index
//! @role   donnee
//! @layer  core
//! @human  miyusearch — module « store ».

//! Store en mémoire pour l'index MiyuSearch (dérivation ; persistance réelle = KindMother).
//! Partagé par index.update, query.execute, suggest.

use std::collections::HashMap;
use std::sync::Mutex;

static INDEX: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();

pub(crate) fn index() -> &'static Mutex<HashMap<String, String>> {
    INDEX.get_or_init(|| Mutex::new(HashMap::new()))
}
