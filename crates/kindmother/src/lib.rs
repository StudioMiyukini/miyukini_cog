//! # KindMother
//!
//! Moteur de données et persistance du Miyukini Core System.
//!
//! KindMother est l'autorité absolue sur la persistance, la synchronisation, et la cohérence des données.
//! Il fournit une abstraction complète de la persistance avec support offline-first et synchronisation automatique.
//!
//! ## Principes fondamentaux
//!
//! - **Autorité exclusive** : Toute opération de persistance passe par KindMother
//! - **Offline-first** : Fonctionnement autonome sans connexion réseau
//! - **Abstraction complète** : SQLite interne, jamais exposé
//! - **Séparation décision/exécution** : KindMother exécute, StrongFather décide
//! - **Traçabilité complète** : Toutes les opérations sont tracées
//!
//! ## Modules
//!
//! - **state** : Gestion d'état des instances (DB Mère, DB Filles)
//! - **storage** : Abstraction de la persistance
//! - **sync** : Synchronisation entre instances
//! - **api** : API CoreData (10 opérations)
//! - **threat** : Détection de menaces et corruption
//! - **observability** : Observabilité et métriques
//!
//! ## Références
//!
//! - [Documentation Fondatrice](../../docs/core/KindMother/foundation/KindMother%20-%20Documentation%20Fondatrice.md)
//! - [CoreDataAPI Contract](../../docs/core/KindMother/contracts/api/KindMother%20-%20CoreDataAPI%20Contract.md)
//! - [Write Intent Lifecycle Contract](../../docs/core/KindMother/contracts/lifecycle/KindMother%20-%20Write%20Intent%20Lifecycle%20Contract.md)

/// @id kindmother_module_state
/// @role infrastructure
/// @layer core
/// @do expose_state_module
//@human / @human Module de gestion d'état. Gère l'identité et l'état des instances (DB Mère, DB Filles).
/// Module de gestion d'état
pub mod state;

/// @id kindmother_module_storage
/// @role infrastructure
/// @layer core
/// @do expose_storage_module
//@human / @human Module d'abstraction de la persistance. Masque SQLite et fournit une interface conceptuelle.
/// Module d'abstraction de la persistance
pub mod storage;

/// @id kindmother_module_sync
/// @role infrastructure
/// @layer core
/// @do expose_sync_module
//@human / @human Module de synchronisation. Gère la propagation des changements entre instances.
/// Module de synchronisation
pub mod sync;

/// @id kindmother_module_api
/// @role infrastructure
/// @layer core
/// @do expose_api_module
//@human / @human Module API CoreData. Expose les 10 opérations de la CoreDataAPI.
/// Module API CoreData
pub mod api;

/// @id kindmother_module_intent
/// @role infrastructure
/// @layer core
/// @do expose_intent_module
//@human / @human Intentions d'écriture typées, lectures typées, migrations et cycle de vie (lot C, K-13).
/// Module des intentions typées
pub mod intent;

/// @id kindmother_module_threat
/// @role infrastructure
/// @layer core
/// @do expose_threat_module
//@human / @human Module de détection de menaces. Détecte la corruption et les anomalies.
/// Module de détection de menaces
pub mod threat;

/// @id kindmother_module_observability
/// @role infrastructure
/// @layer core
/// @do expose_observability_module
//@human / @human Module d'observabilité. Collecte les métriques et l'état de santé.
/// Module d'observabilité
pub mod observability;

/// @id kindmother_module_miyusql_bridge
/// @role infrastructure
/// @layer core
/// @do expose_miyusql_bridge
//@human / @human Pont KindMother ↔ MiyuSQL. Délégation de l'exécution SQL sous mandat.
/// Module pont MiyuSQL
pub mod miyusql_bridge;

// Ré-exports des types principaux pour faciliter l'usage

/// @id kindmother_reexport_state
/// @role infrastructure
/// @layer core
/// @do reexport_state_types
//@human / @human Ré-export des types d'état pour faciliter l'usage.
/// @depends kindmother_module_state
pub use state::{InstanceIdentity, InstanceState, InstanceType};

/// @id kindmother_reexport_storage
/// @role infrastructure
/// @layer core
/// @do reexport_storage_types
//@human / @human Ré-export des types de stockage pour faciliter l'usage.
/// @depends kindmother_module_storage
pub use storage::{MemoryStorage, Storage, StorageError};

/// @id kindmother_reexport_sync
/// @role infrastructure
/// @layer core
/// @do reexport_sync_types
//@human / @human Ré-export des types de synchronisation pour faciliter l'usage.
/// @depends kindmother_module_sync
pub use sync::{DeltaOperation, Sync, SyncDelta, SyncError};

/// @id kindmother_reexport_api
/// @role infrastructure
/// @layer core
/// @do reexport_api_types
//@human / @human Ré-export des types d'API pour faciliter l'usage.
/// @depends kindmother_module_api
pub use api::{APIError, CoreDataAPI, WriteIntent, WriteOperation};

/// @id kindmother_reexport_intent
/// @role infrastructure
/// @layer core
/// @do reexport_intent_types
//@human / @human Ré-export des intentions typées (lot C, K-13, EX-07).
/// @depends kindmother_module_intent
pub use intent::{
    IntentLifecycle, IntentOperation, IntentOutcome, IntentState, LifecycleError, OrderBy,
    SchemaMigration, SqlValue, TypedSelect, TypedWriteIntent,
};

/// @id kindmother_reexport_threat
/// @role infrastructure
/// @layer core
/// @do reexport_threat_types
//@human / @human Ré-export des types de détection de menaces pour faciliter l'usage.
/// @depends kindmother_module_threat
pub use threat::{DefaultThreatDetector, ThreatDetector, ThreatLevel};

pub use miyusql_bridge::{MiyuSQLBridgeImpl, MiyuSQLExecutionBridge};
/// @id kindmother_reexport_observability
/// @role infrastructure
/// @layer core
/// @do reexport_observability_types
//@human / @human Ré-export des types d'observabilité pour faciliter l'usage.
/// @depends kindmother_module_observability
pub use observability::{DefaultObservability, HealthStatus, Metrics, Observability};
