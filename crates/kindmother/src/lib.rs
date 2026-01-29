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

/// @id: kindmother_module_state
/// @role: infrastructure
/// @layer: core
/// @human: Module de gestion d'état. Gère l'identité et l'état des instances (DB Mère, DB Filles).
/// @do: expose_state_module
/// Module de gestion d'état
pub mod state;

/// @id: kindmother_module_storage
/// @role: infrastructure
/// @layer: core
/// @human: Module d'abstraction de la persistance. Masque SQLite et fournit une interface conceptuelle.
/// @do: expose_storage_module
/// Module d'abstraction de la persistance
pub mod storage;

/// @id: kindmother_module_sync
/// @role: infrastructure
/// @layer: core
/// @human: Module de synchronisation. Gère la propagation des changements entre instances.
/// @do: expose_sync_module
/// Module de synchronisation
pub mod sync;

/// @id: kindmother_module_api
/// @role: infrastructure
/// @layer: core
/// @human: Module API CoreData. Expose les 10 opérations de la CoreDataAPI.
/// @do: expose_api_module
/// Module API CoreData
pub mod api;

/// @id: kindmother_module_threat
/// @role: infrastructure
/// @layer: core
/// @human: Module de détection de menaces. Détecte la corruption et les anomalies.
/// @do: expose_threat_module
/// Module de détection de menaces
pub mod threat;

/// @id: kindmother_module_observability
/// @role: infrastructure
/// @layer: core
/// @human: Module d'observabilité. Collecte les métriques et l'état de santé.
/// @do: expose_observability_module
/// Module d'observabilité
pub mod observability;

// Ré-exports des types principaux pour faciliter l'usage

/// @id: kindmother_reexport_state
/// @role: infrastructure
/// @layer: core
/// @human: Ré-export des types d'état pour faciliter l'usage.
/// @do: reexport_state_types
/// @depends: kindmother_module_state
pub use state::{InstanceIdentity, InstanceState, InstanceType};

/// @id: kindmother_reexport_storage
/// @role: infrastructure
/// @layer: core
/// @human: Ré-export des types de stockage pour faciliter l'usage.
/// @do: reexport_storage_types
/// @depends: kindmother_module_storage
pub use storage::{MemoryStorage, Storage, StorageError};

/// @id: kindmother_reexport_sync
/// @role: infrastructure
/// @layer: core
/// @human: Ré-export des types de synchronisation pour faciliter l'usage.
/// @do: reexport_sync_types
/// @depends: kindmother_module_sync
pub use sync::{DeltaOperation, Sync, SyncDelta, SyncError};

/// @id: kindmother_reexport_api
/// @role: infrastructure
/// @layer: core
/// @human: Ré-export des types d'API pour faciliter l'usage.
/// @do: reexport_api_types
/// @depends: kindmother_module_api
pub use api::{APIError, CoreDataAPI, WriteIntent, WriteOperation};

/// @id: kindmother_reexport_threat
/// @role: infrastructure
/// @layer: core
/// @human: Ré-export des types de détection de menaces pour faciliter l'usage.
/// @do: reexport_threat_types
/// @depends: kindmother_module_threat
pub use threat::{DefaultThreatDetector, ThreatDetector, ThreatLevel};

/// @id: kindmother_reexport_observability
/// @role: infrastructure
/// @layer: core
/// @human: Ré-export des types d'observabilité pour faciliter l'usage.
/// @do: reexport_observability_types
/// @depends: kindmother_module_observability
pub use observability::{
    DefaultObservability, HealthStatus, Metrics, Observability,
};
