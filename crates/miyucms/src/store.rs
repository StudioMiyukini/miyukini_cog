//! Store en mémoire MiyuCMS (révisions, médias, commentaires). Persistance réelle = KindMother.

use std::collections::HashMap;
use std::sync::Mutex;

/// revision_id -> (content_id, payload)
static REVISIONS: std::sync::OnceLock<Mutex<HashMap<String, (String, String)>>> =
    std::sync::OnceLock::new();
/// content_id -> Vec<revision_id> (ordre)
static CONTENT_REVISIONS: std::sync::OnceLock<Mutex<HashMap<String, Vec<String>>>> =
    std::sync::OnceLock::new();

/// media_id -> (payload_meta, blob)
static MEDIA: std::sync::OnceLock<Mutex<HashMap<String, (String, Vec<u8>)>>> =
    std::sync::OnceLock::new();

/// comment_id -> (content_id, payload)
static COMMENTS: std::sync::OnceLock<Mutex<HashMap<String, (String, String)>>> =
    std::sync::OnceLock::new();
/// content_id -> Vec<comment_id>
static CONTENT_COMMENTS: std::sync::OnceLock<Mutex<HashMap<String, Vec<String>>>> =
    std::sync::OnceLock::new();

pub(crate) fn revisions() -> &'static Mutex<HashMap<String, (String, String)>> {
    REVISIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) fn content_revisions() -> &'static Mutex<HashMap<String, Vec<String>>> {
    CONTENT_REVISIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) fn media() -> &'static Mutex<HashMap<String, (String, Vec<u8>)>> {
    MEDIA.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) fn comments() -> &'static Mutex<HashMap<String, (String, String)>> {
    COMMENTS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) fn content_comments() -> &'static Mutex<HashMap<String, Vec<String>>> {
    CONTENT_COMMENTS.get_or_init(|| Mutex::new(HashMap::new()))
}
