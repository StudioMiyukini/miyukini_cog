//! Module Médias — SPM CMS
//!
//! Gestion générique des médias (assets) et de leurs métadonnées,
//! sans upload HTTP, sans rendu, sans permissions.
//!
//! Ce module expose uniquement des contrats fonctionnels. Il ne connaît pas la persistance,
//! les permissions, ni la logique métier spécifique. Le produit implémente les adaptateurs.
//!
//! # Exemple d'usage
//!
//! ```rust,no_run
//! use miyukini_spm_cms_media::{MediaManager, MediaInput};
//! use miyukini_kernel::Id;
//!
//! // Le produit implémente MediaManager
//! // struct MyAdapter { ... }
//! // impl MediaManager for MyAdapter { ... }
//!
//! // Créer un média
//! // let input = MediaInput::new("image/jpeg".to_string(), b"metadata".to_vec());
//! // let id = adapter.create_media(input)?;
//! ```

pub mod error;
pub mod media;

#[cfg(feature = "memory")]
pub mod memory;

// Ré-exports publics
pub use error::MediaError;
pub use media::{EntityId, Media, MediaId, MediaInput, MediaManager};

#[cfg(feature = "memory")]
pub use memory::MemoryMediaManager;
