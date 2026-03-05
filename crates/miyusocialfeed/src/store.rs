//! Store en mémoire MiyuSocialFeed (posts, comments, reactions, shares).

use std::collections::HashMap;
use std::sync::Mutex;

/// post_id -> (author_id, content)
pub(crate) fn posts() -> &'static Mutex<HashMap<String, (String, String)>> {
    static P: std::sync::OnceLock<Mutex<HashMap<String, (String, String)>>> =
        std::sync::OnceLock::new();
    P.get_or_init(|| Mutex::new(HashMap::new()))
}

/// comment_id -> (post_id, author_id, content)
pub(crate) fn comments() -> &'static Mutex<HashMap<String, (String, String, String)>> {
    static C: std::sync::OnceLock<Mutex<HashMap<String, (String, String, String)>>> =
        std::sync::OnceLock::new();
    C.get_or_init(|| Mutex::new(HashMap::new()))
}

/// post_id -> Vec<(user_id, reaction_type)>
pub(crate) fn reactions() -> &'static Mutex<HashMap<String, Vec<(String, String)>>> {
    static R: std::sync::OnceLock<Mutex<HashMap<String, Vec<(String, String)>>>> =
        std::sync::OnceLock::new();
    R.get_or_init(|| Mutex::new(HashMap::new()))
}

/// share_id -> (post_id, user_id)
pub(crate) fn shares() -> &'static Mutex<HashMap<String, (String, String)>> {
    static S: std::sync::OnceLock<Mutex<HashMap<String, (String, String)>>> =
        std::sync::OnceLock::new();
    S.get_or_init(|| Mutex::new(HashMap::new()))
}

/// post_id -> Vec<share_id>
pub(crate) fn shares_by_post() -> &'static Mutex<HashMap<String, Vec<String>>> {
    static SB: std::sync::OnceLock<Mutex<HashMap<String, Vec<String>>>> =
        std::sync::OnceLock::new();
    SB.get_or_init(|| Mutex::new(HashMap::new()))
}

/// post_id -> Vec<comment_id>
pub(crate) fn comments_by_post() -> &'static Mutex<HashMap<String, Vec<String>>> {
    static CB: std::sync::OnceLock<Mutex<HashMap<String, Vec<String>>>> =
        std::sync::OnceLock::new();
    CB.get_or_init(|| Mutex::new(HashMap::new()))
}
