//! @id     miyuwebway_participant_store
//! @do     cog_list
//! @role   donnee
//! @layer  core
//! @human  miyuwebway participant — module « store ».

//! Store en mémoire liste COG (MiyuWebwayParticipant). Persistance réelle = KindMother.

use std::collections::HashMap;
use std::sync::Mutex;

/// cog_id / entry -> payload
pub(crate) fn cog_list() -> &'static Mutex<HashMap<String, String>> {
    static C: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();
    C.get_or_init(|| Mutex::new(HashMap::new()))
}
