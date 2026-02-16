//! Store partagé ami/ennemi (en mémoire).

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

pub(super) struct ContactsStore {
    pub friends: HashMap<String, HashSet<String>>,
    pub foes: HashMap<String, HashSet<String>>,
}

impl Default for ContactsStore {
    fn default() -> Self {
        Self {
            friends: HashMap::new(),
            foes: HashMap::new(),
        }
    }
}

pub(super) fn default_store() -> &'static Mutex<ContactsStore> {
    static DEFAULT_STORE: std::sync::OnceLock<Mutex<ContactsStore>> = std::sync::OnceLock::new();
    DEFAULT_STORE.get_or_init(|| Mutex::new(ContactsStore::default()))
}
