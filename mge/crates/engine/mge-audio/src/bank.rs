//! Sound asset registry -- maps logical ids to filesystem paths.

use std::collections::HashMap;

/// Registry that maps sound identifiers to their file paths.
///
/// `SoundBank` is intentionally hardware-free so it can be tested
/// without an audio backend.
#[derive(Debug, Clone, Default)]
pub struct SoundBank {
    entries: HashMap<String, String>,
}

impl SoundBank {
    /// Creates an empty sound bank.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a mapping from `id` to `path`.
    ///
    /// If the id already exists the path is overwritten silently.
    pub fn register(&mut self, id: &str, path: &str) {
        self.entries.insert(id.to_owned(), path.to_owned());
    }

    /// Returns the path associated with `id`, if any.
    pub fn get_path(&self, id: &str) -> Option<&str> {
        self.entries.get(id).map(String::as_str)
    }

    /// Number of registered entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` when the bank contains no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
