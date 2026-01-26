//! KindMother - Moteur interne de données du Miyukini Core System v2.4
//!
//! Ce module implémente le skeleton initial de KindMother.
//! À ce stade, toutes les opérations sont rejetées explicitement.

pub mod api;
pub mod core;
pub mod errors;
pub mod lifecycle;
pub mod observability;
pub mod runtime;
pub mod state;
pub mod sync;
pub mod threat;
mod storage; // Module interne, non exposé publiquement

// Ré-exports publics
pub use api::CoreDataAPI;
pub use core::{
    AuthorityContext, DomainContext, InstanceContext, InstanceType, KindMother, WriteIntent,
};
pub use errors::KMError;
pub use lifecycle::WriteIntentState;
pub use observability::{
    Event, EventCategory, EventContext, IntentJournal, ObservableEvent, Observability,
    QuarantineEntry, QuarantineLog, RejectionEntry, RejectionLog,
};
pub use state::KMState;
pub use sync::{Conflict, ConflictDetector, ConflictType, SyncIntent, SyncIntentState, SyncManager};
pub use threat::{DetectedThreat, ThreatDetector, ThreatDetectorConfig, ThreatSeverity, ThreatType};
