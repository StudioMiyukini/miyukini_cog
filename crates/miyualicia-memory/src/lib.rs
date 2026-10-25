//! Alicia Memory — mémoire persistante pour l'assistante Alicia.
//!
//! Inspiré du système d'**Hermes Agent** (Nous Research) :
//! - **Faits** : énoncés vrais à ré-exposer dans le prompt (ex. « Jean préfère
//!   le métro »)
//! - **Préférences** : choix par défaut paramétrables (ex. `notify=desktop`)
//! - **Épisodes** : journal des interactions (intent + paramètres + résultat)
//! - **Personas** : profils contextuels (`work`, `family`, `voice`...)
//!
//! Stockage : **KindMother**, seul écrivain des données du COG (K-13) —
//! intentions typées, lectures typées, audit chaîné, SQLCipher, une clé par
//! base. Base et opérateur `maia-_alicia`, classés **`nationale`** (mémoire
//! commune MAIA / Alicia, jamais publiable ; voir [`kindmother`]).
//! KindMother injoignable ⇒ erreur explicite, jamais de repli sur un fichier
//! local. L'ancienne base SQLite est importée une fois (feature
//! `import-heritage`, commande `alicia-memoire-import`), puis effacée de
//! façon sûre après vérification (conservation seulement explicite, l'oubli
//! étant alors refusé : voir [`copie_heritee`]).
//!
//! Ce crate ne fait qu'exposer un store typé. L'orchestration (injection dans
//! le prompt, hot reload) se fait côté `miyualicia`.

#![doc(html_root_url = "https://docs.miyukini.com/miyualicia-memory")]
// C-10 : aucun unwrap / expect / panic! / todo! / unimplemented! / unreachable! hors tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

// @id service.alicia.memory.lib
// @role alicia_memory_facade
// @layer 7
// @do expose_memory_store
// @human Façade publique du crate miyualicia-memory.

pub mod copie_heritee;
mod entrepot;
pub mod errors;
#[cfg(feature = "import-heritage")]
pub mod heritage;
pub mod kindmother;
pub mod learning;
pub mod prompt;
mod recherche;
pub mod schema;
pub mod store;
pub mod types;
mod vif;

pub use copie_heritee::{copie_heritee_presente, AncienneBase};
pub use errors::MemoryError;
pub use kindmother::{
    ConfigKindMother, ADRESSE_KINDMOTHER_DEFAUT, BASE_KINDMOTHER, ENV_ADRESSE_KINDMOTHER,
    OPERATEUR_KINDMOTHER,
};
pub use learning::{detect_repetitions, propose_from_pattern, DetectorOptions, RepetitionPattern};
pub use prompt::{borne_jetons, build_persona_context, ContextOptions};
pub use store::MemoryStore;
pub use types::{
    Episode, EpisodeOutcome, Fact, FactTombstone, FactVersion, LearnedSkill, LearnedSkillStatus,
    PendingConfirmation, PendingExpiryReport, PendingStatus, Persona, Preference, PENDING_REASON_EXPIRED,
};
