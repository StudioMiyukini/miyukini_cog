//! Alicia Memory — mémoire persistante pour l'assistante Alicia.
//!
//! Inspiré du système d'**Hermes Agent** (Nous Research) :
//! - **Faits** : énoncés vrais à ré-exposer dans le prompt (ex. « Jean préfère
//!   le métro »)
//! - **Préférences** : choix par défaut paramétrables (ex. `notify=desktop`)
//! - **Épisodes** : journal des interactions (intent + paramètres + résultat)
//! - **Personas** : profils contextuels (`work`, `family`, `voice`...)
//!
//! Backend : **SQLite local** (rusqlite + bundled) avec **FTS5** pour les
//! recherches plein texte sur les faits et épisodes.
//!
//! Ce crate ne fait qu'exposer un store typé. L'orchestration (injection dans
//! le prompt, hot reload) se fera côté `miyualicia` (P2+).

#![doc(html_root_url = "https://docs.miyukini.com/miyualicia-memory")]

// @id service.alicia.memory.lib
// @role alicia_memory_facade
// @layer 7
// @do expose_memory_store
// @human Façade publique du crate miyualicia-memory.

pub mod errors;
pub mod learning;
pub mod prompt;
pub mod store;
pub mod types;

pub use errors::MemoryError;
pub use learning::{detect_repetitions, propose_from_pattern, DetectorOptions, RepetitionPattern};
pub use prompt::{build_persona_context, ContextOptions};
pub use store::MemoryStore;
pub use types::{
    Episode, EpisodeOutcome, Fact, LearnedSkill, LearnedSkillStatus, PendingConfirmation,
    PendingExpiryReport, PendingStatus, Persona, Preference, PENDING_REASON_EXPIRED,
};
