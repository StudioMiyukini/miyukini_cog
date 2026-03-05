//! # Miyukini Kernel
//!
//! Fondation technique minimale de l'écosystème Miyukini.
//!
//! Le Kernel fournit les primitives techniques transversales nécessaires à tous les produits :
//! configuration, identifiants, temps, logging, et gestion du cycle de vie.
//!
//! ## Modules
//!
//! - **config** : Chargement de configuration (env, fichiers, secrets)
//! - **id** : Génération d'identifiants uniques (UUID/ULID)
//! - **time** : Abstraction temps (now, timezone, tests)
//! - **log** : Logging structuré (niveaux, sortie)
//! - **lifecycle** : Boot / shutdown : ordre d'init, hooks d'arrêt
//!
//! ## Principes fondamentaux
//!
//! - **Aucune logique métier** : Le Kernel ne connaît ni les entités domaine, ni les règles de gestion
//! - **Aucune dépendance externe critique** : Fonctionne sans réseau
//! - **Primitives locales sûres uniquement** : Pas d'effets de bord cachés
//! - **Pas de protocole applicatif** : HTTP, WebSocket restent du ressort des produits
//!
//! ## Références
//!
//! - [Definition Kernel](../../docs/kernel/Miyukini%20Core%20System%20-%20Definition%20Kernel.md)
//! - [Structure du Kernel](../../docs/kernel/Miyukini%20Core%20System%20-%20Structure%20du%20Kernel.md)
//! - [Invariants & Guarantees](../../docs/kernel/contracts/Kernel%20-%20Invariants%20&%20Guarantees.md)

/// @id: kernel_module_config
/// @role: infrastructure
/// @layer: kernel
/// @human: Module de configuration. Fournit le mécanisme d'accès à la configuration, pas la politique.
/// @do: expose_config_module
/// Module de configuration
pub mod config;

/// @id: kernel_module_id
/// @role: infrastructure
/// @layer: kernel
/// @human: Module d'identifiants. Fournit la génération d'identifiants uniques avec type opaque.
/// @do: expose_id_module
/// Module d'identifiants
pub mod id;

/// @id: kernel_module_time
/// @role: infrastructure
/// @layer: kernel
/// @human: Module d'abstraction du temps. Permet l'injection en test et évite les appels système dispersés.
/// @do: expose_time_module
/// Module d'abstraction du temps
pub mod time;

/// @id: kernel_module_log
/// @role: infrastructure
/// @layer: kernel
/// @human: Module de logging structuré. Le produit choisit les backends et les formats.
/// @do: expose_log_module
/// Module de logging
pub mod log;

/// @id: kernel_module_lifecycle
/// @role: infrastructure
/// @layer: kernel
/// @human: Module de gestion du cycle de vie. Shutdown hooks uniquement, pas d'orchestration.
/// @do: expose_lifecycle_module
/// Module de gestion du cycle de vie
pub mod lifecycle;

// Ré-exports des types principaux pour faciliter l'usage

/// @id: kernel_reexport_config
/// @role: infrastructure
/// @layer: kernel
/// @human: Ré-export du trait Config et de EnvConfig pour faciliter l'usage.
/// @do: reexport_config_types
/// @depends: kernel_module_config
pub use config::{Config, EnvConfig};

/// @id: kernel_reexport_id
/// @role: infrastructure
/// @layer: kernel
/// @human: Ré-export des types Id, IdGenerator, UuidIdGenerator et IdParseError pour faciliter l'usage.
/// @do: reexport_id_types
/// @depends: kernel_module_id
pub use id::{Id, IdGenerator, IdParseError, UuidIdGenerator};

/// @id: kernel_reexport_time
/// @role: infrastructure
/// @layer: kernel
/// @human: Ré-export du trait Clock et de DefaultClock pour faciliter l'usage.
/// @do: reexport_time_types
/// @depends: kernel_module_time
pub use time::{Clock, DefaultClock};

/// @id: kernel_reexport_log
/// @role: infrastructure
/// @layer: kernel
/// @human: Ré-export du trait Logger, de l'enum Level et de DefaultLogger pour faciliter l'usage.
/// @do: reexport_log_types
/// @depends: kernel_module_log
pub use log::{DefaultLogger, Level, Logger};

/// @id: kernel_reexport_lifecycle
/// @role: infrastructure
/// @layer: kernel
/// @human: Ré-export du trait Lifecycle et de DefaultLifecycle pour faciliter l'usage.
/// @do: reexport_lifecycle_types
/// @depends: kernel_module_lifecycle
pub use lifecycle::{DefaultLifecycle, Lifecycle};
